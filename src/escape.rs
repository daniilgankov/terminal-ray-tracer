use std::fmt::{Display, Formatter, Result};

use crate::Color;

pub(crate) enum Escape {
    MakeCursorInvisible,
    MakeCursorVisible,
    MoveCursorToStart,
    SetBackgroundColor(Color),
    SetForegroundColor(Color),
}

// https://gist.github.com/fnky/458719343aabd01cfb17a3a4f7296797
impl Escape {
    pub(crate) fn encode(&self) -> String {
        let sequence = match self {
            Escape::MakeCursorInvisible => "?25l".to_string(),
            Escape::MakeCursorVisible => "?25h".to_string(),
            Escape::MoveCursorToStart => "H".to_string(),
            Escape::SetBackgroundColor(color) => format!("48{}", color.encode()),
            Escape::SetForegroundColor(color) => format!("38{}", color.encode()),
        };
        format!("{ESCAPE}[{sequence}")
    }
}

const ESCAPE: &str = "\x1b";

impl Display for Escape {
    fn fmt(&self, f: &mut Formatter<'_>) -> Result {
        f.write_str(&self.encode())
    }
}
