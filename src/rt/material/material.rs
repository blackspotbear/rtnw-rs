extern crate cgmath;

use cgmath::prelude::*;
use cgmath::Vector3;

use super::super::renderer::Ray;
use super::super::renderer::HitRecord;

pub trait Material {
    fn scatter(&self, r_in: &Ray, rec: &HitRecord) -> Option<(Ray, Vector3<f32>)>;
    fn emitted(&self, _u: f32, _v: f32, _p: Vector3<f32>) -> Vector3<f32> {
        Vector3::zero()
    }
}
