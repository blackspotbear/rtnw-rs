extern crate cgmath;

use std::borrow::Borrow;
use std::rc::Rc;
use cgmath::vec3;

use super::super::renderer::Ray;
use super::super::renderer::Hitable;
use super::super::renderer::HitRecord;
use super::super::renderer::AABB;
use super::super::material::Material;

pub struct XYRect {
    pub mp: Rc<Material>,
    pub x0: f32,
    pub x1: f32,
    pub y0: f32,
    pub y1: f32,
    pub k: f32
}

impl XYRect {
    pub fn new(x0: f32, x1: f32, y0: f32, y1: f32, k: f32, mat: Rc<Material>) -> Self {
        Self {
            x0: x0,
            x1: x1,
            y0: y0,
            y1: y1,
            k: k,
            mp: mat
        }
    }
}

impl Hitable for XYRect {
    fn hit(&self, r: &Ray, t0: f32, t1: f32) -> Option<HitRecord> {
        let t = (self.k - r.origin().z) / r.direction().z;
        assert!(!t.is_nan());
        if t < t0 || t > t1 {
            return None
        }
        let x = r.origin().x + t * r.direction().x;
        let y = r.origin().y + t * r.direction().y;
        if x < self.x0 || x > self.x1 || y < self.y0 || y > self.y1 {
            return None
        }
        let u = (x - self.x0) / (self.x1 - self.x0);
        let v = (y - self.y0) / (self.y1 - self.y0);
        assert!(!u.is_nan());
        assert!(!v.is_nan());
        Some(HitRecord {
            u: u,
            v: v,
            t: t,
            mat: self.mp.borrow(),
            p: r.point_at_parameter(t),
            normal: vec3(0.0, 0.0, 1.0)
        })
    }

    fn bounding_box(&self, _t0: f32, _t1: f32) -> Option<AABB> {
        Some(AABB {
            min: vec3(self.x0, self.y0, self.k - 0.0001),
            max: vec3(self.x1, self.y1, self.k + 0.0001)
        })
    }
}
