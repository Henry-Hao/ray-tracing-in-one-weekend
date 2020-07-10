mod camera;
mod material;
mod object;
mod ray;
mod rtweekend;
mod vec3;

use camera::*;
use material::*;
use object::*;
use rtweekend::*;
use vec3::*;

use std::fs::File;
use std::io::prelude::*;

use std::cell::RefCell;
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

fn random_scene() -> HittableList {
    let mut world: HittableList = HittableList::new();
    let ground_material: Rc<RefCell<dyn Material>> =
        Rc::new(RefCell::new(Lambertian::new(Color::new(0.5, 0.5, 0.5))));

    world.add(Rc::new(Sphere::new(
        Point3::new(0.0, -1000.0, 0.0),
        1000.0,
        ground_material,
    )));

    for a in -11..11 {
        for b in -11..11 {
            let choose_mat: f32 = random_double();
            let center: Point3 = Point3::new(
                a as f32 + 0.9 * random_double(),
                0.2,
                b as f32 + 0.9 * random_double(),
            );

            if (center - Point3::new(4.0, 0.2, 0.0)).length() > 0.9 {
                world.add(Rc::new(Sphere::new(
                    center,
                    0.2,
                    if choose_mat < 0.8 {
                        Rc::new(RefCell::new(Lambertian::new(
                            Color::random() * Color::random(),
                        )))
                    } else if choose_mat < 0.95 {
                        Rc::new(RefCell::new(Metal::new(
                            Color::random_with_range(0.5, 1.0),
                            random_double_with_range(0.0, 0.5),
                        )))
                    } else {
                        Rc::new(RefCell::new(Dielectric::new(1.5)))
                    },
                )));
            }
        }
    }

    let material1: Rc<RefCell<dyn Material>> = Rc::new(RefCell::new(Dielectric::new(1.5)));
    world.add(Rc::new(Sphere::new(
        Point3::new(0.0, 1.0, 0.0),
        1.0,
        material1,
    )));

    let material2: Rc<RefCell<dyn Material>> =
        Rc::new(RefCell::new(Lambertian::new(Color::new(0.4, 0.2, 0.1))));
    world.add(Rc::new(Sphere::new(
        Point3::new(-4.0, 1.0, 0.0),
        1.0,
        material2,
    )));

    let material3: Rc<RefCell<dyn Material>> =
        Rc::new(RefCell::new(Metal::new(Color::new(0.7, 0.6, 0.5), 0.0)));
    world.add(Rc::new(Sphere::new(
        Point3::new(4.0, 1.0, 0.0),
        1.0,
        material3,
    )));

    return world;
}

fn create_image() -> std::io::Result<()> {
    let mut file = File::create("image.ppm")?;

    file.write_fmt(format_args!(
        "P3\n{} {}\n{}\n",
        IMAGE_WIDTH,
        IMAGE_HEIGHT,
        color::COLOR_RANGE
    ))?;

    let world: HittableList = random_scene();

    let lookfrom: Point3 = Point3::new(13.0, 2.0, 3.0);
    let lookat: Point3 = Point3::new(0.0, 0.0, 0.0);
    let vup: Vec3 = Vec3::new(0.0, 1.0, 0.0);

    let aperture: f32 = 0.1;
    let dist_to_focus: f32 = 10.0;

    let cam: Camera = Camera::new(
        lookfrom,
        lookat,
        vup,
        20.0,
        ASPECT_RATIO,
        aperture,
        dist_to_focus,
    );
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
