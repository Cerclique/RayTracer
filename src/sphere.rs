use geometry3d::point3d::Point3D;
use geometry3d::vector3d::Vector3D;

use crate::ray::Ray;
use crate::hittable_trait::{Hittable, HitRecord};

#[derive(Debug, Copy, Clone)]
pub struct Sphere {
    center: Point3D,
    radius: f64,
}

impl Sphere {
    pub fn new(center: Point3D, radius: f64) -> Sphere {
        Sphere { center, radius }
    }
}

impl Sphere {
    pub fn center(self) -> Point3D {
        self.center
    }

    pub fn radius(self) -> f64 {
        self.radius
    }
}

impl Hittable for Sphere {
    fn hit(self, r: Ray, t_min: f64, t_max: f64) -> Option<HitRecord> {
        let oc = Vector3D::from_point3d(r.origin() - self.center);
        let a = r.direction().length_squared();
        let half_b = oc.dot(r.direction());
        let c = oc.length_squared() - self.radius().powi(2);

        let discriminant = half_b.powi(2) - a * c;
        
        if discriminant < 0.0 {
            return None;
        }

        let sqrd = discriminant.sqrt();

        let root = (-half_b - sqrd) / a;
        if root  < t_min || root > t_max {
            let root = (-half_b + sqrd) / a;
            if root  < t_min || root > t_max {
                return None;
            }
        }

        let hit_point = r.at(root);
        let normal_vec = Vector3D::from_point3d((hit_point - self.center) / self.radius);

        Some(HitRecord::new(hit_point, normal_vec, root))
    }
}