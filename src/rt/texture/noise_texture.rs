extern crate cgmath;

use cgmath::Vector3;
use cgmath::vec3;

use super::texture::Texture;
use super::super::math::Perlin;

pub struct NoiseTexture {
    pub noise: Perlin,
    pub scale: f32
}

impl NoiseTexture {
    pub fn new(sc: f32) -> Self {
        NoiseTexture { noise: Perlin::new(), scale: sc }
    }
}

impl Texture for NoiseTexture {
    fn value(&self, _u: f32, _v: f32, p: Vector3<f32>) -> Vector3<f32> {
        // vec3(1.0, 1.0, 1.0) * 0.5 * (1.0 + self.noise.turb(self.scale * p, 7))
        // vec3(1.0, 1.0, 1.0) * self.noise.turb(self.scale * p, 7)
        // vec3(1.0, 1.0, 1.0) * 0.5 * (1.0 + (self.scale * p.z + 10.0 * self.noise.turb(p, 7)).sin())
        vec3(1.0, 1.0, 1.0) * 0.5 * (1.0 + (self.scale * p.x + 5.0 * self.noise.turb(self.scale * p, 7)).sin())
    }
}
