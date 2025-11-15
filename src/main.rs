use camera::Camera;
use color::Color;
use input::Input;
use math::vec2::vec2;
use math::vec3::{Vec3f, vec3};
use object::Object;
use scene::Scene;
use screen::Screen;
use util::average_sum::AverageSum;
use util::timer::Timer;
use view_mode::ViewMode;

use crate::geometry::aabb::Aabb;
use crate::geometry::sphere::Sphere;
use crate::geometry::triangle::Triangle;
use crate::geometry::triangular::Triangular;

mod camera;
mod color;
mod consts;
mod escape;
mod geometry;
mod input;
mod math;
mod object;
mod palette;
mod scene;
mod screen;
mod sky;
mod symbol;
mod trace_payload;
mod trace_stats;
mod util;
mod view_mode;

fn main() {
    let scene = build_scene();
    let mut screen = Screen::new();
    let mut timer = Timer::new();
    const AVG_WINDOW: usize = 100;
    let mut avg_fps = AverageSum::new(AVG_WINDOW);
    let mut avg_ms = AverageSum::new(AVG_WINDOW);
    let mut view_mode = ViewMode::default();
    const ACCELERATION: f32 = 10.0;
    let mut velocity = vec2!(0.0);
    let mut position = vec2!(0.0f32);
    let mut input = Input::new();
    loop {
        let time_delta = timer.tick().as_secs_f32();

        if let Some(char) = input.pop() {
            match char {
                '1' => view_mode = ViewMode::Color,
                '2' => view_mode = ViewMode::Normal,
                '3' => view_mode = ViewMode::Depth,
                '4' => view_mode = ViewMode::Complexity,
                '\u{1b}' | 'q' => {
                    break;
                }
                'h' => velocity.x -= ACCELERATION * time_delta,
                'j' => velocity.y -= ACCELERATION * time_delta,
                'k' => velocity.y += ACCELERATION * time_delta,
                'l' => velocity.x += ACCELERATION * time_delta,
                _ => (),
            }
        }

        let next_position = position + velocity * time_delta;
        position.x = next_position.x;
        if next_position.y.abs() < 2.0 {
            position.y = next_position.y;
        } else {
            velocity.y = 0.0;
        }
        velocity -= velocity * time_delta;
        const LOOK_AT: Vec3f = vec3!(0.0);
        let offset = vec3!(2.0 * position.x.sin(), position.y, 2.0 * position.x.cos());
        let camera = Camera {
            look_from: LOOK_AT + offset,
            look_at: LOOK_AT,
        };

        screen.append_overlay_text_line("Terminal Ray Tracer".to_owned());
        screen.append_overlay_text_line(format!("View mode: {view_mode} (use 1-4 keys to change)"));
        let fps = 1.0 / time_delta;
        avg_fps.add(fps);
        let ms = 1e3 * time_delta;
        avg_ms.add(ms);
        screen.append_overlay_text_line(format!(
            "{ms:.0} ~{avg_ms:.0} ms, {fps:.1} ~{avg_fps:.1} fps"
        ));
        screen.render(&scene, &camera, &view_mode);
        screen.draw();
    }
}

fn build_scene() -> Scene {
    let mut scene = Scene::new();
    scene.spawn(Object {
        color: Color::RED,
        intersect: Box::new(Sphere {
            center: vec3!(-1.0, -1.0, 0.0),
            radius: 0.5,
        }),
    });
    let triangular = load_suzanne_obj();
    scene.spawn(Object {
        color: Color::GREEN,
        intersect: Box::new(triangular),
    });
    scene.spawn(Object {
        color: Color::BLUE,
        intersect: Box::new(Aabb::centered(vec3!(1.0, -1.0, 0.0), 1.0 / 3.0)),
    });
    scene
}

fn load_suzanne_obj() -> Triangular {
    let text = include_str!("suzanne.obj");
    let mut vertices = vec![];
    let mut index_triplets = vec![];
    for line in text.lines() {
        let (operator, values) = line.split_once(char::is_whitespace).unwrap();
        match operator {
            "v" => {
                let values: Vec<f32> = values.split_whitespace().flat_map(str::parse).collect();
                let [x, y, z] = values.try_into().unwrap();
                let vertex = vec3!(x, y, z);
                vertices.push(vertex);
            }
            "f" => {
                let values: Vec<usize> = values.split_whitespace().flat_map(str::parse).collect();
                let triplet: [_; 3] = values.try_into().unwrap();
                index_triplets.push(triplet);
            }
            _ => panic!("unsupported OBJ operator {operator}"),
        }
    }
    let mut builder = Triangular::builder();
    for triplet in index_triplets {
        let [i, j, k] = triplet;
        let a = vertices[i - 1];
        let b = vertices[j - 1];
        let c = vertices[k - 1];
        let triangle = Triangle::new(a, b, c);
        builder = builder.add_triangle(triangle);
    }
    builder.build()
}
