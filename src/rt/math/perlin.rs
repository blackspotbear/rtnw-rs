extern crate cgmath;
extern crate rand;

use cgmath::prelude::*;
use cgmath::Vector3;
use cgmath::vec3;

pub struct Perlin {
    pub ranvec: Vec<Vector3<f32>>,
    pub perm_x: Vec<i32>,
    pub perm_y: Vec<i32>,
    pub perm_z: Vec<i32>
}

impl Perlin {
    pub fn new() -> Self {
        Perlin {
            ranvec: Self::perlin_generate(),
            perm_x: Self::perlin_generate_perm(),
            perm_y: Self::perlin_generate_perm(),
            perm_z: Self::perlin_generate_perm()
        }
    }

    pub fn perlin_generate() -> Vec<Vector3<f32>> {
        let mut p = Vec::with_capacity(256);
        for _i in 0..256 {
            let v = vec3(
                -1.0 + 2.0 * rand::random::<f32>(),
                -1.0 + 2.0 * rand::random::<f32>(),
                -1.0 + 2.0 * rand::random::<f32>()
            ).normalize();
            p.push(v);
        }
        p
    }

    fn permute(p: &mut Vec<i32>) {
        for i in (1..(p.len())).rev() {
            let target = (rand::random::<f32>() * (i as f32 + 1.0)) as usize;
            let tmp = p[i];
            p[i] = p[target];
            p[target] = tmp;
        }
    }

    fn perlin_generate_perm() -> Vec<i32> {
        let mut p = Vec::with_capacity(256);
        for i in 0..256 {
            p.push(i as i32);
        }
        Self::permute(&mut p);
        p
    }

    fn perlin_interp(c: &[[[Vector3<f32>; 2]; 2]; 2], u: f32, v: f32, w: f32) -> f32 {
        let uu = u * u * (3.0 - 2.0 * u);
        let vv = v * v * (3.0 - 2.0 * v);
        let ww = w * w * (3.0 - 2.0 * w);
        let mut accum = 0.0;

        for ii in 0..2 {
            for jj in 0..2 {
                for kk in 0..2 {
                    let c_ijk = c[ii][jj][kk];
                    let i = ii as f32;
                    let j = jj as f32;
                    let k = kk as f32;
                    let weight_v = vec3(u - i, v - j, w - k);
                    accum +=
                        (i * uu + (1.0 - i) * (1.0 - uu)) *
                        (j * vv + (1.0 - j) * (1.0 - vv)) *
                        (k * ww + (1.0 - k) * (1.0 - ww)) *
                        c_ijk.dot(weight_v)
                }
            }
        }

        accum
    }

    pub fn turb(&self, p: Vector3<f32>, depth: usize) -> f32 {
        let mut accum = 0.0;
        let mut temp_p = p;
        let mut weight = 1.0;

        for _i in 0..depth {
            accum += weight * self.noise(temp_p);
            weight *= 0.5;
            temp_p *= 2.0;
        }

        accum.abs()
    }

    fn noise(&self, p: Vector3<f32>) -> f32 {
        let u = p.x - p.x.floor();
        let v = p.y - p.y.floor();
        let w = p.z - p.z.floor();
        let i = p.x.floor() as usize;
        let j = p.y.floor() as usize;
        let k = p.z.floor() as usize;
        let mut c = [[[vec3(0.0, 0.0, 0.0); 2]; 2]; 2];
        for di in 0..2 {
            for dj in 0..2 {
                for dk in 0..2 {
                    c[di][dj][dk] = self.ranvec[(
                        self.perm_x[(i + di) & 255] ^
                        self.perm_y[(j + dj) & 255] ^
                        self.perm_z[(k + dk) & 255]
                    ) as usize];
                }
            }
        }
        Self::perlin_interp(&c, u, v, w)
    }
}
