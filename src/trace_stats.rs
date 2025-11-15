use std::{
    fmt::{Display, Formatter, Result},
    ops::AddAssign,
};

#[derive(Default)]
pub(crate) struct TraceStats {
    pub(crate) traced: usize,
    pub(crate) reflected: usize,
    pub(crate) hit: usize,
    pub(crate) shadow_traced: usize,
    pub(crate) shadow_hit: usize,
}

impl AddAssign for TraceStats {
    fn add_assign(&mut self, rhs: Self) {
        self.traced += rhs.traced;
        self.reflected += rhs.reflected;
        self.hit += rhs.hit;
        self.shadow_traced += rhs.shadow_traced;
        self.shadow_hit += rhs.shadow_hit;
    }
}

impl Display for TraceStats {
    fn fmt(&self, f: &mut Formatter<'_>) -> Result {
        f.write_fmt(format_args!(
            "{} rays ({} reflected, {} hit), {} shadow rays ({} hit)",
            self.traced, self.reflected, self.hit, self.shadow_traced, self.shadow_hit
        ))
    }
}
