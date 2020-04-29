use std::time::{Duration, Instant};

#[derive(Copy, Clone)]
pub struct Timer {
    start: Instant,
    end: Option<Instant>,
}

impl Timer {
    pub fn start() -> Self {
        Timer {
            start: Instant::now(),
            end: None,
        }
    }

    pub fn stop(&mut self) {
        let t = Instant::now();
        if self.end.is_none() {
            self.end = Some(t);
        }
    }

    pub fn time(&self) -> Duration {
        match self.end {
            Some(e) => e.duration_since(self.start),
            None => self.start.elapsed(),
        }
    }
}
