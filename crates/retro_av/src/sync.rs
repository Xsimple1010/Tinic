use generics::error_handle::ErrorHandle;
use generics::types::{ArcTMutex, TMutex};
use retro_core::av_info::AvInfo;
use std::sync::Arc;
use std::thread::sleep;
use std::time::{Duration, Instant};

pub struct RetroSync {
    last_frame_time: Instant,
    rate_control_delta: f64,
    frame_count: u32,
    pub sync_data: ArcTMutex<SyncData>,
}

pub struct SyncData {
    pub target_frame_duration: Duration,
    pub elapsed: Duration,
    pub fps: f64,
    pub now: Instant,
    pub adjustment: f64,
}

impl RetroSync {
    pub fn new(rate_control_delta: f64) -> Self {
        Self {
            last_frame_time: Instant::now(),
            frame_count: 0,
            rate_control_delta,
            sync_data: TMutex::new(SyncData {
                elapsed: Duration::from_secs_f64(0.0),
                target_frame_duration: Duration::from_secs_f64(0.0),
                fps: 0.0,
                now: Instant::now(),
                adjustment: 0.0,
            }),
        }
    }

    pub fn prepare_sync_data(&mut self, av: &Arc<AvInfo>) -> Result<(), ErrorHandle> {
        let fps = *av.timing.fps.read()?;
        let target_frame_duration = Duration::from_secs_f64(1.0 / fps);
        let now = Instant::now();
        let elapsed = now.duration_since(self.last_frame_time);

        let delta = elapsed.as_secs_f64() - target_frame_duration.as_secs_f64();
        let adjustment = if delta.abs() > self.rate_control_delta {
            delta.clamp(-self.rate_control_delta, self.rate_control_delta)
        } else {
            delta
        };

        self.sync_data.store(SyncData {
            elapsed,
            target_frame_duration,
            fps,
            now,
            adjustment,
        });

        Ok(())
    }

    pub fn sync_now(&mut self) -> Result<(), ErrorHandle> {
        let sync_data = self.sync_data.try_load()?;

        let sleep_time = if sync_data.adjustment < 0.0 {
            sync_data.target_frame_duration - sync_data.elapsed
        } else {
            Duration::from_secs_f64((1.0 / sync_data.fps) - sync_data.adjustment)
        };

        self.last_frame_time = sync_data.now;
        self.frame_count += 1;

        if sleep_time > Duration::ZERO {
            // println!("frame count: {}", self.frame_count);
            sleep(sleep_time);
        }

        Ok(())
    }
}
