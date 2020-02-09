use std::thread::sleep;
use std::time::{Duration, Instant};

use specs::prelude::System;

#[derive(Debug)]
pub struct FrameLimiterSystem {
    frame_duration: Duration,
    checkpoint: Instant,
    frames_since_last_checkpoint: u32,
    last_called: Instant,
    frame_count: u32,
}

impl FrameLimiterSystem {
    pub fn new(desired_fps: u32) -> Self {
        FrameLimiterSystem {
            checkpoint: Instant::now(),
            frame_duration: Duration::from_secs(1) / desired_fps,
            frame_count: 0,
            frames_since_last_checkpoint: 0,
            last_called: Instant::now(),
        }
    }
}

impl<'a> System<'a> for FrameLimiterSystem {
    type SystemData = ();
    fn run(&mut self, _: Self::SystemData) {
        self.frame_count += 1;
        self.frames_since_last_checkpoint += 1;

        let elapsed = Instant::now() - self.last_called;

        if elapsed <= self.frame_duration {
            sleep(self.frame_duration - elapsed);
        }

        self.last_called = Instant::now();
        if self.frame_count % 300 == 0 {
            println!(
                "Frame: {}, FPS: {:.2}",
                self.frame_count,
                self.frames_since_last_checkpoint as f32
                    / (self.last_called - self.checkpoint).as_secs() as f32
            );

            self.checkpoint = self.last_called;
            self.frames_since_last_checkpoint = 0;
        }
    }
}
