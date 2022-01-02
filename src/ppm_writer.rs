use std::fs::File;
use std::io::{BufWriter, Write};

use crate::color::Color;

pub struct PPMWriter {}

impl PPMWriter {
    pub fn write_to_file(path: &str, img_w: u32, img_h: u32, buf: &[Color], sample_per_pixels: u32) -> Result<(), std::io::Error> {
        let f = File::create(path)?;
        let mut f = BufWriter::new(f);

        f.write_all(format!("P3\n{} {}\n255\n", img_w, img_h).as_bytes())?;

        for j in 0..img_h {
            for i in 0..img_w {
                let mut pixel_color = buf[(i + j * img_w) as usize];

                let scale = 1.0 / sample_per_pixels as f64;
                pixel_color *= scale;

                let ir = (256.0 * clamp(pixel_color.r(), 0.0, 0.999)) as u8;
                let ig = (256.0 * clamp(pixel_color.g(), 0.0, 0.999)) as u8;
                let ib = (256.0 * clamp(pixel_color.b(), 0.0, 0.999)) as u8;
            f.write_all(format!("{} {} {}\n", ir, ig, ib).as_bytes())?;
            }
        }

        Ok(())
    }
}

fn clamp(value: f64, min: f64, max: f64) -> f64 {
    if value < min {
        return min;
    }
    else if value > max {
        return max
    }

    value
}
