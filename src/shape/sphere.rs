use std::cell::Cell;
use color;
use color::RGBColor;
use common;
use math::{Ray, Vector3};
use shade_record::{ShadeRecord, ShadeRecordBuilder};
use shape::GeometricObject;
use std::f64;

pub struct Sphere {
    center: Vector3,
    radius: f64,
    color: Cell<RGBColor>
}

impl Sphere {
    pub fn new(c: Vector3, r: f64) -> Sphere {
        Sphere {
            center: c,
            radius: r,
            color: Cell::new(color::BLACK),
        }
    }

    pub fn center(&self) -> Vector3 {
        self.center
    }

    pub fn radius(&self) -> f64 {
        self.radius
    }
}

impl GeometricObject for Sphere {
    fn hit(&self, ray: &Ray) -> (bool, Option<f64>, Option<ShadeRecord>) {
        let delta = ray.origin() - self.center;
        let a = ray.direction().length_square();
        let b = 2.0 * delta * ray.direction();
        let c = delta.length_square() - self.radius.powi(2);

        let discriminant = b.powi(2) - 4.0 * a * c;

        if discriminant < 0.0 {
            return (false, None, None);
        } else {
            let denominator = 2.0 * a;
            let root = discriminant.sqrt();

            let build_result = |t: f64, color: RGBColor| {
                let s = ShadeRecordBuilder::new()
                    .hit_an_object(true)
                    .color(color)
                    .finalize();
                (true, Some(t), Some(s))
            };

            let t = (-b - root) / denominator;
            if t > common::EPSILON {
                return build_result(t, self.color());
            }

            let t = (-b + root) / denominator;
            if t > common::EPSILON {
                return build_result(t, self.color());
            }
        }

        (false, None, None)
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
    use super::Sphere;
    use color::RGBColor;
    use math::vector3;
    use math::{Ray, Vector3};
    use shape::GeometricObject;
    use std::f64;

    #[test]
    fn test_hit() {
        let sphere = Sphere::new(vector3::ZERO, 1.0);
        let c = RGBColor::new(0.1, 0.2, 0.3);
        sphere.set_color(c);

        let ray1 = Ray::new(Vector3::new(0.0, 1.0, 0.0), Vector3::new(0.0, 1.0, 0.0));
        let (hit1, t1, s1) = sphere.hit(&ray1);
        assert!(!hit1);
        assert!(t1.is_none());
        assert!(s1.is_none());

        let ray2 = Ray::new(vector3::ZERO, Vector3::new(0.0, 1.0, 0.0));
        let (hit2, t2, s2) = sphere.hit(&ray2);
        assert!(hit2);
        assert_eq!(t2, Some(1.0));
        match s2 {
            Some(s) => {
                assert!(s.hit_an_object());
                assert_eq!(s.color(), c);
            },
            None => assert!(false)
        }

        let ray3 = Ray::new(Vector3::new(0.0, -1.0, 0.0), Vector3::new(0.0, 1.0, 0.0));
        let (hit3, t3, s3) = sphere.hit(&ray3);
        assert!(hit3);
        assert_eq!(t2, Some(1.0));
        assert!(s3.is_some());
   }

   #[test]
   fn test_color() {
        let sphere = Sphere::new(vector3::ZERO, 1.0);
        let c = RGBColor::new(0.1, 0.2, 0.3);
        sphere.set_color(c);
        assert_eq!(sphere.color(), c);
   }
}