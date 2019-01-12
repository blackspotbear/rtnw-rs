extern crate cgmath;

use cgmath::Vector3;
use cgmath::vec3;
use super::ray::Ray;

#[derive(Copy, Clone)]
pub struct AABB {
    pub min: Vector3<f32>,
    pub max: Vector3<f32>
}

impl AABB {
    pub fn new(a: Vector3<f32>, b: Vector3<f32>) -> Self {
        AABB { min: a, max: b }
    }

    pub fn min(&self) -> Vector3<f32> {
        self.min
    }

    pub fn max(&self) -> Vector3<f32> {
        self.max
    }

    pub fn hit(self: &AABB, r: &Ray, mut tmin: f32, mut tmax: f32) -> bool {
        for a in 0..3 {
            let t0 = ((self.min[a] - r.origin()[a]) / r.direction()[a]).min( (self.max[a] - r.origin()[a]) / r.direction()[a]);
            let t1 = ((self.min[a] - r.origin()[a]) / r.direction()[a]).max( (self.max[a] - r.origin()[a]) / r.direction()[a]);
            assert!(!t0.is_nan());
            assert!(!t1.is_nan());
            tmin = t0.max(tmin);
            tmax = t1.min(tmax);
            if tmax <= tmin {
                return false
            }
        }
        true
    }

    pub fn surrounding_box(&self, other: AABB) -> AABB {
        let small = vec3(
            self.min().x.min(other.min().x),
            self.min().y.min(other.min().y),
            self.min().z.min(other.min().z),
        );
        let big = vec3(
            self.max().x.max(other.max().x),
            self.max().y.max(other.max().y),
            self.max().z.max(other.max().z),
        );
        AABB::new(small, big)
    }
}

