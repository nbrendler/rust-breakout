use specs::prelude::*;

use crate::components::Paddle;
use crate::game_error::GameError;

pub fn init(world: &mut World) -> Result<(), GameError> {
    Paddle::init(world);
    Ok(())
}
