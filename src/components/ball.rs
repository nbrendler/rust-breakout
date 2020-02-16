use cgmath::{InnerSpace, Vector2};
use specs::prelude::*;

use crate::asset_manager::AssetManager;
use crate::components::{Hitbox, Sprite, Transform};
use crate::constants::{BALL_SPEED, WORLD_HEIGHT};

pub struct Ball {
    pub velocity: Vector2<f32>,
}

impl Component for Ball {
    type Storage = HashMapStorage<Self>;
}

impl Ball {
    pub fn init(world: &mut World) {
        let tex_info = {
            let mut asset_manager = world.fetch_mut::<AssetManager>();
            asset_manager
                .load_texture_image("resources/ball.png")
                .expect("Failed to load ball texture")
        };

        let initial_dir = (Vector2::unit_x() - Vector2::unit_y()).normalize();

        let ball = Ball {
            velocity: initial_dir * BALL_SPEED,
        };
        let s1 = Sprite::new(&tex_info, (0, 0), (15, 15));
        let t1 = Transform::default()
            .with_pos((0.5, WORLD_HEIGHT / 2.0))
            .with_scale(15., 15.)
            .with_offsets(0.5, 0.5);
        let hb1 = Hitbox::new((0., 0.), (15., 15.));
        world
            .create_entity()
            .with(s1)
            .with(t1)
            .with(ball)
            .with(hb1)
            .build();
    }
}
