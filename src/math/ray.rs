use math::Vector3;

pub struct Ray {
    origin: Vector3,
    direction: Vector3,
}

impl Ray {
    pub fn new(_origin: Vector3, _direction: Vector3) -> Ray {
        Ray {
            origin: _origin,
            direction: _direction,
        }
    }

    pub fn origin(&self) -> Vector3 {
        self.origin
    }

    pub fn direction(&self) -> Vector3 {
        self.direction
    }
}
