extern crate cgmath;

use std::borrow::Borrow;
use std::rc::Rc;
use cgmath::vec3;

use super::super::renderer::Ray;
use super::super::renderer::Hitable;
use super::super::renderer::HitRecord;
use super::super::renderer::AABB;
use super::super::material::Material;

pub struct YZRect {
    pub mp: Rc<Material>,
    pub y0: f32,
    pub y1: f32,
    pub z0: f32,
    pub z1: f32,
    pub k: f32
}

impl YZRect {
    pub fn new(y0: f32, y1: f32, z0: f32, z1: f32, k: f32, mat: Rc<Material>) -> Self {
        Self {
            y0: y0,
            y1: y1,
            z0: z0,
            z1: z1,
            k: k,
            mp: mat
        }
    }
}

impl Hitable for YZRect {
    fn hit(&self, r: &Ray, t0: f32, t1: f32) -> Option<HitRecord> {
        let t = (self.k - r.origin().x) / r.direction().x;
        if t < t0 || t > t1 {
            return None
        }
        let y = r.origin().y + t * r.direction().y;
        let z = r.origin().z + t * r.direction().z;
        if y < self.y0 || y > self.y1 || z < self.z0 || z > self.z1 {
            return None
        }
        let u = (y - self.y0) / (self.y1 - self.y0);
        let v = (z - self.z0) / (self.z1 - self.z0);
        if u.is_nan() || v.is_nan() {
            return None
        }
        Some(HitRecord {
            u: u,
            v: v,
            t: t,
            mat: self.mp.borrow(),
            p: r.point_at_parameter(t),
            normal: vec3(1.0, 0.0, 0.0)
        })
    }

    fn bounding_box(&self, _t0: f32, _t1: f32) -> Option<AABB> {
        Some(AABB {
            min: vec3(self.k - 0.0001, self.y0, self.z0),
            max: vec3(self.k + 0.0001, self.y1, self.z1)
        })
    }
}
