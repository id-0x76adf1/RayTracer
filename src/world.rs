use std::f64;
use math::Ray;
use shade_record::ShadeRecord;
use shape::GeometricObject;

pub struct World {
    geometric_objects: Vec<Box<GeometricObject>>,
}

impl World {
    pub fn new() -> World {
        World {
            geometric_objects: Vec::new(),
        }
    }

    pub fn hit_objects(&self, ray: &Ray) -> Option<ShadeRecord> {
        let mut result = None;
        let mut tmin = f64::MAX;
        for o in self.geometric_objects.iter() {
            let (hit, t, s) = o.hit(ray);
            if hit && t.unwrap() < tmin {
                result = s;
                tmin = t.unwrap();
            }
        }
        result
    }
}
