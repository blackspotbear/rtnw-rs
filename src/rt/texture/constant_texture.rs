extern crate cgmath;

use cgmath::Vector3;

use super::texture::Texture;

pub struct ConstantTexture {
    pub color: Vector3<f32>
}

impl ConstantTexture {
    pub fn new(c: Vector3<f32>) -> Self {
        ConstantTexture { color: c }
    }
}

impl Texture for ConstantTexture {
    fn value(&self, _u: f32, _v: f32, _p: Vector3<f32>) -> Vector3<f32> {
        self.color
    }
}
