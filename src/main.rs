mod common;
mod color;
mod math;
mod ppm;
mod renderer;
mod shade_record;
mod shape;
mod tracer;
mod view_plane;
mod world;

use std::cell::RefCell;
use std::rc::Rc;
use color::RGBColor;
use renderer::Renderer;
use tracer::{Tracer, MultipleObjects};
use view_plane::ViewPlane;
use world::World;

fn main() {
    let mut view_plane = ViewPlane::new(300, 300, 1.0, 1.0, RGBColor::new(0.5 , 0.5, 0.5));
    let world = Rc::new(RefCell::new(World::new()));
    let tracer = Box::new(MultipleObjects::new(world.clone())) as Box<Tracer>;
    let renderer = Renderer::new(tracer);

    world.borrow().build();
    renderer.render_world(&mut view_plane);
    ppm::write("output", view_plane.width(), view_plane.height(), view_plane.pixels());

}
