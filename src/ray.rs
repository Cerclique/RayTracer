extern crate geometry3d;

use geometry3d::point3d::Point3D;
use geometry3d::vector3d::Vector3D;

use crate::color::Color;
use crate::sphere::Sphere;

#[derive(Debug, Clone, Copy)]
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
    pub fn at(self, t: f64) -> Point3D {
        self.origin + Point3D::from_vector3d(self.direction * t)
    }

    pub fn color(self) -> Color {
        let obj = Sphere::new(Point3D::new(0.0, 0.0, -1.0), 0.5);
        let t = self.hit_sphere(obj);
        if t > 0.0 {
            let normal_vec = (Vector3D::from_point3d(self.at(t)) - Vector3D::new(0.0, 0.0, -1.0)).unit();
            let color = Color::new(normal_vec.x() + 1.0, normal_vec.y() + 1.0, normal_vec.z() + 1.0) * 0.5;
            return color;
        }
        let t = (self.direction.unit().y() + 1.0) * 0.5;
        Color::new(1.0, 1.0, 1.0) * (1.0 - t) + Color::new(0.5, 0.7, 1.0) * t
    }

    pub fn hit_sphere(self, obj: Sphere) -> f64 {
        let oc = Vector3D::from_point3d(self.origin - obj.center());
        let a = self.direction.dot(self.direction);
        let b = oc.dot(self.direction) * 2.0;
        let c = oc.dot(oc) - obj.radius() * obj.radius();

        let discriminant = (b * b) - (4.0 * a * c);

        if discriminant < 0.0 {
            return -1.0;
        } else {
            return (-b - discriminant.sqrt()) / (2.0 * a);
        }
    }
}
