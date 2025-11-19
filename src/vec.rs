use crate::util;
use std::ops::{Add, AddAssign, Div, Mul, MulAssign, Neg, Sub};
use std::simd::{StdFloat, f32x4, num::SimdFloat};
#[derive(Copy, Clone, Default, Debug)]
pub struct Vec3 {
    // We use a 4-lane SIMD vector.
    // e[0]=x, e[1]=y, e[2]=z, e[3]=0.0 (padding)
    e: f32x4,
}

impl Vec3 {
    pub fn new(x: f32, y: f32, z: f32) -> Vec3 {
        // We deliberately set the 4th element to 0.0
        Vec3 {
            e: f32x4::from_array([x, y, z, 0.0]),
        }
    }

    #[inline]
    fn from_simd(v: f32x4) -> Vec3 {
        Vec3 { e: v }
    }

    pub fn random() -> Vec3 {
        Vec3::new(
            util::random_double(),
            util::random_double(),
            util::random_double(),
        )
    }

    pub fn random_range(min: f32, max: f32) -> Vec3 {
        Vec3::new(
            util::random_double_range(min, max),
            util::random_double_range(min, max),
            util::random_double_range(min, max),
        )
    }

    #[inline]
    pub fn x(&self) -> f32 {
        self.e[0]
    }

    #[inline]
    pub fn y(&self) -> f32 {
        self.e[1]
    }

    #[inline]
    pub fn z(&self) -> f32 {
        self.e[2]
    }

    #[inline]
    pub fn length(&self) -> f32 {
        self.length_squared().sqrt()
    }

    #[inline]
    pub fn length_squared(&self) -> f32 {
        (self.e * self.e).reduce_sum()
    }

    #[inline]
    pub fn near_zero(&self) -> bool {
        const EPS: f32 = 1.0e-8;
        // We can check all lanes at once, but we must ensure the padding (0.0)
        // doesn't trigger a false positive if our logic was inverted.
        // Here, we just manually check x,y,z to ignore the 4th lane entirely.
        self.e[0].abs() < EPS && self.e[1].abs() < EPS && self.e[2].abs() < EPS
    }
}

pub type Point3 = Vec3;

// --- Operator Overloading ---

impl Neg for Vec3 {
    type Output = Vec3;
    #[inline]
    fn neg(self) -> Vec3 {
        Vec3::from_simd(-self.e)
    }
}

impl AddAssign for Vec3 {
    #[inline]
    fn add_assign(&mut self, v: Vec3) {
        self.e += v.e;
    }
}

impl MulAssign<f32> for Vec3 {
    #[inline]
    fn mul_assign(&mut self, t: f32) {
        self.e *= f32x4::splat(t);
    }
}

impl Add for Vec3 {
    type Output = Vec3;
    #[inline]
    fn add(self, v: Vec3) -> Vec3 {
        Vec3::from_simd(self.e + v.e)
    }
}

impl Sub for Vec3 {
    type Output = Vec3;
    #[inline]
    fn sub(self, v: Vec3) -> Vec3 {
        Vec3::from_simd(self.e - v.e)
    }
}

impl Mul for Vec3 {
    type Output = Vec3;
    #[inline]
    fn mul(self, v: Vec3) -> Vec3 {
        // Component-wise multiplication
        Vec3::from_simd(self.e * v.e)
    }
}

impl Mul<Vec3> for f32 {
    type Output = Vec3;
    #[inline]
    fn mul(self, v: Vec3) -> Vec3 {
        Vec3::from_simd(f32x4::splat(self) * v.e)
    }
}

impl Mul<f32> for Vec3 {
    type Output = Vec3;
    #[inline]
    fn mul(self, t: f32) -> Vec3 {
        Vec3::from_simd(self.e * f32x4::splat(t))
    }
}

impl Div<f32> for Vec3 {
    type Output = Vec3;
    #[inline]
    fn div(self, t: f32) -> Vec3 {
        // Multiplication by inverse is often faster than division,
        // but strict division is safer for precision.
        Vec3::from_simd(self.e / f32x4::splat(t))
    }
}

// --- Utility Functions ---

#[inline]
pub fn dot(u: Vec3, v: Vec3) -> f32 {
    (u.e * v.e).reduce_sum()
}

#[inline]
pub fn cross(u: Vec3, v: Vec3) -> Vec3 {
    // SIMD cross product usually requires shuffling lanes (swizzling).
    // For clarity, we can stick to the algebraic definition,
    // but we construct the final f32x4 array directly.
    let x = u.e[1] * v.e[2] - u.e[2] * v.e[1];
    let y = u.e[2] * v.e[0] - u.e[0] * v.e[2];
    let z = u.e[0] * v.e[1] - u.e[1] * v.e[0];
    Vec3::new(x, y, z)
}

#[inline]
pub fn unit_vector(v: Vec3) -> Vec3 {
    // We calculate length, splat it, and divide the vector by it
    // Note: If length is 0, this produces NaNs, just like the original code.
    Vec3::from_simd(v.e / f32x4::splat(v.length()))
}

pub fn random_in_unit_sphere() -> Vec3 {
    loop {
        let p = Vec3::random_range(-1.0, 1.0);
        if p.length_squared() < 1.0 {
            return p;
        }
    }
}

pub fn random_in_unit_disk() -> Vec3 {
    loop {
        let p = Vec3::new(
            util::random_double_range(-1.0, 1.0),
            util::random_double_range(-1.0, 1.0),
            0.0,
        );
        if p.length_squared() < 1.0 {
            return p;
        }
    }
}

#[inline]
pub fn reflect(v: Vec3, n: Vec3) -> Vec3 {
    v - n * 2.0 * dot(v, n)
}

pub fn refract(uv: Vec3, n: Vec3, etail_over_etat: f32) -> Vec3 {
    let cos_theta = f32::min(dot(-uv, n), 1.0);
    let r_out_perp = (uv + n * cos_theta) * etail_over_etat;
    let r_out_parallel = n * -f32::sqrt(f32::abs(1.0 - r_out_perp.length_squared()));
    r_out_perp + r_out_parallel
}

#[inline]
pub fn random_unit_vector() -> Vec3 {
    unit_vector(random_in_unit_sphere())
}
