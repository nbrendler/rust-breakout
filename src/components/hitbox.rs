use cgmath::Point2;

use specs::prelude::*;

use crate::types::OverlapType;

#[derive(Default)]
pub struct Hitbox {
    bottom_left: (f32, f32),
    top_right: (f32, f32),
}

impl Component for Hitbox {
    type Storage = DenseVecStorage<Self>;
}

impl Hitbox {
    pub fn new(bottom_left: (f32, f32), top_right: (f32, f32)) -> Self {
        Hitbox {
            bottom_left,
            top_right,
        }
    }

    pub fn intersects<T: Into<Point2<f32>>>(
        &self,
        other: &Hitbox,
        center: T,
        other_center: T,
    ) -> OverlapType {
        let corners = self.corners(center.into());
        let other_corners = other.corners(other_center.into());

        if corners.0.x > other_corners.1.x || other_corners.0.x > corners.1.x {
            return OverlapType::None;
        }

        if corners.0.y > other_corners.1.y || other_corners.0.y > corners.1.y {
            return OverlapType::None;
        }

        let y_overlap = (corners.0.y - other_corners.1.y)
            .abs()
            .min((other_corners.0.y - corners.1.y).abs());
        let x_overlap = (corners.0.x - other_corners.1.x)
            .abs()
            .min((other_corners.0.x - corners.1.x).abs());

        match (x_overlap < 1.0, y_overlap < 1.0) {
            (true, false) => OverlapType::OnlyX,
            (false, true) => OverlapType::OnlyY,
            (true, true) => OverlapType::Both,
            (false, false) => OverlapType::None,
        }
    }

    pub fn corners(&self, center: Point2<f32>) -> (Point2<f32>, Point2<f32>) {
        (
            (center.x + self.bottom_left.0, center.y + self.bottom_left.1).into(),
            (center.x + self.top_right.0, center.y + self.top_right.1).into(),
        )
    }
}

#[cfg(test)]
mod tests {
    use super::*;
    use crate::components::Transform;

    #[test]
    fn test_hitbox() {
        let t1 = Transform::default()
            .with_pos((0.0, 0.0))
            .with_scale(15., 15.)
            .with_offsets(0.5, 0.5);
        let hb1 = Hitbox::new((0., 0.), (15., 15.));

        let t2 = Transform::default()
            .with_pos((7.5, 7.5))
            .with_scale(15., 15.)
            .with_offsets(0.5, 0.5);
        let hb2 = Hitbox::new((0., 0.), (15., 15.));

        println!("{:?} {:?}", t1.as_screen_point(), t2.as_screen_point());
        println!(
            "{:?} {:?}",
            hb1.corners(t1.as_screen_point()),
            hb2.corners(t2.as_screen_point())
        );

        assert_eq!(
            hb2.intersects(&hb1, t1.as_world_point(), t2.as_world_point()),
            OverlapType::Both
        );
    }
}
