extern crate cgmath;

use super::super::renderer::Ray;
use super::super::renderer::Hitable;
use super::super::renderer::HitRecord;
use super::super::renderer::AABB;

pub struct FlipNormals {
    pub ptr: Box<Hitable>
}

impl Hitable for FlipNormals {
    fn hit(&self, r: &Ray, t0: f32, t1: f32) -> Option<HitRecord> {
        if let Some(mut rec) = self.ptr.hit(r, t0, t1) {
            rec.normal = -rec.normal;
            Some(rec)
        } else {
            None
        }
    }

    fn bounding_box(&self, t0: f32, t1: f32) -> Option<AABB> {
        self.ptr.bounding_box(t0, t1)
    }
}
