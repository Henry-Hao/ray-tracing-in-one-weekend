mod vec3;
mod ray;
mod object;
mod rtweekend;
mod camera;

use vec3::*;
use object::*;
use camera::*;
use rtweekend::*;

use std::fs::File;
use std::io::prelude::*;

use std::rc::Rc;

use indicatif::ProgressBar;

const ASPECT_RATIO: f32 = 16.0 / 9.0;
const IMAGE_WIDTH: u16 = 384;
const IMAGE_HEIGHT: u16 = (IMAGE_WIDTH as f32 / ASPECT_RATIO) as u16;
const SAMPLES_PER_PIXEL: u16 = 100;
const DEPTH: u16 = 10;

fn main() -> std::io::Result<()> {
    create_image()
}


fn create_image() -> std::io::Result<()> {

    let mut file = File::create("image.ppm")?;

    file.write_fmt(format_args!("P3\n{} {}\n{}\n",IMAGE_WIDTH, IMAGE_HEIGHT, color::COLOR_RANGE))?;

    let mut world: HittableList = HittableList ::new();
    world.add(Rc::new(Sphere::new(Point3::new(0.0, 0.0, -1.0), 0.5)));
    world.add(Rc::new(Sphere::new(Point3::new(0.0, -100.5, -1.0), 100.0)));

    let cam: Camera = Camera::new();

    let pb: ProgressBar = ProgressBar::new(IMAGE_HEIGHT as u64);
    for j in (0..IMAGE_HEIGHT).rev() {
        pb.inc(1);
        for i in 0..IMAGE_WIDTH {
            let color = (0..SAMPLES_PER_PIXEL).fold(Color::new(0.0, 0.0, 0.0), |acc, _| {
                let u = (i as f32 + random_double()) / (IMAGE_WIDTH - 1) as f32;
                let v = (j as f32 + random_double()) / (IMAGE_HEIGHT - 1) as f32;
                acc + cam.get_ray(u, v).ray_color(&world, DEPTH)
            });
            color.write_to_file(&mut file, SAMPLES_PER_PIXEL)?;
        }
    }
    pb.finish_with_message("Done");


    Ok(())

}
