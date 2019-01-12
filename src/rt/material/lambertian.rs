extern crate cgmath;

use cgmath::Vector3;

use super::super::renderer::Ray;
use super::super::renderer::HitRecord;
use super::material::Material;
use super::super::texture::Texture;
use super::super::math::*;

pub struct Lambertian {
    pub albedo: Box<Texture>
}

impl Material for Lambertian {
    fn scatter(&self, r_in: &Ray, rec: &HitRecord) -> Option<(Ray, Vector3<f32>)> {
        let target = rec.p + rec.normal + random_in_unit_sphere();
        Some((Ray::new(rec.p, target - rec.p, r_in.time()), self.albedo.value(rec.u, rec.v, rec.p)))
    }
}
