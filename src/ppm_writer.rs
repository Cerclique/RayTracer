use std::fs::File;
use std::io::{BufWriter, Write};

use crate::color::Color;

pub struct PPMWriter {}

impl PPMWriter {
    pub fn write_to_file(path: &str, img_w: u32, img_h: u32, buf: &[Color], ) -> Result<(), std::io::Error> {
        let f = File::create(path)?;
        let mut f = BufWriter::new(f);

        f.write_all(format!("P3\n{} {}\n255\n", img_w, img_h).as_bytes())?;

        for j in 0..img_h {
            for i in 0..img_w {
                let pixel_color = buf[(i + j * img_w) as usize];

                let ir = (255.999 * pixel_color.r()) as u8;
                let ig = (255.999 * pixel_color.g()) as u8;
                let ib = (255.999 * pixel_color.b()) as u8;

                f.write_all(format!("{} {} {}\n", ir, ig, ib).as_bytes())?;
            }
        }

        Ok(())
    }
}
