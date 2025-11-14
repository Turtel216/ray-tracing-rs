pub use std::f32::INFINITY;
pub use std::f32::consts::PI;

use rand::Rng;

// Utility functions

pub fn degrees_to_radians(degrees: f32) -> f32 {
    degrees * PI / 180.0
}

pub fn random_double() -> f32 {
    rand::rng().random()
}

pub fn random_double_range(min: f32, max: f32) -> f32 {
    min + (max - min) * random_double()
}

pub fn clamp(x: f32, min: f32, max: f32) -> f32 {
    if x < min {
        return min;
    }
    if x > max {
        return max;
    }
    x
}
