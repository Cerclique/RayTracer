use geometry3d::point3d::Point3D;

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
