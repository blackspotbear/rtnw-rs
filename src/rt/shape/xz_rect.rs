extern crate cgmath;

use std::borrow::Borrow;
use std::rc::Rc;
use cgmath::vec3;

use super::super::renderer::Ray;
use super::super::renderer::Hitable;
use super::super::renderer::HitRecord;
use super::super::renderer::AABB;
use super::super::material::Material;

pub struct XZRect {
    pub mp: Rc<Material>,
    pub x0: f32,
    pub x1: f32,
    pub z0: f32,
    pub z1: f32,
    pub k: f32
}

impl XZRect {
    pub fn new(x0: f32, x1: f32, z0: f32, z1: f32, k: f32, mat: Rc<Material>) -> Self {
        Self {
            x0: x0,
            x1: x1,
            z0: z0,
            z1: z1,
            k: k,
            mp: mat
        }
    }
}

impl Hitable for XZRect {
    fn hit(&self, r: &Ray, t0: f32, t1: f32) -> Option<HitRecord> {
        let t = (self.k - r.origin().y) / r.direction().y;
        if t < t0 || t > t1 {
            return None
        }
        let x = r.origin().x + t * r.direction().x;
        let z = r.origin().z + t * r.direction().z;
        if x < self.x0 || x > self.x1 || z < self.z0 || z > self.z1 {
            return None
        }
        let u = (x - self.x0) / (self.x1 - self.x0);
        let v = (z - self.z0) / (self.z1 - self.z0);
        assert!(!u.is_nan());
        assert!(!v.is_nan());
        Some(HitRecord {
            u: u,
            v: v,
            t: t,
            mat: self.mp.borrow(),
            p: r.point_at_parameter(t),
            normal: vec3(0.0, 1.0, 0.0)
        })
    }

    fn bounding_box(&self, _t0: f32, _t1: f32) -> Option<AABB> {
        Some(AABB {
            min: vec3(self.x0, self.k - 0.0001, self.z0),
            max: vec3(self.x1, self.k + 0.0001, self.z1)
        })
    }
}

