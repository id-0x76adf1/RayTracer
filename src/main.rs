mod common;
mod color;
mod math;
mod shape;
mod view_plane;
mod world;

use color::RGBColor;
use world::World;

fn main() {
    let mut w = World::new(RGBColor::new(0.0, 0.0, 0.0));
    w.build();
    w.render_scene();
}
