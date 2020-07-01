mod vec3;
use vec3::*;
use vec3::ray::*;

use std::fs::File;
use std::io::prelude::*;

const ASPECT_RATIO: f32 = 16.0 / 9.0;
const IMAGE_WIDTH: u16 = 384;
const IMAGE_HEIGHT: u16 = (IMAGE_WIDTH as f32 / ASPECT_RATIO) as u16;

fn main() -> std::io::Result<()> {
    create_image()
}


fn create_image() -> std::io::Result<()> {

    let mut file = File::create("image.ppm")?;

    file.write_fmt(format_args!("P3\n{} {}\n{}\n",IMAGE_WIDTH, IMAGE_HEIGHT, color::COLOR_RANGE))?;
    let viewport_height: f32 = 2.0;
    let viewport_width: f32= ASPECT_RATIO  * viewport_height;
    let focal_length: f32 = 1.0;

    let origin: Point3 = Point3::new(0f32, 0f32, 0f32);
    let horizonal: Point3 = Point3::new(viewport_width, 0f32, 0f32);
    let vertical: Point3 = Point3::new(0f32, viewport_height, 0f32);
    let lower_left_corner: Point3 = origin - horizonal/viewport_width - vertical/viewport_height - Point3::new(0f32, 0f32, focal_length);

    for j in (0..IMAGE_HEIGHT).rev() {
        eprintln!("\rScanlines remaining:{}", j);
        for i in 0..IMAGE_WIDTH {
            let u = i as f32 / (IMAGE_WIDTH - 1) as f32;
            let v = j as f32 / (IMAGE_HEIGHT - 1) as f32;
            let ray: Ray = Ray::new(origin, lower_left_corner + u*horizonal + v*vertical - origin);
            let color: Color = ray.gradient_color();
            color.write_to_file(&mut file)?;
        }
    }
    eprintln!("\rDone");


    Ok(())

}
