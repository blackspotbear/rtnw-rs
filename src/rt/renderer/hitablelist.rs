use super::ray::Ray;
use super::hitable::Hitable;
use super::hit_record::HitRecord;
use super::aabb::AABB;

pub struct HitableList {
    pub hitable: Vec<Box<dyn Hitable>>
}

impl HitableList {
    pub fn new() -> Self {
        HitableList {
            hitable: Vec::new()
        }
    }
}

impl Hitable for HitableList {
    fn hit(&self, r: &Ray, t_min: f32, t_max: f32) -> Option<HitRecord> {
        let mut closest_so_far = t_max;
        let mut result = None;
        for i in 0..self.hitable.len() {
            if let Some(rec) = self.hitable[i].hit(r, t_min, closest_so_far) {
                closest_so_far = rec.t;
                result = Some(rec);
            }
        }
        result
    }

    fn bounding_box(&self, t0: f32, t1: f32) -> Option<AABB> {
        if self.hitable.is_empty() {
            return None
        } else if let Some(mut aabb) = self.hitable[0].bounding_box(t0, t1) {
            for i in 1..self.hitable.len() {
                if let Some(other) = self.hitable[i].bounding_box(t0, t1) {
                    aabb = aabb.surrounding_box(other);
                } else {
                    return None
                }
            }
            return Some(aabb)
        }
        None
    }
}
