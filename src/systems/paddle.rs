use std::time::Instant;

use specs::prelude::*;

use crate::components::{Paddle, Transform};
use crate::constants::{PADDLE_SPEED, WORLD_WIDTH};
use crate::resources::InputState;

pub struct PaddleSystem {
    last_called: Option<Instant>,
}

impl Default for PaddleSystem {
    fn default() -> Self {
        PaddleSystem { last_called: None }
    }
}

impl<'a> System<'a> for PaddleSystem {
    type SystemData = (
        ReadStorage<'a, Paddle>,
        WriteStorage<'a, Transform>,
        Read<'a, InputState>,
    );
    fn run(&mut self, (paddles, mut transforms, input): Self::SystemData) {
        if self.last_called.is_some() {
            for (t, _) in (&mut transforms, &paddles).join() {
                let delta_t: f32 =
                    (Instant::now() - self.last_called.unwrap()).as_millis() as f32 / 1000.0;
                // TODO: set the bounds based on the PPU
                if input.left && !input.right {
                    t.move_left(PADDLE_SPEED * delta_t);
                    t.set_x(t.position.x.max(0.));
                } else if !input.left && input.right {
                    t.move_right(PADDLE_SPEED * delta_t);
                    t.set_x(t.position.x.min(WORLD_WIDTH));
                }
            }
        }

        self.last_called = Some(Instant::now());
    }
}
