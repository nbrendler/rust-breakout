use specs::prelude::*;

use crate::components::{Ball, GlobalTransform, Paddle, Transform};
use crate::constants::WORLD_HEIGHT;
use crate::game_error::GameError;
use crate::types::ScreenContext;

pub fn init(world: &mut World) -> Result<(), GameError> {
    // TODO: move to transform system
    let (w, h) = {
        let screen_context = world.fetch::<ScreenContext>();
        screen_context.dimensions()
    };
    {
        let pixels_per_unit = h as f32 / WORLD_HEIGHT;

        let global_t = GlobalTransform(
            Transform::default()
                .with_pos(((w - h) as f32 / 2.0, 0.))
                .with_scale((pixels_per_unit, pixels_per_unit)),
        );

        world.insert(global_t);
    }
    Paddle::init(world);
    Ball::init(world);
    Ok(())
}
