use std::cmp::{PartialEq, Eq};
use std::ops::{Add, Sub, Mul};

#[derive(Copy, Debug)]
pub struct Vector3 {
    pub x: f64,
    pub y: f64,
    pub z: f64,
}

impl Vector3 {
    pub fn new(_x: f64, _y: f64, _z: f64) -> Vector3 {
        Vector3 { x: _x, y: _y, z: _z }
    }

    pub fn length_square(&self) -> f64 {
        self.x.powi(2) + self.y.powi(2) + self.z.powi(2)
    }

    pub fn length(&self) -> f64 {
        self.length_square().sqrt()
    }

    pub fn normalize(&mut self) {
        let reciprocal_length = 1.0 / self.length();
        self.x *= reciprocal_length;
        self.y *= reciprocal_length;
        self.z *= reciprocal_length;
    }
}

impl Clone for Vector3 {
    fn clone(&self) -> Vector3 {
        Vector3 {
            x: self.x,
            y: self.y,
            z: self.z,
        }
    }
}

impl Add for Vector3 {
    type Output = Vector3;

    fn add(self, other: Vector3) -> Vector3 {
        Vector3 {
            x: self.x + other.x,
            y: self.y + other.y,
            z: self.z + other.z,
        }
    }
}

impl Sub for Vector3 {
    type Output = Vector3;

    fn sub(self, other: Vector3) -> Vector3 {
        Vector3 {
            x: self.x - other.x,
            y: self.y - other.y,
            z: self.z - other.z,
        }
    }
}

impl Mul<f64> for Vector3 {
    type Output = Vector3;

    fn mul(self, other: f64) -> Vector3 {
        Vector3 {
            x: self.x * other,
            y: self.y * other,
            z: self.z * other,
        }
    }
}

impl Mul<Vector3> for f64 {
    type Output = Vector3;

    fn mul(self, other: Vector3) -> Vector3 {
        Vector3 {
            x: self * other.x,
            y: self * other.y,
            z: self * other.z
        }
    }
}

impl Mul for Vector3 {
    type Output = f64;

    fn mul(self, other: Vector3) -> f64 {
        self.x * other.x + self.y * other.y + self.z * other.z
    }
}

impl PartialEq for Vector3 {
    fn eq(&self, other: &Vector3) -> bool {
        self.x == other.x && self.y == other.y && self.z == other.z
    }
}

impl Eq for Vector3 { }

#[cfg(test)]
mod test {
    use super::Vector3;

    #[test]
    fn test_length_square() {
        let v = Vector3::new(1.0, 2.0, 3.0);
        assert_eq!(v.length_square(), 14.0);
    }

    #[test]
    fn test_length() {
        let v = Vector3::new(1.0, 2.0, 3.0);
        assert_eq!(v.length(), 14_f64.sqrt());
    }

    #[test]
    fn test_normalize() {
        let mut v = Vector3::new(2.0, 0.0, 0.0);
        v.normalize();
        assert_eq!(v, Vector3::new(1.0, 0.0, 0.0));
    }

    #[test]
    fn test_add() {
        let v1 = Vector3::new(1.0, 2.0, 3.0);
        let v2 = Vector3::new(4.0, 5.0, 6.0);
        let sum = v1 + v2;
        assert_eq!(sum, Vector3::new(5.0, 7.0, 9.0));
    }

    #[test]
    fn test_sub() {
        let v1 = Vector3::new(1.0, 2.0, 3.0);
        let v2 = Vector3::new(4.0, 5.0, 6.0);
        let sub = v2 - v1;
        assert_eq!(sub, Vector3::new(3.0, 3.0, 3.0));
    }

    #[test]
    fn test_mul() {
        let v1 = Vector3::new(1.0, 2.0, 3.0);
        let v2 = Vector3::new(4.0, 5.0, 6.0);
        let v3 = Vector3::new(2.0, 4.0, 6.0);
        assert_eq!(v1 * v2, 32.0);
        assert_eq!(2.0 * v1, v3);
        assert_eq!(v1 * 2.0, v3);
    }

}
