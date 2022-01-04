use rand::Rng;

use geometry3d::vector3d::Vector3D;

pub fn randf() -> f64 {
    rand::thread_rng().gen::<f64>()
}

fn bounded_randf(min: f64, max: f64) -> f64 {
    min + ((max - min) * randf())
}

fn random_vector3d(min: f64, max: f64) -> Vector3D {
    Vector3D::new(bounded_randf(min, max), bounded_randf(min, max), bounded_randf(min, max))
}

pub fn rand_vector_in_unit_sphere() -> Vector3D {
    loop {
        let v = random_vector3d(-1.0, 1.0);
        let a = v.length_squared();
        if a >= 1.0 {
            continue;
        }
        return v;
    }
}

pub fn random_in_hemisphere(normal: &Vector3D) -> Vector3D {
    let v = rand_vector_in_unit_sphere();
    if v.dot(*normal) > 0.0 {
        return v;
    }
    -v
}
