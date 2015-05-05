use std::ops::{Index, IndexMut};
use color::RGBColor;

#[derive(Copy, Clone, Debug)]
pub struct PixelPosition {
    x: usize,
    y: usize,
}

pub struct ViewPlane {
    width: usize,
    height: usize,
    pixel_size: f32,
    gamma: f32,
    pixels: Vec<RGBColor>,
}

impl ViewPlane {
    pub fn new(w: usize, h: usize, s: f32, g: f32, color: RGBColor) -> ViewPlane {
        let capacity = w * h;
        let mut pixels: Vec<RGBColor> = Vec::with_capacity(capacity);

        for _ in 0..capacity {
            pixels.push(color);
        }

        ViewPlane {
            width: w,
            height: h,
            pixel_size: s,
            gamma: g,
            pixels: pixels,
        }
    }

    pub fn width(&self) -> usize {
        self.width
    }

    pub fn height(&self) -> usize {
        self.height
    }

    pub fn pixel_size(&self) -> f32 {
        self.pixel_size
    }

    pub fn gamma(&self) -> f32 {
        self.gamma
    }
}

impl Index<PixelPosition> for ViewPlane {
    type Output = RGBColor;

    fn index<'a>(&'a self, _index: PixelPosition) -> &'a RGBColor {
        let i = _index.x + _index.y * self.width;
        &self.pixels[i]
    }
}

impl IndexMut<PixelPosition> for ViewPlane {
    fn index_mut<'a>(&'a mut self, _index: PixelPosition) -> &'a mut RGBColor {
        let i = _index.x + _index.y * self.width;
        &mut self.pixels[i]
    }
}

#[cfg(test)]
mod test {
    use color::RGBColor;
    use super::PixelPosition;
    use super::ViewPlane;

    #[test]
    fn test_index() {
        let color = RGBColor::new(0.0, 0.0, 0.0);
        let view_plane = ViewPlane::new(10, 10, 1.0, 1.0, color);
        assert_eq!(view_plane[PixelPosition { x: 0, y: 0 }], color);
    }

    #[test]
    fn test_index_mut() {
        let c1 = RGBColor::new(0.0, 0.0, 0.0);
        let c2 = RGBColor::new(0.5, 0.5, 0.5);
        let p1 = PixelPosition { x: 0, y: 0 };
        let p2 = PixelPosition { x: 1, y: 0 };
        let mut view_plane = ViewPlane::new(10, 10, 1.0, 1.0, c1);
        view_plane[p1] = c2;
        assert_eq!(view_plane[p1], c2);
        assert_eq!(view_plane[p2], c1);
    }
}
