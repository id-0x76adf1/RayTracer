use color::RGBColor;
use shape::GeometricObject;

pub struct World {
    background_color: RGBColor,
    geometric_objects: Vec<Box<GeometricObject>>,
}

impl World {
    pub fn new(color: RGBColor) -> World {
        World {
            background_color: color,
            geometric_objects: Vec::new()
        }
    }

    pub fn build(&mut self) {
    }

    pub fn render_scene(&mut self) {
    }
}
