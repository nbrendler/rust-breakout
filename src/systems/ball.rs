use std::time::Instant;

use cgmath::Vector2;
use specs::prelude::*;

use crate::components::{Ball, Hitbox, Paddle, Transform};
use crate::constants::{WORLD_HEIGHT, WORLD_WIDTH};
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
            for (t, mut b, hb, e) in (&mut transforms, &mut balls, &hitboxes, &entities).join() {
                let delta_t: f32 =
                    (Instant::now() - self.last_called.unwrap()).as_millis() as f32 / 1000.0;
                let dv = delta_t * b.velocity;

                t.set_x(t.x() + dv.x);
                t.set_y(t.y() + dv.y);

                let p = t.as_screen_point();
                let corners = hb.corners(p);

                println!("{:?} {:?}", p, corners);

                let mut bounce_horiz = false;
                let mut bounce_vert = false;

                if p.x - corners.0.x < 0.0 || p.x + corners.1.x > WORLD_WIDTH {
                    println!("bounce off side");
                    bounce_horiz = true;
                }

                if p.y + corners.1.y > WORLD_HEIGHT {
                    println!("bounce off top");
                    bounce_vert = true;
                }

                if p.y - corners.0.y < 0. {
                    println!("lose");
                    b.velocity = Vector2::new(0., 0.);
                }

                ball_info.push((e, bounce_horiz, bounce_vert));
            }

            // Check if it bounced off a paddle
            for (_, t, hb) in (&paddles, &transforms, &hitboxes).join() {
                for (e, bounce_horiz, bounce_vert) in ball_info.iter_mut() {
                    let ball_hb = hitboxes.get(*e).unwrap();
                    let ball_transform = transforms.get(*e).unwrap();
                    match ball_hb.intersects(
                        hb,
                        ball_transform.as_screen_point(),
                        t.as_screen_point(),
                    ) {
                        OverlapType::None => {}
                        OverlapType::OnlyX => {
                            *bounce_horiz = true;
                        }
                        OverlapType::OnlyY => {
                            *bounce_vert = true;
                        }
                        OverlapType::Both => {
                            *bounce_horiz = true;
                            *bounce_vert = true;
                        }
                    }
                }
            }

            // bounce if it hit the sides

            for (e, bounce_horiz, bounce_vert) in ball_info.iter() {
                if *bounce_horiz {
                    let b = balls.get_mut(*e).unwrap();
                    b.velocity.x = -b.velocity.x;
                }
                if *bounce_vert {
                    let b = balls.get_mut(*e).unwrap();
                    b.velocity.y = -b.velocity.y;
                }
            }
        }
        self.last_called = Some(Instant::now());
    }
}
