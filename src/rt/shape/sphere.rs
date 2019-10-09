extern crate cgmath;

use std::borrow::Borrow;
use std::rc::Rc;
use cgmath::prelude::*;
use cgmath::Vector3;
use cgmath::vec3;

use super::super::renderer::Ray;
use super::super::renderer::Hitable;
use super::super::renderer::HitRecord;
use super::super::renderer::AABB;
use super::super::material::Material;
use super::utils::get_sphere_uv;

#[derive(Clone)]
pub struct Sphere {
    pub center: Vector3<f32>,
    pub radius: f32,
    pub mat: Rc<dyn Material>
}

impl Sphere {
    pub fn new(center: Vector3<f32>, radius: f32, mat: Rc<dyn Material>) -> Self {
        Sphere { center: center, radius: radius, mat: mat }
    }

    pub fn hit_record(&self, r: &Ray, t: f32) -> HitRecord {
        let p = r.point_at_parameter(t);
        let normal = (p - self.center) / self.radius;
        let (u, v) = get_sphere_uv(normal);
        HitRecord {
            t: t,
            u: u,
            v: v,
            p: p,
            normal: normal,
            mat: self.mat.borrow()
        }
    }
}

impl Hitable for Sphere {
    fn hit(&self, r: &Ray, t_min: f32, t_max: f32) -> Option<HitRecord> {
        let oc = r.origin() - self.center;
        let a = r.direction().dot(r.direction());
        let b = oc.dot(r.direction());
        let c = oc.dot(oc) - self.radius * self.radius;
        let discriminant = b * b - a * c;
        if discriminant > 0.0 {
            let temp = (-b - (b * b - a * c).sqrt()) / a;
            assert!(!temp.is_nan());
            if temp < t_max && temp > t_min {
                return Some(self.hit_record(r, temp));
            }
            let temp = (-b + (b * b - a * c).sqrt()) / a;
            assert!(!temp.is_nan());
            if temp < t_max && temp > t_min {
                return Some(self.hit_record(r, temp));
            }
        }
        None
    }

    fn bounding_box(&self, _t0: f32, _t1: f32) -> Option<AABB> {
        Some(AABB::new(
            self.center - vec3(self.radius, self.radius, self.radius),
            self.center + vec3(self.radius, self.radius, self.radius)
        ))
    }
}
