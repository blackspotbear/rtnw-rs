extern crate cgmath;

use std::borrow::Borrow;
use std::rc::Rc;
use cgmath::prelude::*;
use cgmath::Vector3;
use cgmath::vec3;
use super::utils::get_sphere_uv;

use super::super::renderer::Ray;
use super::super::renderer::Hitable;
use super::super::renderer::HitRecord;
use super::super::renderer::AABB;
use super::super::material::Material;

pub struct MovingSphere {
    pub center0: Vector3<f32>,
    pub center1: Vector3<f32>,
    pub time0: f32,
    pub time1: f32,
    pub radius: f32,
    pub mat: Rc<dyn Material>
}

impl MovingSphere {
    pub fn new(cen0: Vector3<f32>, cen1: Vector3<f32>, t0: f32, t1: f32, r: f32, m: Rc<dyn Material>) -> MovingSphere {
        MovingSphere {
            center0: cen0,
            center1: cen1,
            time0: t0,
            time1: t1,
            radius: r,
            mat: m
        }
    }

    pub fn center(&self, time: f32) -> Vector3<f32> {
        let t = (time - self.time0) / (self.time1 - self.time0);
        assert!(!t.is_nan());
        self.center0 + t * (self.center1 - self.center0)
    }

    pub fn hit_record(&self, r: &Ray, t: f32) -> HitRecord {
        let p = r.point_at_parameter(t);
        let normal = (p - self.center(r.time())) / self.radius;
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

impl Hitable for MovingSphere {
    fn hit(&self, r: &Ray, t_min: f32, t_max: f32) -> Option<HitRecord> {
        let oc = r.origin() - self.center(r.time());
        let a = r.direction().dot(r.direction());
        let b = oc.dot(r.direction());
        let c = oc.dot(oc) - self.radius * self.radius;
        let discriminant = b * b - a * c;
        if discriminant > 0.0 {
            let temp = (-b - (b * b - a * c).sqrt()) / a;
            if temp < t_max && temp > t_min {
                return Some(self.hit_record(r, temp));
            }
            let temp = (-b + (b * b - a * c).sqrt()) / a;
            if temp < t_max && temp > t_min {
                return Some(self.hit_record(r, temp));
            }
        }
        None
    }

    fn bounding_box(&self, t0: f32, t1: f32) -> Option<AABB> {
        Some(AABB::new(
            self.center(t0) - vec3(self.radius, self.radius, self.radius),
            self.center(t0) + vec3(self.radius, self.radius, self.radius)
        ).surrounding_box(
            AABB::new(
                self.center(t1) - vec3(self.radius, self.radius, self.radius),
                self.center(t1) + vec3(self.radius, self.radius, self.radius)
            )
        ))
    }

}

