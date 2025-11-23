use crate::util;
use crate::vec::Vec3;

pub type Color = Vec3;

pub fn write_color(pixel_color: Color, samples_per_pixel: i32) -> Vec<u8> {
    let mut r = pixel_color.x();
    let mut g = pixel_color.y();
    let mut b = pixel_color.z();

    // Divide the color by the number of samples
    let scale = 1.0 / samples_per_pixel as f32;
    r = f32::sqrt(scale * r);
    g = f32::sqrt(scale * g);
    b = f32::sqrt(scale * b);

    format!(
        "{} {} {}\n",
        (256.0 * util::clamp(r, 0.0, 0.999)) as i32,
        (256.0 * util::clamp(g, 0.0, 0.999)) as i32,
        (256.0 * util::clamp(b, 0.0, 0.999)) as i32,
    )
    .into_bytes()
}
