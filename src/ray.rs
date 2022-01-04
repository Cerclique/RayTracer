extern crate geometry3d;

use geometry3d::point3d::Point3D;
use geometry3d::vector3d::Vector3D;

use crate::color::Color;
use crate::random::random_in_hemisphere;

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

    pub fn color(&self, world: &HittableList, depth: i32) -> Color {
        if depth <= 0 {
            return Color::zeroes();
        }

        if let Some(rec) = world.hit(self, 0.001, f64::MAX) {
            let target = rec.hit_point() + Point3D::from_vector3d(random_in_hemisphere(&rec.normal_vec()));
            
            let new_direction = Vector3D::from_point3d(target - rec.hit_point()).unit();
            return Ray::new(rec.hit_point(), new_direction).color(world, depth - 1) * 0.5;            
        }

        let t = (self.direction.unit().y() + 1.0) * 0.5;
        Color::ones() * (1.0 - t) + Color::new(0.5, 0.7, 1.0) * t
    }
}
