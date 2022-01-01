use geometry3d::point3d::Point3D;
use geometry3d::vector3d::Vector3D;

use crate::ray::Ray;

pub struct HitRecord {
    hit_point: Point3D,
    normal_vec: Vector3D,
    t: f64
}

impl HitRecord {
    pub fn new(hit_point: Point3D, normal_vec: Vector3D, t: f64) -> HitRecord{
        HitRecord {hit_point, normal_vec, t}
    }
}

pub trait Hittable {
    fn hit(self, r: Ray, t_min: f64, t_max: f64) -> Option<HitRecord>;
}