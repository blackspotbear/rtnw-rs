extern crate cgmath;

use std::f32;
use std::rc::Rc;
use std::borrow::Borrow;
use cgmath::prelude::*;
use cgmath::vec3;

use super::super::renderer::Ray;
use super::super::renderer::Hitable;
use super::super::renderer::HitRecord;
use super::super::renderer::AABB;
use super::super::material::Material;
use super::super::material::Isotropic;
use super::super::texture::Texture;

pub struct ConstantMedium {
    pub boundary: Box<dyn Hitable>,
    pub density: f32,
    pub phase_function: Rc<dyn Material>
}

impl ConstantMedium {
    pub fn new(b: Box<dyn Hitable>, d: f32, a: Box<dyn Texture>) -> Self {
        Self {
            boundary: b,
            density: d,
            phase_function: Rc::new(Isotropic::new(a))
        }
    }
}

impl Hitable for ConstantMedium {
    fn hit(&self, r: &Ray, t_min: f32, t_max: f32) -> Option<HitRecord> {
        if let Some(mut rec1) = self.boundary.hit(r, -f32::MAX, f32::MAX) {
            if let Some(mut rec2) = self.boundary.hit(r, rec1.t + 0.0001, f32::MAX) {
                if rec1.t < t_min {
                    rec1.t = t_min;
                }
                if rec2.t > t_max {
                    rec2.t = t_max;
                }
                if rec1.t >= rec2.t {
                    return None
                }
                if rec1.t < 0.0 {
                    rec1.t = 0.0
                }
                let distance_inside_boundary = (rec2.t - rec1.t) * r.direction().magnitude();
                let hit_distance = -(1.0 / self.density) * (rand::random::<f32>().ln());
                if hit_distance < distance_inside_boundary {
                    let t = rec1.t + hit_distance / r.direction().magnitude();
                    assert!(!t.is_nan());
                    return Some(HitRecord::new(
                        t,
                        0.0,
                        0.0,
                        r.point_at_parameter(t),
                        vec3(1.0, 0.0, 0.0), // arbitrary
                        self.phase_function.borrow()
                    ))
                }
            }
        }

        None
    }

    fn bounding_box(&self, t0: f32, t1: f32) -> Option<AABB> {
        self.boundary.bounding_box(t0, t1)
    }
}
