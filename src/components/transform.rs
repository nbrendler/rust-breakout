use cgmath::{Matrix4, Point2, SquareMatrix, Vector2, Vector3, Vector4};
use specs::{storage::DenseVecStorage, Component};

pub struct GlobalTransform(pub Transform);

// TODO: rotation
#[derive(Copy, Clone, Debug)]
pub struct Transform {
    pub position: Vector2<f32>,
    pub scale: Vector2<f32>,
    pub global: Matrix4<f32>,
}

impl Component for Transform {
    type Storage = DenseVecStorage<Self>;
}

impl Default for Transform {
    fn default() -> Transform {
        Transform {
            position: Vector2::new(0., 0.),
            scale: Vector2::new(1., 1.),
            global: Matrix4::<f32>::identity(),
        }
    }
}

impl Transform {
    pub fn with_pos<P: Into<Vector2<f32>>>(mut self, pos: P) -> Self {
        self.position = pos.into();
        self
    }

    pub fn with_scale<S: Into<Vector2<f32>>>(mut self, scale: S) -> Self {
        self.scale = scale.into();
        self
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

    pub fn matrix(&self) -> Matrix4<f32> {
        let position =
            Matrix4::<f32>::from_translation(Vector3::new(self.position.x, self.position.y, 0.));
        position * Matrix4::<f32>::from_nonuniform_scale(self.scale.x, self.scale.y, 1.0)
    }

    pub fn as_world_point(&self) -> Point2<f32> {
        let v = (self.matrix() * Vector4::unit_w()).xy();
        Point2::new(v.x, v.y)
    }

    pub fn as_screen_point(&self) -> Point2<f32> {
        let v = (self.global * self.matrix() * Vector4::unit_w()).xy();
        Point2::new(v.x, v.y)
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn transform_default() {
        let t = Transform::default();

        assert_eq!(t.position, Vector2::new(0., 0.));
    }

    #[test]
    fn transform_pos() {
        let t = Transform::default().with_pos((10., 10.));
        assert_eq!(t.position, Vector2::new(10., 10.));
    }

    #[test]
    fn transform_pos_matrix() {
        let t = Transform::default().with_pos((10., 10.));
        let m = t.matrix();

        let result = (m * Vector4::unit_w()).xy();
        assert_eq!(result, Vector2::new(10., 10.));
    }

    #[test]
    fn transform_scale() {
        let t = Transform::default().with_pos((5., 5.)).with_scale((2., 5.));
        assert_eq!(t.scale, Vector2::new(2., 5.));
    }

    #[test]
    fn transform_scale_matrix() {
        let t = Transform::default().with_pos((5., 5.)).with_scale((2., 5.));
        let m = t.matrix();

        let result = (m * Vector4::unit_w()).xy();
        assert_eq!(result, Vector2::new(5., 5.));
    }

    #[test]
    fn transform_global_matrix() {
        let pixels_per_unit: f32 = 15.0;

        let g = Transform::default()
            .with_scale((pixels_per_unit, pixels_per_unit))
            .with_pos((100., 0.));

        let mut t1 = Transform::default().with_pos((0., 0.));
        t1.global = g.matrix();
        let t2 = t1.clone().with_pos((40., 0.));
        let t3 = t1.clone().with_pos((0., 40.));
        let t4 = t1.clone().with_pos((40., 40.));

        assert_eq!(t1.as_screen_point(), (100., 0.).into());
        assert_eq!(t2.as_screen_point(), (700., 0.).into());
        assert_eq!(t3.as_screen_point(), (100., 600.).into());
        assert_eq!(t4.as_screen_point(), (700., 600.).into());
    }
}
