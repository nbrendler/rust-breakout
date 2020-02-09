use specs::prelude::{Builder as _, DispatcherBuilder, System as _, World, WorldExt as _};

mod asset_manager;
mod components;
mod game_error;
mod systems;
mod types;
mod util;

use crate::asset_manager::AssetManager;
use crate::components::Sprite;
pub use crate::game_error::GameError;
use crate::systems::{FrameLimiterSystem, RenderingSystem};
pub use crate::types::WindowState;

pub fn start_app(world: &mut World) -> Result<(), GameError> {
    {
        let mut manager = AssetManager::new();
        let tex_info = manager.load_texture_image("resources/sprites.png")?;
        world.register::<Sprite>();
        let sprite = Sprite::new(&tex_info, (1, 3), (17, 27));

        world.create_entity().with(sprite).build();
        world.insert::<AssetManager>(manager);
    };

    world.insert(WindowState::default());

    let mut renderer = {
        let (width, height) = (800, 600);
        RenderingSystem::new(width, height)
    };

    renderer.setup(world);

    let mut dispatcher = DispatcherBuilder::new()
        .with(FrameLimiterSystem::new(60), "fps_stuff", &[])
        .with_thread_local(renderer)
        .build();

    loop {
        dispatcher.dispatch(world);
    }

    Ok(())
}
