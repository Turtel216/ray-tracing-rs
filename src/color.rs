//! A module for color representation and manipulation.
//!
//! This module defines the `Color` type, an alias for `Vec3`, to represent
//! RGB color values. It also provides utility functions for processing these
//! _color!s for output, such as applying gamma correction and converting them
//! into a format suitable for image files like PPM.

use crate::util;
use crate::vec::Vec3;

/// A type alias for `Vec3` to represent an RGB color.
///
/// The `x`, `y`, and `z` components of the `Vec3` correspond to the
/// red, green, and blue channels of the color, respectively. These values are
/// typically represented as floating-point numbers in the range `[0.0, 1.0]`.
pub type Color = Vec3;

/// Converts a floating-point `Color` value into a byte vector for PPM image output.
///
/// This function performs several key steps:
/// 1.  **Averaging:** It scales the raw accumulated color by the number of samples
///     taken for that pixel to get the average color.
/// 2.  **Gamma Correction:** It applies a gamma-2 correction (by taking the square root)
///     to the color components. This is crucial for accurate color reproduction on most displays.
/// 3.  **Scaling and Clamping:** It maps the floating-point color components from the `[0.0, 1.0]`
///     range to the integer range `[0, 255]`.
///
/// The final output is a `Vec<u8>` containing a string in the format "R G B\n".
///
/// # Arguments
///
/// * `pixel_color` - The accumulated `Color` value from all samples for a single pixel.
/// * `samples_per_pixel` - The total number of samples that contributed to `pixel_color`.
///
/// # Returns
///
/// A `Vec<u8>` representing the final, formatted RGB values for a single pixel.
///
/// # Examples
///
/// ```
/// use crate::color::{Color, write_color};
///
/// // A sample color (e.g., a medium gray) accumulated over 100 samples.
/// // In a real scenario, this would be the sum of 100 different color values.
/// let accumulated_color = Color::new(50.0, 50.0, 50.0);
/// let samples = 100;
///
/// let byte_color = write_color(accumulated_color, samples);
///
/// // The expected process:
/// // 1. Average: 50.0 / 100 = 0.5
/// // 2. Gamma correct: sqrt(0.5) ≈ 0.707
/// // 3. Scale and clamp: 256 * 0.707 ≈ 181
/// // 4. Format: "181 181 181\n"
/// assert_eq!(byte_color, b"181 181 181\n");
/// ```
pub fn write_color(pixel_color: Color, samples_per_pixel: i32) -> Vec<u8> {
    let mut r = pixel_color.x();
    let mut g = pixel_color.y();
    let mut b = pixel_color.z();

    // Divide the color by the number of samples to get the average color.
    // This is part of antialiasing.
    let scale = 1.0 / samples_per_pixel as f32;
    r *= scale;
    g *= scale;
    b *= scale;

    // Apply gamma correction (gamma = 2.0).
    // This transforms the color from linear space to gamma space.
    r = r.sqrt();
    g = g.sqrt();
    b = b.sqrt();

    // Format the color components into a string "R G B\n" and convert to bytes.
    // The values are scaled to [0, 255] and clamped to ensure they are within a valid range.
    format!(
        "{} {} {}\n",
        (256.0 * util::clamp(r, 0.0, 0.999)) as u8,
        (256.0 * util::clamp(g, 0.0, 0.999)) as u8,
        (256.0 * util::clamp(b, 0.0, 0.999)) as u8,
    )
    .into_bytes()
}
