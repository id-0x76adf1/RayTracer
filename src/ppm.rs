use std::fs::File;
use std::io::prelude::*;
use color::RGBColor;

pub fn write(name: &str, width: usize, height: usize, data: &Vec<RGBColor>) {
    let mut name_with_ext = String::new();
    name_with_ext.push_str(name);
    name_with_ext.push_str(".ppm");

    let mut f = File::create(name_with_ext)
        .ok()
        .expect("create file failed");

    f.write(format!("P3\n{} {}\n255\n", width, height).as_bytes())
        .ok()
        .expect("write header failed");

    for r in (0..height).rev() {
        for c in 0..width {
            let i = r * width + c;
            let color = data[i];
            let r = (color.r() * 255.0) as u8;
            let g = (color.g() * 255.0) as u8;
            let b = (color.b() * 255.0) as u8;
            f.write(format!("{:>3} {:>3} {:>3}  ", r, g, b).as_bytes()).
                ok().
                expect("write color failed");
        }
        f.write(format!("\n").as_bytes()).
            ok().
            expect("write line failed");
    }
}
