use cgmath::{Matrix4, Point2, Vector2, Vector3, Vector4};
use specs::{storage::DenseVecStorage, Component};

// TODO: rotation
#[derive(Copy, Clone, Debug)]
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

    pub fn with_scale(mut self, scale_x: f32, scale_y: f32) -> Self {
        self.scale = Vector2::new(scale_x, scale_y);
        self
    }

    pub fn with_offsets(mut self, offset_x: f32, offset_y: f32) -> Self {
        self.offsets = [offset_x, offset_y];
        self
    }

    pub fn x(&self) -> f32 {
        self.position.x
    }

    pub fn y(&self) -> f32 {
        self.position.y
    }

    pub fn set_x<T: Into<f32>>(&mut self, x: T) {
        self.position.x = x.into();
    }

    pub fn set_y<T: Into<f32>>(&mut self, y: T) {
        self.position.y = y.into();
    }

    pub fn move_left<T: Into<f32>>(&mut self, x: T) {
        self.position.x -= x.into();
    }

    pub fn move_right<T: Into<f32>>(&mut self, x: T) {
        self.position.x += x.into();
    }

    pub fn get_matrix(&self) -> Matrix4<f32> {
        let position =
            Matrix4::<f32>::from_translation(Vector3::new(self.position.x, self.position.y, 0.));
        let offset =
            Matrix4::<f32>::from_translation(Vector3::new(-self.offsets[0], -self.offsets[1], 1.0));
        Matrix4::<f32>::from_nonuniform_scale(self.scale.x, self.scale.y, 1.0) * offset * position
    }

    pub fn as_screen_point(&self) -> Point2<f32> {
        let v = Vector4::unit_w();
        let transformed = (self.get_matrix() * v).xy();
        Point2::new(transformed.x, transformed.y)
    }

    pub fn as_world_point(&self) -> Point2<f32> {
        Point2::new(self.x(), self.y())
    }
}
