extern crate rand;
use rand::prelude::*;
pub const INFINITY: f32 = std::f32::INFINITY;
pub const PI: f32 = std::f32::consts::PI;


pub fn degree_to_radians(degrees: f32) -> f32 {
    degrees * PI / 180.0
}

pub fn random_double(min: f32, max: f32) -> f32 {
    rand::thread_rng().gen_range(min, max)
}

pub fn clamp(x: f32, min: f32, max: f32) -> f32 {
    if x < min { min }
    else if x > max { max }
    else { x }
}
