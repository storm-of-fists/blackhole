use std::time::{Duration, Instant};

pub struct Clock {
    /// The instant we started.
    start_instant: Instant,
}

impl Clock {
    pub fn new() -> Self {
        Self {
            start_instant: Instant::now(),
        }
    }

    pub fn set_start_instant(&mut self, start_instant: Instant) {
        self.start_instant = start_instant;
    }

    pub fn start_instant(&self) -> &Instant {
        &self.start_instant
    }

    pub fn duration_since_start(&self) -> Duration {
        self.start_instant.elapsed()
    }
}

// pub struct ControlClock {
//     /// Application control cycle time.
//     control_time: Duration,
//     control_period: Duration,
//     clock: Clock,

// }

// impl ControlClock {
//     pub fn new(control_period: Duration) -> Self {
//         Self {
//             control_time: Duration::ZERO,
//             control_period,
//             clock: Clock::new(),
//         }
//     }
//     pub fn start_time(&self) -> &Duration {
//         &self.clock.start_time
//     }

//     pub fn start_instant(&self) -> &Instant {
//         &self.clock.start_instant
//     }

//     pub fn duration_since_start(&self) -> Duration {
//         self.clock.start_instant.elapsed()
//     }

//     pub fn control_time(&self) -> &Duration {
//         &self.control_time
//     }
//     pub fn increment_control_time(&mut self) {
//         self.control_time += self.control_period;
//     }
// }