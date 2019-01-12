extern crate cgmath;
extern crate rand;

use cgmath::Vector3;
use cgmath::vec3;
use cgmath::prelude::*;
use rand::random;

pub fn random_in_unit_sphere() -> Vector3<f32> {
    loop {
        let p = 2.0 * vec3(random::<f32>(), random::<f32>(), random::<f32>()) - vec3(1.0, 1.0, 1.0);
        if p.magnitude2() < 1.0 {
            return p
        }
    }
}

pub fn random_in_unit_disk() -> Vector3<f32> {
    loop {
        let p = 2.0 * vec3(random::<f32>(), random::<f32>(), 0.0) - vec3(1.0, 1.0, 0.0);
        if p.dot(p) < 1.0 {
            return p
        }
    }
}

pub fn reflect(v: Vector3<f32>, n: Vector3<f32>) -> Vector3<f32> {
    v - 2.0 * v.dot(n) * n
}

pub fn refract(v: Vector3<f32>, n: Vector3<f32>, ni_over_nt: f32) -> Option<Vector3<f32>> {
    let uv = v.normalize();
    let dt = uv.dot(n);
    let discriminant = 1.0 - ni_over_nt * ni_over_nt * (1.0 - dt * dt);
    if discriminant > 0.0 {
        Some(ni_over_nt * (uv - n * dt) - n * discriminant.sqrt())
    } else {
        None
    }
}

pub fn schlick(cosine: f32, ref_idx: f32) -> f32 {
    let mut r0 = (1.0 - ref_idx) / (1.0 + ref_idx);
    r0 = r0 * r0;
    r0 + (1.0 - r0) * (1.0 - cosine).powf(5.0)
}
