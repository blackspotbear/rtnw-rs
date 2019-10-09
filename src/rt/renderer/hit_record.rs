extern crate cgmath;

use cgmath::Vector3;
use super::super::material::Material;

#[derive(Copy, Clone)]
pub struct HitRecord<'a> {
    pub t: f32,
    pub u: f32,
    pub v: f32,
    pub p: Vector3<f32>,
    pub normal: Vector3<f32>,
    pub mat: &'a dyn Material
}

impl<'a> HitRecord<'a> {
    pub fn new(t: f32, u: f32, v: f32, p: Vector3<f32>, normal: Vector3<f32>, mat: &'a dyn Material) -> HitRecord<'a> {
        HitRecord {
            t: t,
            u: u,
            v: v,
            p: p,
            normal: normal,
            mat: mat
        }
    }
}
