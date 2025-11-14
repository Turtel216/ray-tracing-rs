use std::io::Write;

use crate::util;
use crate::vec::Vec3;

pub type Color = Vec3;

pub fn write_color(out: &mut impl Write, pixel_color: Color, samples_per_pixel: i32) {
    let mut r = pixel_color.x();
    let mut g = pixel_color.y();
    let mut b = pixel_color.z();

    // Divide the color by the number of samples
    let scale = 1.0 / samples_per_pixel as f32;
    r *= scale;
    g *= scale;
    b *= scale;

    writeln!(
        out,
        "{} {} {}",
        (256.0 * util::clamp(r, 0.0, 0.999)) as i32,
        (256.0 * util::clamp(g, 0.0, 0.999)) as i32,
        (256.0 * util::clamp(b, 0.0, 0.999)) as i32,
    )
    .expect("writing color");
}
