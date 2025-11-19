use std::ops::{Add, AddAssign, Div, Mul, MulAssign, Neg, Sub};
use std::simd::f32x4;
use std::simd::num::SimdFloat;

use crate::util;

#[derive(Copy, Clone, Default)]
pub struct Vec3 {
    e: [f32; 3],
}

impl Vec3 {
    pub fn new(x: f32, y: f32, z: f32) -> Vec3 {
        Vec3 { e: [x, y, z] }
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
        f32::sqrt(self.length_squared())
    }

    #[inline]
    pub fn length_squared(&self) -> f32 {
        self.e[0] * self.e[0] + self.e[1] * self.e[1] + self.e[2] * self.e[2]
    }

    #[inline]
    pub fn near_zero(&self) -> bool {
        const EPS: f32 = 1.0e-8;
        // Return true if the vector is close to zero in all dimensions
        self.e[0].abs() < EPS && self.e[1].abs() < EPS && self.e[2].abs() < EPS
    }
}

pub type Point3 = Vec3;

impl Neg for Vec3 {
    type Output = Vec3;

    #[inline]
    fn neg(self) -> Vec3 {
        Vec3::new(-self.x(), -self.y(), -self.z())
    }
}

impl AddAssign for Vec3 {
    #[inline]
    fn add_assign(&mut self, v: Vec3) {
        *self = *self + v;
    }
}

impl MulAssign<f32> for Vec3 {
    #[inline]
    fn mul_assign(&mut self, t: f32) {
        *self = *self * t;
    }
}

impl Add for Vec3 {
    type Output = Vec3;

    #[inline]
    fn add(self, v: Vec3) -> Vec3 {
        Vec3::new(self.x() + v.x(), self.y() + v.y(), self.z() + v.z())
    }
}

impl Sub for Vec3 {
    type Output = Vec3;

    #[inline]
    fn sub(self, v: Vec3) -> Vec3 {
        Vec3::new(self.x() - v.x(), self.y() - v.y(), self.z() - v.z())
    }
}

impl Mul for Vec3 {
    type Output = Vec3;

    #[inline]
    fn mul(self, v: Vec3) -> Vec3 {
        Vec3::new(self.x() * v.x(), self.y() * v.y(), self.z() * v.z())
    }
}

impl Mul<Vec3> for f32 {
    type Output = Vec3;

    #[inline]
    fn mul(self, v: Vec3) -> Vec3 {
        Vec3::new(self * v.x(), self * v.y(), self * v.z())
    }
}

impl Mul<f32> for Vec3 {
    type Output = Vec3;

    #[inline]
    fn mul(self, t: f32) -> Vec3 {
        Vec3::new(self.x() * t, self.y() * t, self.z() * t)
    }
}

impl Div<f32> for Vec3 {
    type Output = Vec3;

    #[inline]
    fn div(self, t: f32) -> Vec3 {
        Vec3::new(self.x() / t, self.y() / t, self.z() / t)
    }
}

#[inline]
pub fn dot(u: Vec3, v: Vec3) -> f32 {
    u.e[0] * v.e[0] + u.e[1] * v.e[1] + u.e[2] * v.e[2]
}

#[inline]
pub fn cross(u: Vec3, v: Vec3) -> Vec3 {
    Vec3::new(
        u.e[1] * v.e[2] - u.e[2] * v.e[1],
        u.e[2] * v.e[0] - u.e[0] * v.e[2],
        u.e[0] * v.e[1] - u.e[1] * v.e[0],
    )
}

#[inline]
pub fn unit_vector(v: Vec3) -> Vec3 {
    v / v.length()
}

#[inline]
fn dot_simd(u: Vec3, v: Vec3) -> f32 {
    let u_simd = f32x4::from_array([u.e[0], u.e[1], u.e[2], 0.0]);
    let v_simd = f32x4::from_array([v.e[0], v.e[1], v.e[2], 0.0]);

    let mul = u_simd * v_simd;
    mul.reduce_sum()
}

#[inline]
fn dot_scalar(u: Vec3, v: Vec3) -> f32 {
    u.e[0] * v.e[0] + u.e[1] * v.e[1] + u.e[2] * v.e[2]
}

pub fn random_in_unit_sphere() -> Vec3 {
    loop {
        let p = Vec3::random_range(-1.0, 1.0);
        if p.length_squared() >= 1.0 {
            continue;
        }
        return p;
    }
}

pub fn random_in_unit_disk() -> Vec3 {
    loop {
        let p = Vec3::new(
            util::random_double_range(-1.0, 1.0),
            util::random_double_range(-1.0, 1.0),
            0.0,
        );

        if p.length_squared() >= 1.0 {
            continue;
        }

        return p;
    }
}

#[inline]
pub fn reflect(v: Vec3, n: Vec3) -> Vec3 {
    v - 2.0 * dot(v, n) * n
}

pub fn refract(uv: Vec3, n: Vec3, etail_over_etat: f32) -> Vec3 {
    let cos_theta = f32::min(dot(-uv, n), 1.0);
    let r_out_perp = etail_over_etat * (uv + cos_theta * n);
    let r_out_parallel = -f32::sqrt(f32::abs(1.0 - r_out_perp.length_squared())) * n;

    r_out_perp + r_out_parallel
}

#[inline]
pub fn random_unit_vector() -> Vec3 {
    unit_vector(random_in_unit_sphere())
}
