use std::ops::{Add, AddAssign, Div, Mul, MulAssign, Neg, Sub};
use std::simd::f32x4;
use std::simd::num::SimdFloat;

#[derive(Copy, Clone, Default)]
pub struct Vec3 {
    e: [f32; 3],
}

impl Vec3 {
    pub fn new(x: f32, y: f32, z: f32) -> Vec3 {
        Vec3 { e: [x, y, z] }
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
}

pub type Point3 = Vec3;

impl Neg for Vec3 {
    type Output = Vec3;

    fn neg(self) -> Vec3 {
        Vec3::new(-self.x(), -self.y(), -self.z())
    }
}

impl AddAssign for Vec3 {
    fn add_assign(&mut self, v: Vec3) {
        *self = *self + v;
    }
}

impl MulAssign<f32> for Vec3 {
    fn mul_assign(&mut self, t: f32) {
        *self = *self * t;
    }
}

impl Add for Vec3 {
    type Output = Vec3;

    fn add(self, v: Vec3) -> Vec3 {
        Vec3::new(self.x() + v.x(), self.y() + v.y(), self.z() + v.z())
    }
}

impl Sub for Vec3 {
    type Output = Vec3;

    fn sub(self, v: Vec3) -> Vec3 {
        Vec3::new(self.x() - v.x(), self.y() - v.y(), self.z() - v.z())
    }
}

impl Mul for Vec3 {
    type Output = Vec3;

    fn mul(self, v: Vec3) -> Vec3 {
        Vec3::new(self.x() * v.x(), self.y() * v.y(), self.z() * v.z())
    }
}

impl Mul<Vec3> for f32 {
    type Output = Vec3;

    fn mul(self, v: Vec3) -> Vec3 {
        Vec3::new(self * v.x(), self * v.y(), self * v.z())
    }
}

impl Mul<f32> for Vec3 {
    type Output = Vec3;

    fn mul(self, t: f32) -> Vec3 {
        Vec3::new(self.x() * t, self.y() * t, self.z() * t)
    }
}

impl Div<f32> for Vec3 {
    type Output = Vec3;

    fn div(self, t: f32) -> Vec3 {
        Vec3::new(self.x() / t, self.y() / t, self.z() / t)
    }
}

pub fn dot(u: Vec3, v: Vec3) -> f32 {
    #[cfg(target_arch = "x86_64")]
    {
        if is_x86_feature_detected!("sse4.1") {
            dot_simd(u, v)
        } else {
            dot_scalar(u, v)
        }
    }

    #[cfg(not(target_arch = "x86_64"))]
    {
        dot_scalar(u, v)
    }
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
