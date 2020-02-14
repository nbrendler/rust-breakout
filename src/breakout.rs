use specs::prelude::*;

use crate::asset_manager::AssetManager;
use crate::components::{Sprite, Transform};
use crate::game_error::GameError;

pub fn init(world: &mut World) -> Result<(), GameError> {
    let tex_info = {
        let mut asset_manager = world.fetch_mut::<AssetManager>();
        asset_manager.load_texture_image("resources/paddle.png")?
    };

    let s1 = Sprite::new(&tex_info, (0, 0), (50, 15));
    let t1 = Transform::default()
        .with_pos((5.0, 0.0))
        .with_offsets(0.5, 0.5);
    world.create_entity().with(s1).with(t1).build();

    Ok(())
}
