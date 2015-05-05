use std::cell::Cell;
use color;
use color::RGBColor;
use common;
use math::{Ray, Vector3};
use shape::GeometricObject;
use std::f64;

pub struct Plane {
    point: Vector3,
    normal: Vector3,
    color: Cell<RGBColor>,
}

impl Plane {
    pub fn new(p: Vector3, n: Vector3) -> Plane {
        let mut normal = n;
        if !normal.is_normalized() {
            normal.normalize();
        }
        Plane {
            point: p,
            normal: normal,
            color: Cell::new(color::BLACK),
        }
    }

    pub fn point(&self) -> Vector3 {
        self.point
    }

    pub fn normal(&self) -> Vector3 {
        self.normal
    }
}

impl GeometricObject for Plane {
    fn hit(&self, ray: &Ray) -> (bool, Option<f64>) {
        let t = ((self.point - ray.origin()) * self.normal) / (ray.direction() * self.normal);
        if t > common::EPSILON {
            (true, Some(t))
        } else {
            (false, None)
        }
    }

    fn color(&self) -> RGBColor {
        self.color.get()
    }

    fn set_color(&self, color: RGBColor) {
        self.color.set(color);
    }
}

#[cfg(test)]
mod test {
    use super::Plane;
    use color::RGBColor;
    use math::vector3;
    use math::{Ray, Vector3};
    use shape::GeometricObject;
    use std::f64;

    #[test]
    fn test_new() {
        let p = Plane::new(vector3::ZERO, Vector3::new(1.0, 2.0, 3.0));
        assert!(p.normal().is_normalized());
    }

    #[test]
    fn test_hit() {
        let plane1 = Plane::new(vector3::ZERO, Vector3::new(1.0, 1.0, 1.0));
        let ray1 = Ray::new(vector3::ZERO, Vector3::new(0.0, 1.0, 0.0));
        let (hit1, t1) = plane1.hit(&ray1);
        assert!(!hit1);
        assert_eq!(t1, None);

        let plane2 = Plane::new(vector3::ZERO, Vector3::new(0.0, 1.0, 0.0));
        let ray2 = Ray::new(Vector3::new(0.0, -1.0, 0.0), Vector3::new(0.0, 2.0, 0.0));
        let (hit2, t2) = plane2.hit(&ray2);
        assert!(hit2);
        assert_eq!(t2, Some(1.0));
    }

    #[test]
    fn test_color() {
        let p = Plane::new(vector3::ZERO, Vector3::new(1.0, 2.0, 3.0));
        let c = RGBColor::new(0.1, 0.2, 0.3);
        p.set_color(c);
        assert_eq!(p.color(), c);
    }
}
