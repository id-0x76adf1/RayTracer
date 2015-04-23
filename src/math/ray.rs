use math::Vector3;

pub struct Ray {
    origin: Vector3,
    direction: Vector3,
}

impl Ray {
    pub fn new(o: Vector3, d: Vector3) -> Ray {
        let mut direction = d;
        if !direction.is_normalized() {
            direction.normalize()
        }
        Ray {
            origin: o,
            direction: direction,
        }
    }

    pub fn origin(&self) -> Vector3 {
        self.origin
    }

    pub fn direction(&self) -> Vector3 {
        self.direction
    }
}

#[cfg(test)]
mod test {
    use super::Ray;
    use math::vector3;
    use math::Vector3;

    #[test]
    fn test_new() {
        let r = Ray::new(vector3::ZERO, Vector3::new(1.0, 2.0, 3.0));
        assert!(r.direction().is_normalized());
    }
}
