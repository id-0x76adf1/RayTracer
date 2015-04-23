use math::Vector3;

pub struct Sphere {
    point: Vector3,
    radius: f64,
}

impl Sphere {
    pub fn new(p: Vector3, r: f64) -> Sphere {
        Sphere {
            point: p,
            radius: r,
        }
    }

    pub fn point(&self) -> Vector3 {
        self.point
    }

    pub fn radius(&self) -> f64 {
        self.radius
    }
}
