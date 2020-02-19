use specs::prelude::{DispatcherBuilder, World};
use specs::shrev::EventChannel;

mod asset_manager;
mod breakout;
mod collidable;
mod components;
mod constants;
mod game_error;
mod resources;
mod systems;
mod types;
mod util;

use crate::asset_manager::AssetManager;
pub use crate::game_error::GameError;
use crate::systems::{BallSystem, FrameLimiterSystem, InputSystem, PaddleSystem, RenderingSystem};
pub use crate::types::GameEvent;

pub fn start_app(world: &mut World) -> Result<(), GameError> {
    {
        world.insert::<AssetManager>(AssetManager::new());
    };

    let mut reader = {
        let mut ch = EventChannel::<GameEvent>::new();

        let reader = ch.register_reader();
        world.insert(ch);
        reader
    };

    let renderer = {
        let (width, height) = (800, 600);
        RenderingSystem::new(width, height)
    };

    let mut dispatcher = DispatcherBuilder::new()
        .with(InputSystem::default(), "input", &[])
        .with(PaddleSystem::default(), "paddle movement", &["input"])
        .with(BallSystem::default(), "ball movement", &[])
        .with_barrier()
        .with(FrameLimiterSystem::new(60), "fps_limiter", &[])
        .with_thread_local(renderer)
        .build();

    dispatcher.setup(world);

    breakout::init(world)?;

    'app: loop {
        dispatcher.dispatch(world);
        let ch = world.fetch::<EventChannel<GameEvent>>();
        for event in ch.read(&mut reader) {
            if let GameEvent::CloseWindow = event {
                break 'app;
            }
        }
    }

    Ok(())
}
