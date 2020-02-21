use specs::prelude::*;

use crate::components::{Ball, Block, Paddle};
use crate::game_error::GameError;

pub fn init(world: &mut World) -> Result<(), GameError> {
    Paddle::init(world);
    Ball::init(world);
    Block::init(world);
    Ok(())
}
