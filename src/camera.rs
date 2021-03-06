use geometry3d::point3d::Point3D;
use geometry3d::vector3d::Vector3D;

use crate::ray::Ray;

pub struct Camera {
    origin: Point3D,
    lower_left_corner: Point3D,
    horizontal: Point3D,
    vertical: Point3D
}

impl Camera {
    pub fn new() -> Camera {
        let aspect_ratio = 16.0 / 9.0;
        let viewport_height = 2.0;
        let viewport_width = aspect_ratio * viewport_height;
        let focal_length = 1.0;
    
        let origin = Point3D::zeroes();
        let horizontal = Point3D::new(viewport_width, 0.0, 0.0);
        let vertical = Point3D::new(0.0, viewport_height, 0.0);
        let lower_left_corner = origin - horizontal * 0.5 - vertical * 0.5 - Point3D::new(0.0, 0.0, focal_length);

        Camera {
            origin,
            lower_left_corner,
            horizontal,
            vertical
        }
    }
}

impl Camera {
    pub fn get_ray(&self, u: f64, v: f64) -> Ray {
        let direction = Vector3D::from_point3d(self.lower_left_corner + self.horizontal * u + self.vertical * v - self.origin);
        Ray::new(self.origin, direction)
    }
}