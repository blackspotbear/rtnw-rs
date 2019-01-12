extern crate cgmath;

use cgmath::Vector3;

use super::super::renderer::Ray;
use super::super::renderer::Hitable;
use super::super::renderer::HitRecord;
use super::super::renderer::AABB;

pub struct Translate {
    pub ptr: Box<Hitable>,
    pub offset: Vector3<f32>
}

impl Translate {
    pub fn new(p: Box<Hitable>, displacement: Vector3<f32>) -> Self {
        Translate {
            ptr: p,
            offset: displacement
        }
    }
}

impl Hitable for Translate {
    fn hit(&self, r: &Ray, t0: f32, t1: f32) -> Option<HitRecord> {
        let moved_r = Ray::new(r.origin() - self.offset, r.direction(), r.time());
        if let Some(mut rec) = self.ptr.hit(&moved_r, t0, t1) {
            rec.p += self.offset;
            Some(rec)
        } else {
            None
        }
    }

    fn bounding_box(&self, t0: f32, t1: f32) -> Option<AABB> {
        if let Some(aabb) = self.ptr.bounding_box(t0, t1) {
            Some(AABB::new(aabb.min() + self.offset, aabb.max() + self.offset))
        } else {
            None
        }
    }
}
