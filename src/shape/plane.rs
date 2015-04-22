use math::Vector3;

pub struct Plane {
    point: Vector3,
    normal: Vector3,
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
        }
    }

    pub fn point(&self) -> Vector3 {
        self.point
    }

    pub fn normal(&self) -> Vector3 {
        self.normal
    }
}

#[cfg(test)]
mod test {
    use math::vector3;
    use math::Vector3;
    use super::Plane;

    #[test]
    fn test_new() {
        let p = Plane::new(vector3::ZERO, Vector3::new(1.0, 2.0, 3.0));
        assert!(p.normal().is_normalized());
    }
}
