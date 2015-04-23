use common;
use math::{Ray, Vector3};
use shape::GeometricObject;
use std::f64;

pub struct Sphere {
    center: Vector3,
    radius: f64,
}

impl Sphere {
    pub fn new(c: Vector3, r: f64) -> Sphere {
        Sphere {
            center: c,
            radius: r,
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
    fn hit(&self, ray: &Ray) -> (bool, f64) {
        let delta = ray.origin() - self.center;
        let a = ray.direction().length_square();
        let b = 2.0 * delta * ray.direction();
        let c = delta.length_square() - self.radius.powi(2);

        let discriminant = b.powi(2) - 4.0 * a * c;

        if discriminant < 0.0 {
            return (false, f64::MAX);
        } else {
            let denominator = 2.0 * a;
            let root = discriminant.sqrt();

            let t = (-b - root) / denominator;
            if t > common::EPSILON {
                return (true, t);
            }

            let t = (-b + root) / denominator;
            if t > common::EPSILON {
                return (true, t);
            }
        }

        (false, f64::MAX)
    }
}

#[cfg(test)]
mod test {
    use super::Sphere;
    use math::vector3;
    use math::{Ray, Vector3};
    use shape::GeometricObject;
    use std::f64;

    #[test]
    fn test_hit() {
        let sphere = Sphere::new(vector3::ZERO, 1.0);

        let ray1 = Ray::new(Vector3::new(0.0, 1.0, 0.0), Vector3::new(0.0, 1.0, 0.0));
        let (hit1, t1) = sphere.hit(&ray1);
        assert!(!hit1);
        assert_eq!(t1, f64::MAX);

        let ray2 = Ray::new(vector3::ZERO, Vector3::new(0.0, 1.0, 0.0));
        let (hit2, t2) = sphere.hit(&ray2);
        assert!(hit2);
        assert_eq!(t2, 1.0);

        let ray3 = Ray::new(Vector3::new(0.0, -1.0, 0.0), Vector3::new(0.0, 1.0, 0.0));
        let (hit3, t3) = sphere.hit(&ray3);
        assert!(hit3);
        assert_eq!(t2, 1.0);
   }
}