extern crate cgmath;

use cgmath::Vector3;

use super::super::renderer::Ray;
use super::super::renderer::HitRecord;
use super::material::Material;
use super::super::texture::Texture;


pub struct DiffuseLight {
    pub emit: Box<Texture>
}

impl Material for DiffuseLight {
    fn scatter(&self, _r_in: &Ray, _rec: &HitRecord) -> Option<(Ray, Vector3<f32>)> {
        None
    }

    fn emitted(&self, u: f32, v: f32, p: Vector3<f32>) -> Vector3<f32> {
        self.emit.value(u, v, p)
    }
}
