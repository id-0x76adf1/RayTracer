use color::RGBColor;
use math::Vector3;

pub struct ShadeRecord {
    hit_an_object: bool,
    local_hit_point: Option<Vector3>,
    normal: Option<Vector3>,
    color: Option<RGBColor>,
}

impl ShadeRecord {
    pub fn hit_an_object(&self) -> bool {
        self.hit_an_object
    }

    pub fn local_hit_point(&self) -> Vector3 {
        self.local_hit_point.unwrap()
    }

    pub fn normal(&self) -> Vector3 {
        self.normal.unwrap()
    }

    pub fn color(&self) -> RGBColor {
        self.color.unwrap()
    }
}

pub struct ShadeRecordBuilder {
    hit_an_object: bool,
    local_hit_point: Option<Vector3>,
    normal: Option<Vector3>,
    color: Option<RGBColor>,
}

impl ShadeRecordBuilder {
    pub fn new() -> ShadeRecordBuilder {
        ShadeRecordBuilder {
            hit_an_object: false,
            local_hit_point: None,
            normal: None,
            color: None,
        }
    }

    pub fn hit_an_object(&mut self, v: bool) -> &mut ShadeRecordBuilder {
        self.hit_an_object = v;
        self
    }

    pub fn local_hit_point(&mut self, v: Vector3) -> &mut ShadeRecordBuilder {
        self.local_hit_point = Some(v);
        self
    }

    pub fn normal(&mut self, v: Vector3) -> &mut ShadeRecordBuilder {
        let mut normal = v;
        if !normal.is_normalized() {
            normal.normalize();
        }
        self.normal = Some(normal);
        self
    }

    pub fn color(&mut self, v: RGBColor) -> &mut ShadeRecordBuilder {
        self.color = Some(v);
        self
    }

    pub fn finalize(&self) -> ShadeRecord {
        ShadeRecord {
            hit_an_object: self.hit_an_object,
            local_hit_point: self.local_hit_point,
            normal: self.normal,
            color: self.color
        }
    }
}

#[cfg(test)]
mod test {
    use super::ShadeRecordBuilder;
    use color::RGBColor;
    use math::Vector3;

    #[test]
    fn test_builder() {
        let local_hit_point = Vector3::new(1.0, 2.0, 3.0);
        let color = RGBColor::new(0.1, 0.2, 0.3);
        let s = ShadeRecordBuilder::new()
                    .hit_an_object(true)
                    .local_hit_point(local_hit_point)
                    .normal(Vector3::new(0.0, 2.0, 0.0))
                    .color(color)
                    .finalize();

        assert_eq!(s.hit_an_object(), true);
        assert_eq!(s.local_hit_point(), local_hit_point);
        assert_eq!(s.normal(), Vector3::new(0.0, 1.0, 0.0));
        assert_eq!(s.color(), color);
    }
}
