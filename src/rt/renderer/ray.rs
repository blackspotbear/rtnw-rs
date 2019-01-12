extern crate cgmath;

use cgmath::Vector3;

pub struct Ray {
    pub a: Vector3<f32>,
    pub b: Vector3<f32>,
    pub time: f32
}

impl Ray {
    pub fn new(a: Vector3<f32>, b: Vector3<f32>, ti: f32) -> Self {
        Ray { a: a, b: b, time: ti }
    }
    pub fn origin(&self) -> Vector3<f32> { self.a }
    pub fn direction(&self) -> Vector3<f32> { self.b }
    pub fn time(&self) -> f32 { self.time }
    pub fn point_at_parameter(&self, t: f32) -> Vector3<f32> { self.a + self.b * t }
}
