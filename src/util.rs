//! A collection of utility functions for math and random number generation.
//!
//! This module provides simple, commonly used helper functions for tasks
//! such as generating random floating-point numbers and clamping values
//! within a specified range.
use rand::Rng;

/// Generates a random `f32` value in the range `[0.0, 1.0)`.
///
/// This function is a convenient wrapper around `rand::thread_rng().gen()`.
///
/// # Examples
///
/// ```
/// let random_val = random_double();
/// assert!(random_val >= 0.0 && random_val < 1.0);
/// ```
#[inline]
pub fn random_double() -> f32 {
    rand::rng().random()
}

/// Generates a random `f32` value within a specified range `[min, max)`.
///
/// # Arguments
///
/// * `min` - The inclusive lower bound of the random number.
/// * `max` - The exclusive upper bound of the random number.
///
/// # Examples
///
/// ```
/// let random_val = random_double_range(5.0, 10.0);
/// assert!(random_val >= 5.0 && random_val < 10.0);
/// ```
#[inline]
pub fn random_double_range(min: f32, max: f32) -> f32 {
    // This uses the formula: min + (max - min) * random_double()
    // It is written this way to potentially leverage the Fused Multiply-Add (FMA) CPU instruction
    // for a slight performance improvement, though compiler optimizations may vary.
    (max - min).mul_add(random_double(), min)
}

/// Clamps a floating-point number `x` to be within the range `[min, max]`.
///
/// If `x` is less than `min`, it returns `min`. If `x` is greater than `max`,
/// it returns `max`. Otherwise, it returns `x`.
///
/// Note: This function provides the same functionality as the built-in `f32::clamp`.
///
/// # Arguments
///
/// * `x` - The value to clamp.
/// * `min` - The inclusive lower bound.
/// * `max` - The inclusive upper bound.
///
/// # Examples
///
/// ```
/// assert_eq!(clamp(5.0, 0.0, 10.0), 5.0);
/// assert_eq!(clamp(-5.0, 0.0, 10.0), 0.0);
/// assert_eq!(clamp(15.0, 0.0, 10.0), 10.0);
/// ```
#[inline]
pub fn clamp(x: f32, min: f32, max: f32) -> f32 {
    if x < min {
        min
    } else if x > max {
        max
    } else {
        x
    }
}
