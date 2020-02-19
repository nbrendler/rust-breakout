use specs::prelude::*;

use crate::asset_manager::AssetManager;
use crate::components::{Sprite, Transform};
use crate::constants::WORLD_WIDTH;

#[derive(Default)]
pub struct Paddle;

impl Component for Paddle {
    type Storage = NullStorage<Self>;
}

impl Paddle {
    pub fn init(world: &mut World) {
        let global_t = {
            use cgmath::{Matrix4, Vector3};
            Transform::default()
                .with_pos((100., 0.))
                .with_scale((15., 15.))
        };
        let tex_info = {
            let mut asset_manager = world.fetch_mut::<AssetManager>();
            asset_manager
                .load_texture_image("resources/paddle.png")
                .expect("Failed to load paddle texture")
        };

        let mut s1 = Sprite::new(&tex_info, (0, 0), (50, 15));
        s1.offsets = [0.5, 0.5];
        let mut t1 = Transform::default().with_pos((WORLD_WIDTH / 2.0, 1.0));
        t1.global = global_t.matrix();
        world.create_entity().with(s1).with(t1).with(Paddle).build();
    }
}
