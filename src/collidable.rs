use cgmath::Point2;

use crate::components::Transform;
use crate::types::OverlapType;

pub trait Collidable {
    fn get_hitbox(&self, transform: &Transform) -> ((i32, i32), (i32, i32));
    fn intersects<C: Collidable>(
        &self,
        other: &C,
        center: &Transform,
        other_center: &Transform,
    ) -> OverlapType {
        let ((x0, y0), (x1, y1)) = self.corners(center);
        let ((a0, b0), (a1, b1)) = other.corners(other_center);

        if x0 > a1 || a0 > x1 {
            return OverlapType::None;
        }

        if y0 > b1 || b0 > y1 {
            return OverlapType::None;
        }

        let y_overlap = (y0 - b1).abs().min((b0 - y1).abs());
        let x_overlap = (x0 - a1).abs().min((a0 - x1).abs());

        match (x_overlap == 0, y_overlap == 0) {
            (true, false) => OverlapType::OnlyX,
            (false, true) => OverlapType::OnlyY,
            (true, true) => OverlapType::Both,
            _ => OverlapType::None,
        }
    }

    fn corners(&self, center: &Transform) -> ((i32, i32), (i32, i32)) {
        let (bl, tr) = self.get_hitbox(center);
        let c: Point2<f32> = center.as_screen_point();

        let half_width = (tr.0 - bl.0) / 2;
        let half_height = (tr.1 - bl.1) / 2;
        (
            (
                c.x.round() as i32 - half_width,
                c.y.round() as i32 - half_height,
            ),
            (
                c.x.round() as i32 + half_width,
                c.y.round() as i32 + half_height,
            ),
        )
    }
}

#[cfg(test)]
mod tests {
    use super::*;
    use crate::components::{Sprite, Transform};
    use crate::types::TextureInfo;

    #[test]
    fn test_sprite_hitbox_intersect_corner() {
        let t1 = Transform::default().with_pos((0.0, 0.0));
        let tex = TextureInfo::new(0, 100, 50);
        let mut s1 = Sprite::new(&tex, (0, 0), (100, 50));
        s1.offsets = [0.5, 0.5];

        let t2 = Transform::default().with_pos((100., 50.));
        let tex = TextureInfo::new(0, 100, 50);
        let mut s2 = Sprite::new(&tex, (0, 0), (100, 50));
        s2.offsets = [0.5, 0.5];

        assert_eq!(s1.intersects(&s2, &t1, &t2), OverlapType::Both);
    }

    #[test]
    fn test_sprite_hitbox_intersect_x() {
        let t1 = Transform::default().with_pos((0.0, 0.0));
        let tex = TextureInfo::new(0, 100, 50);
        let mut s1 = Sprite::new(&tex, (0, 0), (100, 50));
        s1.offsets = [0.5, 0.5];

        let t2 = Transform::default().with_pos((100., 0.));
        let tex = TextureInfo::new(0, 100, 50);
        let mut s2 = Sprite::new(&tex, (0, 0), (100, 50));
        s2.offsets = [0.5, 0.5];

        assert_eq!(s1.intersects(&s2, &t1, &t2), OverlapType::OnlyX);
    }

    #[test]
    fn test_sprite_hitbox_intersect_y() {
        let t1 = Transform::default().with_pos((0.0, 0.0));
        let tex = TextureInfo::new(0, 100, 50);
        let mut s1 = Sprite::new(&tex, (0, 0), (100, 50));
        s1.offsets = [0.5, 0.5];

        let t2 = Transform::default().with_pos((0., 50.));
        let tex = TextureInfo::new(0, 100, 50);
        let mut s2 = Sprite::new(&tex, (0, 0), (100, 50));
        s2.offsets = [0.5, 0.5];

        assert_eq!(s1.intersects(&s2, &t1, &t2), OverlapType::OnlyY);
    }

    #[test]
    fn test_sprite_hitbox_no_intersect() {
        let t1 = Transform::default().with_pos((0.0, 0.0));
        let tex = TextureInfo::new(0, 100, 50);
        let mut s1 = Sprite::new(&tex, (0, 0), (100, 50));
        s1.offsets = [0.5, 0.5];

        let t2 = Transform::default().with_pos((200., 100.));
        let tex = TextureInfo::new(0, 100, 50);
        let mut s2 = Sprite::new(&tex, (0, 0), (100, 50));
        s2.offsets = [0.5, 0.5];

        assert_eq!(s1.intersects(&s2, &t1, &t2), OverlapType::None);
    }
}
