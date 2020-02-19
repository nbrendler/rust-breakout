use std::time::Instant;

use cgmath::Vector2;
use specs::prelude::*;

use crate::collidable::Collidable;
use crate::components::{Ball, Paddle, Sprite, Transform};
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
        ReadStorage<'a, Sprite>,
        Entities<'a>,
    );
    fn run(&mut self, (mut balls, paddles, mut transforms, sprites, entities): Self::SystemData) {
        if self.last_called.is_some() {
            let mut ball_info: Vec<(Entity, bool, bool)> = vec![];

            // Move the ball(s)
            for (t, mut b, hb, e) in (&mut transforms, &mut balls, &sprites, &entities).join() {
                let delta_t: f32 =
                    (Instant::now() - self.last_called.unwrap()).as_millis() as f32 / 1000.0;
                let dv = delta_t * b.velocity;

                t.set_x(t.position.x + dv.x);
                t.set_y(t.position.y + dv.y);

                let ((x0, y0), (x1, y1)) = hb.corners(&t);
                let center = t.as_screen_point();
                let lower_bounds = t.clone().with_pos((0., 0.)).as_screen_point();
                let upper_bounds = t
                    .clone()
                    .with_pos((WORLD_WIDTH, WORLD_HEIGHT))
                    .as_screen_point();

                let mut bounce_horiz = false;
                let mut bounce_vert = false;

                let half_width = (x1 - x0) as f32 / 2.0;
                let half_height = (y1 - y0) as f32 / 2.0;

                if (center.x - half_width) < lower_bounds.x
                    || (center.x + half_width) > upper_bounds.x
                {
                    bounce_horiz = true;
                }

                if center.y + half_height > upper_bounds.y {
                    bounce_vert = true;
                }

                if (center.y - half_height) < lower_bounds.y {
                    println!("loser");
                    b.velocity = Vector2::new(0., 0.);
                }

                ball_info.push((e, bounce_horiz, bounce_vert));
            }

            // Check if it bounced off a paddle
            for (_, t, s) in (&paddles, &transforms, &sprites).join() {
                for (e, bounce_horiz, bounce_vert) in ball_info.iter_mut() {
                    let ball_sprite = sprites.get(*e).unwrap();
                    let ball_transform = transforms.get(*e).unwrap();
                    match ball_sprite.intersects(s, &ball_transform, &t) {
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
