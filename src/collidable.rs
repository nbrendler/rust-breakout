use cgmath::Point2;

use crate::components::Transform;
use crate::types::OverlapType;

pub trait Collidable {
    fn get_hitbox(&self) -> ((f32, f32), (f32, f32));
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

        let c = center.as_screen_point();

        let x_coll = (c.x <= a0 || c.x >= a1) && (b0 <= c.y && c.y <= b1);
        let y_coll = c.y >= b1 && (a0 <= c.x && c.x <= a1);

        match (x_coll, y_coll) {
            (true, false) => OverlapType::OnlyX,
            (false, true) => OverlapType::OnlyY,
            _ => OverlapType::Both,
        }
    }

    fn corners(&self, center: &Transform) -> ((f32, f32), (f32, f32)) {
        let (bl, tr) = self.get_hitbox();
        let c: Point2<f32> = center.as_screen_point();

        (
            (c.x - bl.0.abs(), c.y - bl.1.abs()),
            (c.x + tr.0.abs(), c.y + tr.1.abs()),
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
