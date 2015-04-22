use math::Vector3;

pub struct Ray {
    origin: Vector3,
    direction: Vector3,
}

impl Ray {
    pub fn new(o: Vector3, d: Vector3) -> Ray {
        Ray {
            origin: o,
            direction: d,
        }
    }

    pub fn origin(&self) -> Vector3 {
        self.origin
    }

    pub fn direction(&self) -> Vector3 {
        self.direction
    }
}
