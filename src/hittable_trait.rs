use crate::ray::Ray;
use crate::hitrecord::HitRecord;

pub trait Hittable {
    fn hit(&self, r: &Ray, t_min: f64, t_max: f64) -> Option<HitRecord>;
}