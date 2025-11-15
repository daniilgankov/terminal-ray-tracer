use crate::{Color, escape::Escape};

#[derive(Clone, Default)]
pub(crate) struct Symbol {
    pub(crate) color: Color,
    pub(crate) text: Option<char>,
}

impl Symbol {
    pub(crate) fn with_color(color: Color) -> Self {
        Self {
            color,
            ..Default::default()
        }
    }

    pub(crate) fn encode(&self) -> String {
        let text = self.text.unwrap_or(' ');
        format!(
            "{}{}{}",
            Escape::SetBackgroundColor(self.color),
            Escape::SetForegroundColor(self.color.inverted()),
            text
        )
    }
}
