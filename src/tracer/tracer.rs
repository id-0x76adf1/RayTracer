use color::RGBColor;
use math::Ray;

pub trait Tracer {
    fn trace_ray(ray: &Ray) -> RGBColor;
}
