use geometry3d::point3d::Point3D;

mod camera;
mod color;
mod ppm_writer;
mod ray;
mod sphere;
mod hitrecord;
mod hittable_trait;
mod hittable_list;
mod random;

use color::Color;
use ppm_writer::PPMWriter;
use sphere::Sphere;
use hittable_list::HittableList;
use camera::Camera;
use random::randf;

fn main() {
    // Image
    let aspect_ratio = 16.0 / 9.0;
    let image_width: u32 = 1280;
    let image_height = (image_width as f64 / aspect_ratio) as u32;
    let sample_per_pixels = 1;

    // World
    let mut world = HittableList::new();
    world.add(Sphere::new(Point3D::new(0.0, 0.0, -1.0), 0.5));
    world.add(Sphere::new(Point3D::new(0.0, -100.5, -1.0), 100.0));

    // Camera
    let cam = Camera::new();

    // Compute pixels
    let mut pixels: Vec<Color> = Vec::new();
    pixels.reserve((image_width * image_height) as usize);

    for j in (0..image_height).rev() {
        println!("{}", j);
        for i in 0..image_width {
            let mut pixel_color = Color::zeroes();
            for _ in 0..sample_per_pixels {
                let u = ((i as f64) + randf()) / (image_width - 1) as f64;
                let v = ((j as f64) + randf()) / (image_height - 1) as f64;
                let r = cam.get_ray(u, v);
                pixel_color += r.color(&world);
            }
            pixels.push(pixel_color);
        }
    }

    PPMWriter::write_to_file("img.ppm", image_width, image_height, &pixels, sample_per_pixels).unwrap();
}
