extern crate geometry3d;

use geometry3d::point3d::Point3D;
use geometry3d::vector3d::Vector3D;

use crate::color::Color;

use crate::hittable_list::HittableList;
use crate::hittable_trait::Hittable;

pub struct Ray {
    origin: Point3D,
    direction: Vector3D,
}

impl Ray {
    pub fn new(origin: Point3D, direction: Vector3D) -> Ray {
        Ray { origin, direction }
    }
}

impl Ray {
    pub fn origin(&self) -> Point3D {
        self.origin
    }

    pub fn direction(&self) -> Vector3D {
        self.direction
    }
}

impl Ray {
    pub fn at(&self, t: f64) -> Point3D {
        self.origin + Point3D::from_vector3d(self.direction * t)
    }

    pub fn color(&self, world: &HittableList) -> Color {
        if let Some(rec) = world.hit(self, 0.0, f64::INFINITY) {
            return Color::new(rec.normal_vec().x() + 1.0, rec.normal_vec().y() + 1.0, rec.normal_vec().z() + 1.0) * 0.5;
        }

        let t = (self.direction.unit().y() + 1.0) * 0.5;
        Color::new(1.0, 1.0, 1.0) * (1.0 - t) + Color::new(0.5, 0.7, 1.0) * t
    }
}
