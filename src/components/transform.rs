use cgmath::{Matrix4, Vector2, Vector3};
use specs::{storage::DenseVecStorage, Component};

// TODO: rotation
pub struct Transform {
    position: Vector2<f32>,
    scale: Vector2<f32>,
}

impl Component for Transform {
    type Storage = DenseVecStorage<Self>;
}

impl Default for Transform {
    fn default() -> Transform {
        Transform {
            position: Vector2::new(0., 0.),
            scale: Vector2::new(1., 1.),
        }
    }
}

impl Transform {
    pub fn set_pos<P: Into<Vector2<f32>>>(&mut self, pos: P) {
        self.position = pos.into();
    }

    pub fn set_scale<S: Into<Vector2<f32>>>(&mut self, scale: S) {
        self.scale = scale.into();
    }

    pub fn get_matrix(&self) -> Matrix4<f32> {
        let position =
            Matrix4::<f32>::from_translation(Vector3::new(self.position.x, self.position.y, 0.));
        position * Matrix4::<f32>::from_nonuniform_scale(self.scale.x, self.scale.y, 1.0)
    }
}
