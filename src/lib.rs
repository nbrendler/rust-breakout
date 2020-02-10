use specs::prelude::{DispatcherBuilder, System as _, World, WorldExt as _};
use specs::shrev::EventChannel;

mod asset_manager;
mod breakout;
mod components;
mod game_error;
mod systems;
mod types;
mod util;

use crate::asset_manager::AssetManager;
use crate::components::{Sprite, Transform};
pub use crate::game_error::GameError;
use crate::systems::{FrameLimiterSystem, RenderingSystem};
pub use crate::types::{GameEvent, WindowState};

pub fn start_app(world: &mut World) -> Result<(), GameError> {
    {
        world.register::<Sprite>();
        world.register::<Transform>();
        world.insert::<AssetManager>(AssetManager::new());
    };

    breakout::init(world)?;

    world.insert(WindowState::default());

    let mut reader = {
        let mut ch = EventChannel::<GameEvent>::new();

        let reader = ch.register_reader();
        world.insert(ch);
        reader
    };

    let mut renderer = {
        let (width, height) = (800, 600);
        RenderingSystem::new(width, height)
    };

    renderer.setup(world);

    let mut dispatcher = DispatcherBuilder::new()
        .with_barrier()
        .with(FrameLimiterSystem::new(60), "fps_limiter", &[])
        .with_thread_local(renderer)
        .build();

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
