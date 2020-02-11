use cgmath::{Matrix4, Vector4};
use specs::prelude::*;

use crate::asset_manager::AssetManager;
use crate::components::{Sprite, Transform};
use crate::game_error::GameError;

pub fn init(world: &mut World) -> Result<(), GameError> {
    let tex_info = {
        let mut asset_manager = world.fetch_mut::<AssetManager>();
        asset_manager.load_texture_image("resources/paddle.png")?
    };
    let sprite = Sprite::new(&tex_info, (0, 0), (50, 15));
    let mut transform = Transform::default();

    let world_transform = Matrix4::<f32>::from_nonuniform_scale(50.0, 15.0, 1.0);
    let pos = (world_transform * Vector4::new(8., 0., 0., 0.)).xy();

    transform = transform.with_pos(pos).with_offsets(25., 7.5);

    world.create_entity().with(sprite).with(transform).build();

    Ok(())
}
