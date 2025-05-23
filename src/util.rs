use std::time::Duration;

pub struct Timer {
    duration: Duration,
    elapsed: Duration,
}

impl Timer {
    pub fn new(duration: Duration) -> Timer {
        Timer {
            duration: duration,
            elapsed: Duration::from_secs(0),
        }
    }

    pub fn update(&mut self, delta_time: f32) {
        self.elapsed += Duration::from_secs_f32(delta_time);
    }

    pub fn time_left(&mut self) -> i32 {
        self.duration.abs_diff(self.elapsed).as_secs_f32().round() as i32
    }

    pub fn is_finished(&self) -> bool {
        self.elapsed >= self.duration
    }

    pub fn reset(&mut self) {
        self.elapsed = Duration::from_secs(0);
    }
}
