extern crate cgmath;

use cgmath::prelude::*;
use cgmath::Vector3;

use super::super::renderer::Ray;
use super::super::renderer::HitRecord;
use super::material::Material;
use super::super::math::*;

pub struct Metal {
    pub albedo: Vector3<f32>,
    pub fuzz: f32
}

impl Metal {
    pub fn new(albedo: Vector3<f32>, fuzz: f32) -> Self {
        Metal {
            albedo: albedo,
            fuzz: if fuzz < 1.0 { fuzz } else { 1.0 }
        }
    }
}

impl Material for Metal {
    fn scatter(&self, r_in: &Ray, rec: &HitRecord) -> Option<(Ray, Vector3<f32>)> {
        let reflected = reflect(r_in.direction().normalize(), rec.normal);
        let scattered = Ray::new(rec.p, reflected + self.fuzz * random_in_unit_sphere(), r_in.time());
        if scattered.direction().dot(rec.normal) > 0.0 {
            Some((scattered, self.albedo))
        } else {
            None
        }
    }
}
