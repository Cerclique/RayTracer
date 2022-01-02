use crate::hittable_trait::Hittable;
use crate::hitrecord::HitRecord;

use crate::ray::Ray;

pub struct HittableList {
    objects: Vec<Box<dyn Hittable>>,
}

impl HittableList {
    pub fn new() -> HittableList {
        HittableList {
            objects: Vec::new(),
        }
    }
}

impl HittableList {
    pub fn add<T: Hittable + 'static>(&mut self, obj: T) {
        self.objects.push(Box::new(obj));
    }
}

impl Hittable for HittableList {
    fn hit(&self, r: &Ray, t_min: f64, t_max: f64) -> Option<HitRecord> {
        let mut closest_so_far = t_max;
        let mut output_result: Option<HitRecord> = None;

        for obj in &self.objects {
            if let Some(record) = obj.hit(r, t_min, closest_so_far) {
                closest_so_far = record.t();
                output_result = Some(record);
            }
        }
        
        output_result
    }
}

