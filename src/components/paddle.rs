use specs::prelude::*;

use crate::asset_manager::AssetManager;
use crate::components::{Hitbox, Sprite, Transform};
use crate::constants::WORLD_WIDTH;

#[derive(Default)]
pub struct Paddle;

impl Component for Paddle {
    type Storage = NullStorage<Self>;
}

impl Paddle {
    pub fn init(world: &mut World) {
        let tex_info = {
            let mut asset_manager = world.fetch_mut::<AssetManager>();
            asset_manager
                .load_texture_image("resources/paddle.png")
                .expect("Failed to load paddle texture")
        };

        let s1 = Sprite::new(&tex_info, (0, 0), (50, 15));
        let t1 = Transform::default()
            .with_pos((WORLD_WIDTH / 2.0, 0.0))
            .with_scale(50., 15.)
            .with_offsets(0.5, 0.5);
        let hb1 = Hitbox::new((0., 0.), (50., 15.));
        world
            .create_entity()
            .with(s1)
            .with(t1)
            .with(Paddle)
            .with(hb1)
            .build();
    }
}
