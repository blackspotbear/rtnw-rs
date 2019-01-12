
use std::rc::Rc;

use super::ray::Ray;
use super::hitable::Hitable;
use super::hit_record::HitRecord;
use super::aabb::AABB;

pub struct BVHNode {
    left: Rc<Hitable>,
    right: Rc<Hitable>,
    aabb: AABB
}

impl BVHNode {
    pub fn new(l: &mut [Rc<Hitable>], time0: f32, time1: f32) -> Self {
        let axis = (rand::random::<f32>() * 3.0) as i32;
        l.sort_by(|a, b| {
            let aabb_left = a.bounding_box(0.0, 0.0).unwrap();
            let aabb_right = b.bounding_box(0.0, 0.0).unwrap();
            let diff = match axis {
                0 => aabb_left.min().x - aabb_right.min().x,
                1 => aabb_left.min().y - aabb_right.min().y,
                _ => aabb_left.min().z - aabb_right.min().z
            };
            if diff < 0.0 {
                std::cmp::Ordering::Less
            } else {
                std::cmp::Ordering::Greater
            }
        });
        let left;
        let right;
        if l.len() == 1 {
            left = l[0].clone();
            right = l[0].clone();
        } else if l.len() == 2 {
            left = l[0].clone();
            right = l[1].clone();
        } else {
            let len = l.len();
            let len2 = l.len() / 2;
            left = Rc::new(BVHNode::new(&mut l[0..len2], time0, time1));
            right = Rc::new(BVHNode::new(&mut l[len2..len], time0, time1));
        }

        let left_aabb = left.bounding_box(time0, time1).unwrap();
        let right_aabb = right.bounding_box(time0, time1).unwrap();
        BVHNode {
            left: left,
            right: right,
            aabb: left_aabb.surrounding_box(right_aabb)
        }
    }
}

impl Hitable for BVHNode {
    fn hit(&self, r: &Ray, t_min: f32, t_max: f32) -> Option<HitRecord> {
        if self.aabb.hit(r, t_min, t_max) {
            let hit_left = self.left.hit(r, t_min, t_max);
            let hit_right = self.right.hit(r, t_min, t_max);
            if let (Some(left_rec), Some(right_rec)) = (hit_left, hit_right) {
                if left_rec.t < right_rec.t {
                    return hit_left
                } else {
                    return hit_right
                }
            } else if hit_left.is_some() {
                return hit_left
            } else if hit_right.is_some() {
                return hit_right
            }
        }
        None
    }

    fn bounding_box(&self, _t0: f32, _t1: f32) -> Option<AABB> {
        Some(self.aabb)
    }
}
