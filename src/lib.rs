use std::thread::sleep;
use std::time::{Duration, Instant};

use luminance_glfw::{Action, GlfwSurface, Key, Surface as _, WindowEvent};
use specs::prelude::{Builder as _, DispatcherBuilder, World, WorldExt as _};

mod asset_manager;
mod components;
mod game_error;
mod systems;
mod types;
mod util;

use crate::asset_manager::AssetManager;
use crate::components::Sprite;
pub use crate::game_error::GameError;
use crate::systems::RenderingSystem;
pub use crate::types::WindowState;

fn limit_frame_rate<F, V>(fps: u32, main_loop: F) -> impl (Fn() -> V)
where
    F: Fn() -> V,
{
    let frame_duration = Duration::from_secs(1) / fps;
    move || {
        let start = Instant::now();
        let val = main_loop();
        let elapsed = Instant::now() - start;
        if elapsed <= frame_duration {
            sleep(frame_duration - elapsed);
        }

        val
    }
}

pub fn start_app(mut surface: GlfwSurface, world: &mut World) -> Result<(), GameError> {
    let asset_manager = {
        let mut manager = AssetManager::new();
        let tex_info = manager.load_texture_image(&mut surface, "resources/sprites.png")?;
        world.register::<Sprite>();
        let sprite = Sprite::new(&tex_info, (1, 3), (17, 27));

        world.create_entity().with(sprite).build();
        manager
    };

    let mut renderer = {
        let window_state = world.fetch::<WindowState>();
        RenderingSystem::new(window_state.width, window_state.height)
    };

    let mut resize = false;

    'app: loop {
        for event in surface.poll_events() {
            match event {
                WindowEvent::Close | WindowEvent::Key(Key::Escape, _, Action::Release, _) => {
                    break 'app;
                }
                WindowEvent::FramebufferSize(..) => {
                    resize = true;
                }
                _ => {}
            }
        }

        if resize {
            let back_buffer = surface
                .back_buffer()
                .map_err(|_| GameError("error getting the back buffer".to_owned()))?;
            let mut window_state = world.fetch_mut::<WindowState>();
            let width = back_buffer.width();
            let height = back_buffer.height();
            window_state.width = width;
            window_state.height = height;
            renderer.resize(width, height);
        }

        let mut dispatcher = DispatcherBuilder::new().build();
        dispatcher.dispatch(world);

        renderer.render(&mut surface, world, &asset_manager);

        surface.swap_buffers();
    }
    Ok(())
}
