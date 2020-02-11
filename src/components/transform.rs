use cgmath::{Matrix4, Vector2, Vector3};
use specs::{storage::DenseVecStorage, Component};

// TODO: rotation
#[derive(Debug)]
pub struct Transform {
    pub position: Vector2<f32>,
    scale: Vector2<f32>,
    pub offsets: [f32; 2],
}

impl Component for Transform {
    type Storage = DenseVecStorage<Self>;
}

impl Default for Transform {
    fn default() -> Transform {
        Transform {
            position: Vector2::new(0., 0.),
            scale: Vector2::new(1., 1.),
            offsets: [0., 0.],
        }
    }
}

impl Transform {
    pub fn with_pos<P: Into<Vector2<f32>>>(mut self, pos: P) -> Self {
        self.position = pos.into();
        self
    }

    pub fn with_scale(mut self, scale: f32) -> Self {
        self.scale = Vector2::new(scale, scale);
        self
    }

    pub fn with_offsets(mut self, offset_x: f32, offset_y: f32) -> Self {
        self.offsets = [offset_x, offset_y];
        self
    }

    pub fn get_matrix(&self) -> Matrix4<f32> {
        let position =
            Matrix4::<f32>::from_translation(Vector3::new(self.position.x, self.position.y, 0.));
        position * Matrix4::<f32>::from_nonuniform_scale(self.scale.x, self.scale.y, 1.0)
    }
}
