extern crate cgmath;

use cgmath::prelude::*;
use cgmath::Vector3;
use cgmath::vec3;

use super::super::renderer::Ray;
use super::super::renderer::HitRecord;
use super::material::Material;
use super::super::math::*;

pub struct Dielectric {
    pub ref_idx: f32
}

impl Material for Dielectric {
    fn scatter(&self, r_in: &Ray, rec: &HitRecord) -> Option<(Ray, Vector3<f32>)> {
        let outward_normal;
        let reflected = reflect(r_in.direction(), rec.normal);
        let ni_over_nt;
        let attenuation = vec3(1.0, 1.0, 1.0);
        let cosine;
        if r_in.direction().dot(rec.normal) > 0.0 {
            outward_normal = -rec.normal;
            ni_over_nt = self.ref_idx;
            cosine = self.ref_idx * r_in.direction().dot(rec.normal) / r_in.direction().magnitude();
        } else {
            outward_normal = rec.normal;
            ni_over_nt = 1.0 / self.ref_idx;
            cosine = -r_in.direction().dot(rec.normal) / r_in.direction().magnitude();
        }
        assert!(!cosine.is_nan());
        if let Some(refracted) = refract(r_in.direction(), outward_normal, ni_over_nt) {
            let reflect_prob = schlick(cosine, self.ref_idx);
            if rand::random::<f32>() > reflect_prob {
                return Some((Ray::new(rec.p, refracted, r_in.time()), attenuation))
            }
        }
        Some((Ray::new(rec.p, reflected, r_in.time()), attenuation))
    }
}
