use std::cell::RefCell;
use std::f64;
use color;
use color::RGBColor;
use math::{Ray, vector3, Vector3};
use shade_record::ShadeRecord;
use shape::{GeometricObject, Plane, Sphere};

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
        {
            let sphere = Sphere::new(Vector3::new(0.0, -25.0, 0.0), 80.0);
            sphere.set_color(color::RED);
            self.add_object(Box::new(sphere));
        }

        {
            let sphere = Sphere::new(Vector3::new(0.0, 30.0, 0.0), 60.0);
            sphere.set_color(color::YELLOW);
            self.add_object(Box::new(sphere));
        }

        {
            let plane = Plane::new(vector3::ZERO, Vector3::new(0.0, 1.0, 1.0));
            plane.set_color(RGBColor::new(0.0, 0.3, 0.0));
            self.add_object(Box::new(plane));
        }
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

    fn add_object(&self, object: Box<GeometricObject>) {
        self.geometric_objects.borrow_mut().push(object);
    }
}
