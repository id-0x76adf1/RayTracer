use std::rc::Rc;
use color::RGBColor;
use math::Ray;
use tracer::Tracer;
use world::World;

pub struct MultipleObjects {
    world: Rc<World>,
}

impl MultipleObjects {
    pub fn new(w: Rc<World>) -> MultipleObjects {
        MultipleObjects {
            world: w,
        }
    }
}

impl Tracer for MultipleObjects {
    fn trace_ray(&self, ray: &Ray) -> RGBColor {
        RGBColor::new(0.0, 0.0, 0.0)
    }
}
