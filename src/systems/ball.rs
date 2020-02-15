use std::time::Instant;

use specs::prelude::*;

use crate::components::{Ball, Hitbox, Paddle, Transform};
use crate::types::OverlapType;

pub struct BallSystem {
    last_called: Option<Instant>,
}

impl Default for BallSystem {
    fn default() -> Self {
        BallSystem { last_called: None }
    }
}

impl<'a> System<'a> for BallSystem {
    type SystemData = (
        WriteStorage<'a, Ball>,
        ReadStorage<'a, Paddle>,
        WriteStorage<'a, Transform>,
        ReadStorage<'a, Hitbox>,
        Entities<'a>,
    );
    fn run(&mut self, (mut balls, paddles, mut transforms, hitboxes, entities): Self::SystemData) {
        if self.last_called.is_some() {
            let mut ball_info: Vec<(Entity, bool, bool)> = vec![];

            // Move the ball(s)
            for (t, b, _, e) in (&mut transforms, &balls, &hitboxes, &entities).join() {
                let delta_t: f32 =
                    (Instant::now() - self.last_called.unwrap()).as_millis() as f32 / 1000.0;
                let dv = delta_t * b.velocity;

                t.set_x(t.x() + dv.x);
                t.set_y(t.y() + dv.y);

                ball_info.push((e, false, false));
            }

            // Check if they bounced off a paddle
            for (_, t, hb) in (&paddles, &transforms, &hitboxes).join() {
                for (e, bounce_horiz, bounce_vert) in ball_info.iter_mut() {
                    let ball_hb = hitboxes.get(*e).unwrap();
                    let ball_transform = transforms.get(*e).unwrap();
                    match ball_hb.intersects(hb, ball_transform, t) {
                        OverlapType::None => {}
                        OverlapType::OnlyX => {
                            *bounce_horiz = true;
                        }
                        OverlapType::OnlyY => {
                            *bounce_vert = true;
                        }
                        OverlapType::Both => {
                            println!("ding");
                            *bounce_horiz = true;
                            *bounce_vert = true;
                        }
                    }
                }
            }

            for (e, bounce_horiz, bounce_vert) in ball_info.iter() {
                //if *bounce_horiz {
                //    let b = balls.get_mut(*e).unwrap();
                //    b.velocity.x = -b.velocity.x;
                //}
                if *bounce_vert {
                    let b = balls.get_mut(*e).unwrap();
                    b.velocity.y = -b.velocity.y;
                }
            }
        }
        self.last_called = Some(Instant::now());
    }
}
