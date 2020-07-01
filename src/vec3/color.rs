use super::*;
use std::fs::File;
use std::io::prelude::*;

pub const COLOR_RANGE: u16 = 256;

impl Color {
    pub fn write_to_file(&self, file: &mut File) -> std::io::Result<()> {
        file.write_fmt(format_args!("{} {} {}\n", 
                                    (self.x * (COLOR_RANGE - 1) as f32) as u16,
                                    (self.y * (COLOR_RANGE - 1) as f32) as u16,
                                    (self.z * (COLOR_RANGE - 1) as f32) as u16)
                       )?;
        Ok(())
    }
}

