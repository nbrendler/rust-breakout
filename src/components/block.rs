use specs::prelude::*;

use crate::asset_manager::AssetManager;
use crate::components::{IsCollidable, Sprite, Transform};
use crate::constants::{WORLD_HEIGHT, WORLD_WIDTH};

#[derive(Default)]
pub struct Block;

impl Component for Block {
    type Storage = NullStorage<Self>;
}

impl Block {
    pub fn init(world: &mut World) {
        let global_t = {
            Transform::default()
                .with_pos((100., 0.))
                .with_scale((15., 15.))
        };
        let tex_info = {
            let mut asset_manager = world.fetch_mut::<AssetManager>();
            asset_manager
                .load_texture_image("resources/block.png")
                .expect("Failed to load block texture")
        };

        let mut s1 = Sprite::new(&tex_info, (0, 0), (30, 15));
        s1.offsets = [0.5, 0.5];
        for x in 0..(WORLD_WIDTH / 2.0) as u32 {
            for y in (WORLD_HEIGHT - 4.) as u32..WORLD_HEIGHT as u32 {
                let mut t1 = Transform::default().with_pos((2.0 * x as f32 + 0.5, y as f32 - 0.5));
                t1.global = global_t.matrix();
                world
                    .create_entity()
                    .with(s1)
                    .with(t1)
                    .with(Block)
                    .with(IsCollidable)
                    .build();
            }
        }
    }
}
