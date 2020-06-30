use std::fs::File;
use std::io::prelude::*;

fn main() -> std::io::Result<()> {
    create_image()
}


fn create_image() -> std::io::Result<()> {
    const IMAGE_WIDTH: u16 = 256;
    const IMAGE_HEIGHT: u16 = 256;
    const COLOR_RANGE: u16 = 255;

    let mut file = File::create("image.ppm")?;

    file.write_all(b"P3\n")?;
    file.write_fmt(format_args!("{} {}\n",IMAGE_WIDTH, IMAGE_HEIGHT))?;
    file.write_fmt(format_args!("{}\n", COLOR_RANGE))?;

    for j in (0..IMAGE_HEIGHT).rev() {
        eprintln!("\rScanlines remaining:{}", j);
        for i in 0..IMAGE_WIDTH {
            let r = i as f32 / IMAGE_WIDTH as f32;
            let g = j as f32 / IMAGE_HEIGHT as f32;
            let b = 0.25;


            let ir = (r * IMAGE_WIDTH as f32) as u16;
            let ig = (g * IMAGE_HEIGHT as f32) as u16;
            let ib = (b * COLOR_RANGE as f32) as u16;

            file.write_fmt(format_args!("{} {} {}\n", ir, ig, ib))?;
        }
    }
    eprintln!("\rDone");


    Ok(())

}
