use math::{Ray, Vector3};
use tracer::Tracer;
use view_plane::{ViewPlane, PixelPosition};

pub struct Renderer {
    tracer: Box<Tracer>,
}

impl Renderer {
    pub fn new(_tracer: Box<Tracer>) -> Renderer {
        Renderer {
            tracer: _tracer,
        }
    }

    pub fn render_world(&self, view_plane: &mut ViewPlane) {
        let direction = Vector3::new(0.0, 0.0, -1.0);
        let width = view_plane.width() as f64;
        let height = view_plane.height() as f64;
        let pixel_size = view_plane.pixel_size() as f64;

        let z: f64 = 100.0;

        for r in 0..view_plane.height() {
            let row = r as f64;
            let y = (row + 0.5 - 0.5 * height) * pixel_size;
            for c in 0..view_plane.width() {
                let column = c as f64;
                let x = (column + 0.5 - 0.5 * width) * pixel_size;
                let ray = Ray::new(Vector3::new(x, y, z), direction);
                let pixel_color = self.tracer.trace_ray(&ray);
                
                match pixel_color {
                    Some(color) => {
                        let p = PixelPosition {
                            x: c,
                            y: r,
                        };
                        view_plane[p] = color;
                    },
                    None => { }
                }
            }
        }
    }
}
