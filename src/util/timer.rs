use std::time::{Duration, Instant};

pub(crate) struct Timer {
    tick: Instant,
}

impl Timer {
    pub(crate) fn new() -> Self {
        let now = Instant::now();
        Self { tick: now }
    }

    pub(crate) fn tick(&mut self) -> Duration {
        let now = Instant::now();
        let elapsed = now - self.tick;
        self.tick = now;
        elapsed
    }
}
