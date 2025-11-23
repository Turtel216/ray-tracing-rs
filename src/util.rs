use rand::Rng;

#[inline]
pub fn random_double() -> f32 {
    rand::rng().random()
}

#[inline]
pub fn random_double_range(min: f32, max: f32) -> f32 {
    (max - min).mul_add(random_double(), min)
}

#[inline]
pub fn clamp(x: f32, min: f32, max: f32) -> f32 {
    if x < min {
        return min;
    }
    if x > max {
        return max;
    }
    x
}
