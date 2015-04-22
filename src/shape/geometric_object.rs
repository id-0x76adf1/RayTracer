use math::Ray;

pub trait GeometricObject {
    fn hit(ray: &Ray) -> (bool, f64);
}
