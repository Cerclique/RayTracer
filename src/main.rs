use geometry3d::point3d::Point3D;
use geometry3d::vector3d::Vector3D;

mod color;
mod ppm_writer;
mod ray;
mod sphere;

use color::Color;
use ppm_writer::PPMWriter;
use ray::Ray;

fn main() {
    // Image
    let aspect_ratio = 16.0 / 9.0;
    let image_width: u32 = 1280;
    let image_height = (image_width as f64 / aspect_ratio) as u32;

    // Camera
    let viewport_height = 2.0;
    let viewport_width = aspect_ratio * viewport_height;
    let focal_length = 1.0;

    let origin = Point3D::zeroes();
    let horizontal = Point3D::new(viewport_width, 0.0, 0.0);
    let vertical = Point3D::new(0.0, viewport_height, 0.0);
    let lower_left_corner = origin - horizontal * 0.5 - vertical * 0.5 - Point3D::new(0.0, 0.0, focal_length);

    // Compute pixels
    let mut pixels: Vec<Color> = Vec::new();
    pixels.reserve((image_width * image_height) as usize);

    for j in (0..image_height).rev() {
        for i in 0..image_width {
            let u = i as f64 / (image_width - 1) as f64;
            let v = j as f64 / (image_height - 1) as f64;

            let dir = Vector3D::from_point3d(lower_left_corner + horizontal * u + vertical * v - origin);
            pixels.push(Ray::new(origin, dir).color());
        }
    }

    PPMWriter::write_to_file("img.ppm", image_width, image_height, &pixels).unwrap();
}
