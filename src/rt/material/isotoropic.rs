extern crate cgmath;

use cgmath::Vector3;

use super::super::renderer::Ray;
use super::super::renderer::HitRecord;
use super::material::Material;
use super::super::texture::Texture;
use super::super::math::*;

pub struct Isotropic {
    pub albedo: Box<dyn Texture>
}

impl Isotropic {
    pub fn new(a: Box<dyn Texture>) -> Self {
        Isotropic {
            albedo: a
        }
    }
}

impl Material for Isotropic {
    fn scatter(&self, r_in: &Ray, rec: &HitRecord) -> Option<(Ray, Vector3<f32>)> {
        Some((
            Ray::new(rec.p, random_in_unit_sphere(), r_in.time()),
            self.albedo.value(rec.u, rec.v, rec.p)
        ))
    }
}
