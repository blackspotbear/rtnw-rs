extern crate cgmath;

use cgmath::Vector3;
use super::texture::Texture;

pub struct CheckerTexture {
    pub odd: Box<dyn Texture>,
    pub even: Box<dyn Texture>
}

impl CheckerTexture {
    pub fn new(t0: Box<dyn Texture>, t1: Box<dyn Texture>) -> Self {
        CheckerTexture { odd: t0, even: t1 }
    }
}

impl Texture for CheckerTexture {
    fn value(&self, u: f32, v: f32, p: Vector3<f32>) -> Vector3<f32> {
        let sines = (10.0 * p.x).sin() * (10.0 * p.y).sin() * (10.0 * p.z).sin();
        if sines < 0.0 {
            self.odd.value(u, v, p)
        } else {
            self.even.value(u, v, p)
        }
    }
}
