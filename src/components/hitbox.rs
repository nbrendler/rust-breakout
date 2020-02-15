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

        let x_overlap;
        let y_overlap;

        if corners.0.x > other_corners.1.x || other_corners.0.x > corners.1.x {
            return OverlapType::None;
        } else {
            x_overlap = true;
        }

        if corners.0.y > other_corners.1.y || other_corners.0.y > corners.1.y {
            return OverlapType::None;
        } else {
            y_overlap = true;
        }

        match (x_overlap, y_overlap) {
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
