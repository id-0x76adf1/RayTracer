use color::RGBColor;
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
}
