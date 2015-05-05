use color::RGBColor;
use math::Ray;
use shade_record::ShadeRecord;

pub trait GeometricObject {
    fn hit(&self, ray: &Ray) -> (bool, Option<f64>, Option<ShadeRecord>);

    fn color(&self) -> RGBColor;
    fn set_color(&self, color: RGBColor);
}
