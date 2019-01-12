extern crate cgmath;

use cgmath::Vector3;

pub fn get_sphere_uv(p: Vector3<f32>) -> (f32, f32) {
    use std::f32::consts::PI;
    let phi = p.z.atan2(p.x);
    let theta = p.y.asin();
    (
        1.0 - (phi + PI) / (2.0 * PI),
        (theta + PI / 2.0) / PI
    )
}
