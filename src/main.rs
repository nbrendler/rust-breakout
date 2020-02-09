#![warn(clippy::pedantic)]
#![deny(clippy::all)]
#![allow(clippy::cast_precision_loss)]

use specs::prelude::{World, WorldExt};

use breakout_clone::{start_app, GameError};

fn main() -> Result<(), GameError> {
    let mut world = World::new();

    start_app(&mut world)?;

    Ok(())
}
