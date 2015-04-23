use math::Ray;

pub trait GeometricObject {
    fn hit(&self, ray: &Ray) -> (bool, f64);
}
