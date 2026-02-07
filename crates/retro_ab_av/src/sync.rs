use std::thread::sleep;
use std::time::{Duration, Instant};

#[derive(Default)]
pub struct RetroSync {
    last_time: Option<Instant>,
}

impl RetroSync {
    fn fps_to_millis(&self, fps: f64) -> f64 {
        (1f64 / fps) * 1000f64
    }

    pub fn sync(&mut self, fps: f64) -> bool {
        let now = Instant::now();

        if self.last_time.is_none() {
            self.last_time = Some(now);
            return true;
        }

        if let Some(last_time) = self.last_time {
            let time_lapse = now - last_time;
            let core_default_duration = Duration::from_millis(self.fps_to_millis(fps) as u64);

            if time_lapse >= core_default_duration {
                self.last_time = Some(now);

                return true;
            }
        }

        sleep(Duration::from_millis(1));

        false
    }
}
