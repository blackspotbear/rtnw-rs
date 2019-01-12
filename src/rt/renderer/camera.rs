extern crate cgmath;
extern crate rand;

use std::f32;
use cgmath::prelude::*;
use cgmath::Vector3;

use super::super::math::*;
use super::ray::Ray;

pub struct Camera {
    pub origin: Vector3<f32>,
    pub lower_left_corner: Vector3<f32>,
    pub horizontal: Vector3<f32>,
    pub vertical: Vector3<f32>,
    pub u: Vector3<f32>,
    pub v: Vector3<f32>,
    pub w: Vector3<f32>,
    pub time0: f32, // shutter open/close times
    pub time1: f32,
    pub lens_radius: f32
}

impl Camera {
    pub fn new(lookfrom: Vector3<f32>, lookat: Vector3<f32>, vup: Vector3<f32>, vfov: f32, aspect: f32, aperture: f32, focus_dist: f32, t0: f32, t1: f32) -> Self {
        let lens_radius = aperture / 2.0;
        let theta = vfov * f32::consts::PI / 180.0;
        let half_height = (theta / 2.0).tan();
        let half_width = aspect * half_height;
        let origin = lookfrom;
        let w = (lookfrom - lookat).normalize();
        let u = vup.cross(w).normalize();
        let v = w.cross(u);
        Camera {
            origin: origin,
            lower_left_corner: origin - half_width * focus_dist * u - half_height * focus_dist * v - focus_dist * w,
            horizontal: 2.0 * half_width * focus_dist * u,
            vertical: 2.0 * half_height * focus_dist * v,
            u: u,
            v: v,
            w: w,
            time0: t0,
            time1: t1,
            lens_radius: lens_radius
        }
    }

    pub fn get_ray(&self, s: f32, t: f32) -> Ray {
        let rd = self.lens_radius * random_in_unit_disk();
        let offset = self.u * rd.x + self.v * rd.y;
        let time = self.time0 + rand::random::<f32>() * (self.time1 - self.time0);
        Ray::new(
            self.origin + offset,
            self.lower_left_corner + s * self.horizontal + t * self.vertical - self.origin - offset,
            time
        )
    }
}
