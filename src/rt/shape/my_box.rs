extern crate cgmath;

use std::rc::Rc;
use cgmath::Vector3;

use super::super::renderer::Ray;
use super::super::renderer::Hitable;
use super::super::renderer::HitableList;
use super::super::renderer::HitRecord;
use super::super::renderer::AABB;
use super::super::material::Material;
use super::xy_rect::XYRect;
use super::xz_rect::XZRect;
use super::yz_rect::YZRect;
use super::flip_normals::FlipNormals;

pub struct MyBox {
    pub pmin: Vector3<f32>,
    pub pmax: Vector3<f32>,
    pub list_ptr: HitableList
}

impl MyBox {
    pub fn new(p0: Vector3<f32>, p1: Vector3<f32>, ptr: Rc<dyn Material>) -> Self {
        let mut list_ptr = HitableList::new();

        list_ptr.hitable.push(Box::new(XYRect::new(p0.x, p1.x, p0.y, p1.y, p1.z, ptr.clone())));
        list_ptr.hitable.push(Box::new(FlipNormals{ptr: Box::new(XYRect::new(p0.x, p1.x, p0.y, p1.y, p0.z, ptr.clone()))}));

        list_ptr.hitable.push(Box::new(XZRect::new(p0.x, p1.x, p0.z, p1.z, p1.y, ptr.clone())));
        list_ptr.hitable.push(Box::new(FlipNormals{ptr: Box::new(XZRect::new(p0.x, p1.x, p0.z, p1.z, p0.y, ptr.clone()))}));

        list_ptr.hitable.push(Box::new(YZRect::new(p0.y, p1.y, p0.z, p1.z, p1.x, ptr.clone())));
        list_ptr.hitable.push(Box::new(FlipNormals{ptr: Box::new(YZRect::new(p0.y, p1.y, p0.z, p1.z, p0.x, ptr.clone()))}));

        MyBox {
            pmin: p0,
            pmax: p1,
            list_ptr: list_ptr
        }
    }
}

impl Hitable for MyBox {
    fn hit(&self, r: &Ray, t0: f32, t1: f32) -> Option<HitRecord> {
        self.list_ptr.hit(r, t0, t1)
    }

    fn bounding_box(&self, _t0: f32, _t1: f32) -> Option<AABB> {
        Some(AABB{min: self.pmin, max: self.pmax})
    }
}
