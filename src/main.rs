#![warn(clippy::pedantic)]
#![deny(clippy::all)]
#![allow(clippy::cast_precision_loss)]

use luminance_glfw::{GlfwSurface, Surface, WindowDim, WindowOpt};
use specs::prelude::{World, WorldExt};

use breakout_clone::{start_app, GameError, WindowState};

fn main() -> Result<(), GameError> {
    let (width, height) = (800, 600);

    let surface = GlfwSurface::new(
        WindowDim::Windowed(width, height),
        "No Tilearino",
        WindowOpt::default(),
    )
    .expect("unable to create surface");

    let mut world = World::new();
    world.insert::<WindowState>(WindowState::new(width, height));

    start_app(surface, &mut world)?;

    Ok(())
}
