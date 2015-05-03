mod common;
mod color;
mod math;
mod shape;
mod tracer;
mod view_plane;
mod world;

use color::RGBColor;
use view_plane::ViewPlane;
use world::World;

fn main() {
    let view_plane = ViewPlane::new(640, 480, 1.0, 1.0, color::BLACK);
    let mut w = World::new(view_plane);
    w.build();
    w.render_scene();
}
