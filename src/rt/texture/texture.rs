extern crate cgmath;

use cgmath::Vector3;

pub trait Texture {
    fn value(&self, u: f32, v: f32, p: Vector3<f32>) -> Vector3<f32>;
}
