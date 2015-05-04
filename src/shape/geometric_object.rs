use color::RGBColor;
use math::Ray;

pub trait GeometricObject {
    fn hit(&self, ray: &Ray) -> (bool, f64);
    
    fn color(&self) -> RGBColor;
    fn set_color(&self, color: RGBColor);
}
