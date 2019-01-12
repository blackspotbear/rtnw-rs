extern crate cgmath;

use cgmath::Vector3;
use cgmath::vec3;

use super::texture::Texture;

pub struct ImageTexture {
    pub data: Vec<u8>,
    pub nx: usize,
    pub ny: usize
}

impl ImageTexture {
    pub fn new(pixels: Vec<u8>, a: usize, b: usize) -> Self {
        ImageTexture{ data: pixels, nx: a, ny: b }
    }
}

impl Texture for ImageTexture {
    fn value(&self, u: f32, v: f32, _p: Vector3<f32>) -> Vector3<f32> {
        let i = ((u * self.nx as f32).max(0.0) as usize).min(self.nx - 1);
        let j = (((1.0 - v) * (self.ny as f32) - 0.001).max(0.0) as usize).min(self.ny - 1);
        let r = (self.data[3 * i + 3 * self.nx * j    ] as f32) / 255.0;
        let g = (self.data[3 * i + 3 * self.nx * j + 1] as f32) / 255.0;
        let b = (self.data[3 * i + 3 * self.nx * j + 2] as f32) / 255.0;
        vec3(r, g, b)
    }
}
