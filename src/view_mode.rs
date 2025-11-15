use std::fmt::{Display, Formatter, Result};

#[derive(Default)]
pub(crate) enum ViewMode {
    #[default]
    Color,
    Normal,
    Depth,
    Complexity,
}

impl ViewMode {
    fn name(&self) -> &'static str {
        match self {
            ViewMode::Color => "color",
            ViewMode::Normal => "normal",
            ViewMode::Depth => "depth",
            ViewMode::Complexity => "complexity",
        }
    }
}

impl Display for ViewMode {
    fn fmt(&self, f: &mut Formatter<'_>) -> Result {
        f.write_str(self.name())
    }
}
