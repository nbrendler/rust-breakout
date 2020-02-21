use specs::prelude::*;

#[derive(Default)]
pub struct IsCollidable;

impl Component for IsCollidable {
    type Storage = NullStorage<Self>;
}
