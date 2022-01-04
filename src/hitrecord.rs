use geometry3d::point3d::Point3D;
use geometry3d::vector3d::Vector3D;

#[derive(Copy, Clone)]
pub struct HitRecord {
    hit_point: Point3D,
    normal_vec: Vector3D,
    t: f64,
    front_face: bool
}

impl HitRecord {
    pub fn new(hit_point: Point3D, normal_vec: Vector3D, t: f64, front_face: bool) -> HitRecord{
        HitRecord {hit_point, normal_vec, t, front_face}
    }
}

impl HitRecord {
    pub fn t(&self) -> f64 {
        self.t
    }

    pub fn normal_vec(&self) -> Vector3D {
        self.normal_vec
    }

    pub fn hit_point(&self) -> Point3D {
        self.hit_point
    }
}
