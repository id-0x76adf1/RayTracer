use std::cmp::{PartialEq, Eq};
use std::ops::{Add, Mul, Div};

#[derive(Copy, Clone, Debug)]
pub struct RGBColor {
    r: f64,
    g: f64,
    b: f64,
}

impl RGBColor {
    pub fn new(_r: f64, _g: f64, _b: f64) -> RGBColor {
        assert!(0.0 <= _r && _r <= 1.0);
        assert!(0.0 <= _g && _g <= 1.0);
        assert!(0.0 <= _b && _b <= 1.0);
        RGBColor { r: _r, g: _g, b: _b }
    }

    pub fn power(&self, p: f64) -> RGBColor {
        RGBColor {
            r: clamp(self.r.powf(p), 0.0, 1.0),
            g: clamp(self.g.powf(p), 0.0, 1.0),
            b: clamp(self.b.powf(p), 0.0, 1.0),
        }
    }

    pub fn r(&self) -> f64 {
        self.r
    }

    pub fn g(&self) -> f64 {
        self.b
    }

    pub fn b(&self) -> f64 {
        self.b
    }
}

fn clamp(value: f64, min: f64, max: f64) -> f64 {
    if value < min {
        return min
    }
    if value > max {
        return max
    }
    value
}

impl Add for RGBColor {
    type Output = RGBColor;

    fn add(self, other: RGBColor) -> RGBColor {
        RGBColor {
            r: clamp(self.r + other.r, 0.0, 1.0),
            g: clamp(self.g + other.g, 0.0, 1.0),
            b: clamp(self.b + other.b, 0.0, 1.0),
        }
    }
}

impl Mul<f64> for RGBColor {
    type Output = RGBColor;

    fn mul(self, other: f64) -> RGBColor {
        RGBColor {
            r: clamp(self.r * other, 0.0, 1.0),
            g: clamp(self.g * other, 0.0, 1.0),
            b: clamp(self.b * other, 0.0, 1.0),
        }
    }
}

impl Mul<RGBColor> for f64 {
    type Output = RGBColor;

    fn mul(self, other: RGBColor) -> RGBColor {
        RGBColor {
            r: clamp(self * other.r, 0.0, 1.0),
            g: clamp(self * other.g, 0.0, 1.0),
            b: clamp(self * other.b, 0.0, 1.0),
        }
    }
}

impl Mul for RGBColor {
    type Output = RGBColor;

    fn mul(self, other: RGBColor) -> RGBColor {
        RGBColor {
            r: self.r * other.r,
            g: self.g * other.g,
            b: self.b * other.b,
        }
    }
}

impl Div<f64> for RGBColor {
    type Output = RGBColor;

    fn div(self, other: f64) -> RGBColor {
        RGBColor {
            r: clamp(self.r / other, 0.0, 1.0),
            g: clamp(self.g / other, 0.0, 1.0),
            b: clamp(self.b / other, 0.0, 1.0),
        }
    }
}

impl PartialEq for RGBColor {
    fn eq(&self, other: &RGBColor) -> bool {
        self.r == other.r && self.g == other.g && self.b == self.b
    }
}

impl Eq for RGBColor { }

#[cfg(test)]
mod test {
    use super::clamp;
    use super::RGBColor;

    #[test]
    fn test_clamp() {
        assert_eq!(clamp(2.0, 1.0, 3.0), 2.0);
        assert_eq!(clamp(0.0, 1.0, 3.0), 1.0);
        assert_eq!(clamp(4.0, 1.0, 3.0), 3.0);
    }

    #[test]
    fn test_add() {
        let c1 = RGBColor::new(0.0, 0.0, 0.0);
        let c2 = RGBColor::new(0.5, 0.5, 0.5);
        let c3 = RGBColor::new(0.6, 0.7, 0.8);

        assert_eq!(c1 + c2, c2);
        assert_eq!(c1 + c3, c3);
        assert_eq!(c2 + c3, RGBColor::new(1.0, 1.0, 1.0));
    }

    #[test]
    fn test_mul() {
        let c1 = RGBColor::new(0.0, 0.0, 0.0);
        let c2 = RGBColor::new(0.5, 0.5, 0.5);
        let c3 = RGBColor::new(0.6, 0.7, 0.8);
        let c4 = RGBColor::new(1.0, 1.0, 1.0);
        let c5 = RGBColor::new(0.25, 0.25, 0.25);

        assert_eq!(c1 * c2, c1);
        assert_eq!(c2 * c4, c2);
        assert_eq!(c2 * c3, RGBColor::new(0.3, 0.35, 0.4));
        assert_eq!(c3 * c4, c3);

        assert_eq!(c1 * 2.0, c1);
        assert_eq!(2.0 * c1, c1);

        assert_eq!(c2 * 2.0, c4);
        assert_eq!(2.0 * c2, c4);

        assert_eq!(c2 * 0.5, c5);
        assert_eq!(0.5 * c2, c5);

        assert_eq!(c3 * 2.0, c4);
        assert_eq!(2.0 * c3, c4);
    }

    #[test]
    fn test_div() {
        let c = RGBColor::new(0.5, 0.5, 0.5);
        assert_eq!(c / 2.0, RGBColor::new(0.25, 0.25, 0.25));
        assert_eq!(c / 0.001, RGBColor::new(1.0, 1.0, 1.0));
    }

    #[test]
    fn test_power() {
        let c = RGBColor::new(0.5, 0.6, 0.7);
        assert_eq!(c.power(2.0), RGBColor::new(0.25, 0.36, 0.49));
    }
}
