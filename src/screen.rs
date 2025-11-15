use std::{num::NonZero, sync::mpsc};

use libc::{STDOUT_FILENO, TIOCGWINSZ, ioctl, winsize};

use crate::{
    ViewMode,
    camera::Camera,
    escape::Escape,
    math::vec2::{Vec2f, Vec2u, vec2},
    scene::Scene,
    symbol::Symbol,
    trace_payload::TracePayload,
    trace_stats::TraceStats,
};

pub(crate) struct Screen {
    size_in_symbols: Vec2u,
    size_in_pixels: Vec2u,
    symbols: Vec<Symbol>,
    overlay_text_lines: Vec<String>,
}

impl Screen {
    pub(crate) fn new() -> Self {
        let mut winsize = winsize {
            ws_row: 0,
            ws_col: 0,
            ws_xpixel: 0,
            ws_ypixel: 0,
        };
        assert_eq!(unsafe { ioctl(STDOUT_FILENO, TIOCGWINSZ, &mut winsize) }, 0);
        let size_in_symbols = vec2!(winsize.ws_col as usize, winsize.ws_row as usize);
        let size_in_pixels = if winsize.ws_xpixel != 0 && winsize.ws_ypixel != 0 {
            vec2!(winsize.ws_xpixel as usize, winsize.ws_ypixel as usize)
        } else {
            // TODO: Search for other ways to know the font size
            //
            // For now, let's fall back to VGA 437 8x16 font
            // https://en.wikipedia.org/wiki/Code_page_437
            // https://wiki.archlinux.org/title/Linux_console#Fonts
            let font_size = vec2!(8, 16);
            font_size.hadamard(size_in_symbols)
        };
        let colors = vec![Symbol::default(); size_in_symbols.area()];
        Self {
            size_in_symbols,
            size_in_pixels,
            symbols: colors,
            overlay_text_lines: vec![],
        }
    }

    pub(crate) fn render(&mut self, scene: &Scene, camera: &Camera, view_mode: &ViewMode) {
        let size_in_symbols = self.size_in_symbols;
        self.overlay_text_lines.push(format!(
            "Symbol size: {}x{} ({} total)",
            size_in_symbols.x,
            size_in_symbols.y,
            size_in_symbols.area()
        ));
        let size_in_pixels = self.size_in_pixels;
        let thread_count = std::thread::available_parallelism()
            .map(NonZero::get)
            .unwrap_or(1);
        let symbol_count = size_in_symbols.area();
        let task_count = symbol_count.next_multiple_of(thread_count);
        let tasks_per_thread = task_count / thread_count;
        let (sender, receiver) = mpsc::channel();
        std::thread::scope(|scope| {
            for thread_index in 0..thread_count {
                let sender = sender.clone();
                scope.spawn(move || {
                    let start_index = thread_index * tasks_per_thread;
                    for task_index in start_index..start_index + tasks_per_thread {
                        if task_index >= symbol_count {
                            break;
                        }
                        let i = task_index % size_in_symbols.x;
                        let j = task_index / size_in_symbols.x;
                        let index = vec2!(i, j);
                        let position = Vec2f::from(index) / Vec2f::from(size_in_symbols);
                        let mut viewport_position = 2.0 * position - 1.0;
                        viewport_position.y = -viewport_position.y;
                        let aspect_ratio = size_in_pixels.x as f32 / size_in_pixels.y as f32;
                        viewport_position.x *= aspect_ratio;
                        let viewport_ray = camera.viewport_ray(viewport_position);
                        let payload = scene.trace(viewport_ray, view_mode);
                        sender.send((task_index, payload)).unwrap();
                    }
                });
            }
        });
        let mut overall_stats = TraceStats::default();
        for _ in 0..symbol_count {
            let (index, TracePayload { color, stats }) = receiver.recv().unwrap();
            self.symbols[index] = Symbol::with_color(color);
            overall_stats += stats;
        }
        self.overlay_text_lines.push(overall_stats.to_string());
    }

    pub(crate) fn append_overlay_text_line(&mut self, line: String) {
        self.overlay_text_lines.push(line);
    }

    pub(crate) fn draw(&mut self) {
        print!(
            "{}{}",
            Escape::MakeCursorInvisible,
            Escape::MoveCursorToStart
        );
        for (j, line) in self.overlay_text_lines.iter().enumerate() {
            for (i, text) in line.chars().enumerate() {
                let index = i + self.size_in_symbols.x * j;
                let _ = self.symbols[index].text.insert(text);
            }
        }
        self.overlay_text_lines.clear();
        for j in 0..self.size_in_symbols.y {
            for i in 0..self.size_in_symbols.x {
                let index = i + self.size_in_symbols.x * j;
                print!("{}", self.symbols[index].encode());
            }
        }
        print!("{}", Escape::MakeCursorVisible);
    }
}
