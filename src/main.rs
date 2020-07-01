mod vec3;
use vec3::*;

use std::fs::File;
use std::io::prelude::*;

fn main() -> std::io::Result<()> {
    create_image()
}


fn create_image() -> std::io::Result<()> {
    const IMAGE_WIDTH: u16 = 256;
    const IMAGE_HEIGHT: u16 = 256;

    let mut file = File::create("image.ppm")?;

    file.write_all(b"P3\n")?;
    file.write_fmt(format_args!("{} {}\n",IMAGE_WIDTH, IMAGE_HEIGHT))?;
    file.write_fmt(format_args!("{}\n", color::COLOR_RANGE))?;

    for j in (0..IMAGE_HEIGHT).rev() {
        eprintln!("\rScanlines remaining:{}", j);
        for i in 0..IMAGE_WIDTH {
            let color: Color = Color::new(
                i as f32 / IMAGE_WIDTH as f32,
                j as f32 / IMAGE_HEIGHT as f32,
                0.25
                );
            color.write_to_file(&mut file)?;
        }
    }
    eprintln!("\rDone");


    Ok(())

}
