extern crate cgmath;

use std::f32;
use cgmath::vec3;

use super::super::renderer::Ray;
use super::super::renderer::Hitable;
use super::super::renderer::HitRecord;
use super::super::renderer::AABB;

pub struct RotateY {
    pub ptr: Box<Hitable>,
    pub sin_theta: f32,
    pub cos_theta: f32,
    pub bbox: Option<AABB>
}

impl RotateY {
    pub fn new(p: Box<Hitable>, angle: f32) -> Self {
        use std::f32::consts::PI;
        let radians = (PI / 180.0) * angle;
        let sin_theta = radians.sin();
        let cos_theta = radians.cos();
        let bbox = p.bounding_box(0.0, 1.0);
        if let Some(bbox) = bbox {
            let mut min = vec3(f32::MAX, f32::MAX, f32::MAX);
            let mut max = vec3(-f32::MAX, -f32::MAX, -f32::MAX);
            for i in 0..2 {
                for j in 0..2 {
                    for k in 0..2 {
                        let i = i as f32;
                        let j = j as f32;
                        let k = k as f32;
                        let x = i * bbox.max().x + (1.0 - i) * bbox.min().x;
                        let y = j * bbox.max().y + (1.0 - i) * bbox.min().y;
                        let z = k * bbox.max().z + (1.0 - i) * bbox.min().z;
                        let newx = cos_theta * x + sin_theta * z;
                        let newz = -sin_theta * x + cos_theta * z;
                        let tester = vec3(newx, y, newz);
                        for c in 0..3 {
                            if tester[c] > max[c] {
                                max[c] = tester[c];
                            }
                            if tester[c] < min[c] {
                                min[c] = tester[c];
                            }
                        }
                    }
                }
            }
            Self {
                ptr: p,
                sin_theta: sin_theta,
                cos_theta: cos_theta,
                bbox: Some(AABB::new(min, max))
            }
        } else {
            Self {
                ptr: p,
                sin_theta: sin_theta,
                cos_theta: cos_theta,
                bbox: None
            }
        }
    }
}

impl Hitable for RotateY {
    fn hit(&self, r: &Ray, t_min: f32, t_max: f32) -> Option<HitRecord> {
        let mut origin = r.origin();
        let mut direction = r.direction();
        origin[0] = self.cos_theta * r.origin()[0] - self.sin_theta * r.origin()[2];
        origin[2] = self.sin_theta * r.origin()[0] + self.cos_theta * r.origin()[2];
        direction[0] = self.cos_theta * r.direction()[0] - self.sin_theta * r.direction()[2];
        direction[2] = self.sin_theta * r.direction()[0] + self.cos_theta * r.direction()[2];
        let rotated_r = Ray::new(origin, direction, r.time());
        if let Some(mut rec) = self.ptr.hit(&rotated_r, t_min, t_max) {
            let mut p = rec.p;
            let mut normal = rec.normal;
            p[0] = self.cos_theta * rec.p[0] + self.sin_theta * rec.p[2];
            p[2] = -self.sin_theta * rec.p[0] + self.cos_theta * rec.p[2];
            normal[0] = self.cos_theta * rec.normal[0] + self.sin_theta * rec.normal[2];
            normal[2] = -self.sin_theta * rec.normal[0] + self.cos_theta * rec.normal[2];
            rec.p = p;
            rec.normal = normal;
            Some(rec)
        } else {
            None
        }
    }

    fn bounding_box(&self, _t0: f32, _t1: f32) -> Option<AABB> {
        self.bbox
    }
}
