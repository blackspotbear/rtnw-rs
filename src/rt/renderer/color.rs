extern crate cgmath;

use std::f32;

use cgmath::prelude::*;
use cgmath::Vector3;
use cgmath::vec3;

use super::ray::Ray;
use super::hitable::Hitable;

#[derive(Copy, Clone)]
pub enum Background {
    Black,
    Sky
}

pub fn color(r: &Ray, world: &Hitable, depth: i32, bg: Background) -> Vector3<f32> {
    // Peter says:
    // Some of the reflected rays hit the object they are reflecting off of
    // not at exactly t=0, but instead at t=-0.0000001 or t=0.00000001 or
    // whatever floating point approximation the sphere intersector gives
    // us. So we need to ignore hits very near zero
    if let Some(rec) = world.hit(r, 0.0001, f32::MAX) {
        let emitted = rec.mat.emitted(rec.u, rec.v, rec.p);
        if depth < 50 {
            if let Some((scattered, attenuation)) = rec.mat.scatter(r, &rec) {
                return emitted + attenuation.mul_element_wise(color(&scattered, world, depth + 1, bg));
            }
        }
        emitted
    } else {
        match bg {
            Background::Black => vec3(0.0, 0.0, 0.0),
            Background::Sky => {
                let unit_direction = r.direction().normalize();
                let t = 0.5 * (unit_direction.y + 1.0);
                (1.0 - t) * vec3(1.0, 1.0, 1.0) + t * vec3(0.5, 0.7, 1.0)
            }
        }
    }
}
