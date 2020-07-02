use super::*;
use std::fs::File;
use std::io::prelude::*;
use crate::rtweekend::*;

pub const COLOR_RANGE: u16 = 256;

impl Color {
    pub fn write_to_file(&self, file: &mut File, sample_per_pixel: u16) -> std::io::Result<()> {
        let scale = 1.0 / sample_per_pixel as f32;
        let r = (self.x() * scale).sqrt();
        let g = (self.y() * scale).sqrt();
        let b = (self.z() * scale).sqrt();


        file.write_fmt(format_args!("{} {} {}\n", 
                                    (COLOR_RANGE as f32 * clamp(r, 0.0, 1.0)) as u16,
                                    (COLOR_RANGE as f32 * clamp(g, 0.0, 1.0)) as u16,
                                    (COLOR_RANGE as f32 * clamp(b, 0.0, 1.0)) as u16)
                       )?;
        Ok(())
    }
}

