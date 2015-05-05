use std::f64;
use color::RGBColor;
use math::Ray;
use shade_record::ShadeRecord;
use shape::GeometricObject;
use view_plane::ViewPlane;

pub struct World {
    view_plane: ViewPlane,
    geometric_objects: Vec<Box<GeometricObject>>,
}

impl World {
    pub fn new(_view_plane: ViewPlane) -> World {
        World {
            view_plane: _view_plane,
            geometric_objects: Vec::new()
        }
    }

    pub fn build(&mut self) {
    }

    pub fn render_scene(&mut self) {
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
