use std::cell::RefCell;
use std::f64;
use color;
use math::{Ray, vector3};
use shade_record::ShadeRecord;
use shape::{GeometricObject, Sphere};

pub struct World {
    geometric_objects: RefCell<Vec<Box<GeometricObject>>>,
}

impl World {
    pub fn new() -> World {
        World {
            geometric_objects: RefCell::new(Vec::new()),
        }
    }

    pub fn build(&self) {
        let sphere = Sphere::new(vector3::ZERO, 100.0);
        sphere.set_color(color::RED);
        self.geometric_objects.borrow_mut().push(Box::new(sphere));
    }

    pub fn hit_objects(&self, ray: &Ray) -> Option<ShadeRecord> {
        let mut result = None;
        let mut tmin = f64::MAX;
        for o in self.geometric_objects.borrow().iter() {
            let (hit, t, s) = o.hit(ray);
            if hit && t.unwrap() < tmin {
                result = s;
                tmin = t.unwrap();
            }
        }
        result
    }
}
