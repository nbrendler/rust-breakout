use specs::prelude::*;

use crate::components::{Ball, Paddle};
use crate::game_error::GameError;

pub fn init(world: &mut World) -> Result<(), GameError> {
    Paddle::init(world);
    Ball::init(world);
    Ok(())
}
