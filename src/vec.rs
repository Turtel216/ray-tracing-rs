//! A 3-dimensional vector module utilizing SIMD for performance.
//!
//! This module provides the `Vec3` struct, which represents a 3D vector or point.
//! It leverages `std::simd::f32x4` to perform vector operations in a more
//! efficient, parallelized manner. Common vector operations like addition,
//! subtraction, dot product, cross product, and normalization are provided.
//!
//! A type alias `Point3` is also available for geometric clarity when a `Vec3`
//! is used to represent a point in space.

use crate::util;
use std::ops::{Add, AddAssign, Div, Mul, MulAssign, Neg, Sub};
use std::simd::{f32x4, num::SimdFloat};

/// Represents a 3D vector, point, or color using SIMD for performance.
///
/// Internally, it uses an `f32x4` to store the `x`, `y`, and `z` components,
/// with the fourth element unused. This allows for hardware-accelerated
/// vector operations.
#[derive(Copy, Clone, Default, Debug)]
pub struct Vec3 {
    e: f32x4,
}

impl Vec3 {
    /// Creates a new `Vec3` with the given x, y, and z components.
    ///
    /// # Examples
    ///
    /// ```
    /// let v = Vec3::new(1.0, 2.0, 3.0);
    /// assert_eq!(v.x(), 1.0);
    /// assert_eq!(v.y(), 2.0);
    /// assert_eq!(v.z(), 3.0);
    /// ```
    pub const fn new(x: f32, y: f32, z: f32) -> Self {
        Self {
            e: f32x4::from_array([x, y, z, 0.0]),
        }
    }

    /// Creates a `Vec3` from an `f32x4` SIMD vector.
    #[inline]
    const fn from_simd(v: f32x4) -> Self {
        Self { e: v }
    }

    /// Creates a `Vec3` with random components between 0.0 and 1.0.
    pub fn random() -> Self {
        Self::new(
            util::random_double(),
            util::random_double(),
            util::random_double(),
        )
    }

    /// Creates a `Vec3` with random components within a specified range.
    ///
    /// # Arguments
    ///
    /// * `min` - The minimum value for each component.
    /// * `max` - The maximum value for each component.
    pub fn random_range(min: f32, max: f32) -> Self {
        Self::new(
            util::random_double_range(min, max),
            util::random_double_range(min, max),
            util::random_double_range(min, max),
        )
    }

    /// Returns the x-component of the vector.
    #[inline]
    pub fn x(&self) -> f32 {
        self.e[0]
    }

    /// Returns the y-component of the vector.
    #[inline]
    pub fn y(&self) -> f32 {
        self.e[1]
    }

    /// Returns the z-component of the vector.
    #[inline]
    pub fn z(&self) -> f32 {
        self.e[2]
    }

    /// Calculates the magnitude (length) of the vector.
    #[inline]
    pub fn length(&self) -> f32 {
        self.length_squared().sqrt()
    }

    /// Calculates the squared magnitude of the vector.
    ///
    /// This is often more efficient than `length()` as it avoids a square root.
    #[inline]
    pub fn length_squared(&self) -> f32 {
        (self.e * self.e).reduce_sum()
    }

    /// Checks if the vector is very close to the zero vector.
    ///
    /// This is useful for handling potential floating-point inaccuracies.
    #[inline]
    pub fn near_zero(&self) -> bool {
        const EPS: f32 = 1.0e-8;
        self.e[0].abs() < EPS && self.e[1].abs() < EPS && self.e[2].abs() < EPS
    }
}

/// A type alias for `Vec3` to represent a point in 3D space.
pub type Point3 = Vec3;

impl Neg for Vec3 {
    type Output = Self;
    /// Negates the vector, reversing its direction.
    ///
    /// # Examples
    ///
    /// ```
    /// let v = Vec3::new(1.0, -2.0, 3.0);
    /// let neg_v = -v;
    /// assert_eq!(neg_v.x(), -1.0);
    /// assert_eq!(neg_v.y(), 2.0);
    /// assert_eq!(neg_v.z(), -3.0);
    /// ```
    #[inline]
    fn neg(self) -> Self {
        Self::from_simd(-self.e)
    }
}

impl AddAssign for Vec3 {
    /// Adds another `Vec3` to this one in-place.
    #[inline]
    fn add_assign(&mut self, v: Self) {
        self.e += v.e;
    }
}

impl MulAssign<f32> for Vec3 {
    /// Multiplies the vector by a scalar in-place.
    #[inline]
    fn mul_assign(&mut self, t: f32) {
        self.e *= f32x4::splat(t);
    }
}

impl Add for Vec3 {
    type Output = Self;
    /// Performs vector addition.
    #[inline]
    fn add(self, v: Self) -> Self {
        Self::from_simd(self.e + v.e)
    }
}

impl Sub for Vec3 {
    type Output = Self;
    /// Performs vector subtraction.
    #[inline]
    fn sub(self, v: Self) -> Self {
        Self::from_simd(self.e - v.e)
    }
}

impl Mul for Vec3 {
    type Output = Self;
    /// Performs element-wise multiplication (Hadamard product).
    #[inline]
    fn mul(self, v: Self) -> Self {
        Self::from_simd(self.e * v.e)
    }
}

impl Mul<Vec3> for f32 {
    type Output = Vec3;
    /// Performs scalar multiplication: `scalar * vector`.
    #[inline]
    fn mul(self, v: Vec3) -> Vec3 {
        Vec3::from_simd(f32x4::splat(self) * v.e)
    }
}

impl Mul<f32> for Vec3 {
    type Output = Self;
    /// Performs scalar multiplication: `vector * scalar`.
    #[inline]
    fn mul(self, t: f32) -> Self {
        Self::from_simd(self.e * f32x4::splat(t))
    }
}

impl Div<f32> for Vec3 {
    type Output = Self;
    /// Performs scalar division.
    #[inline]
    fn div(self, t: f32) -> Self {
        Self::from_simd(self.e / f32x4::splat(t))
    }
}

/// Computes the dot product of two vectors.
#[inline]
pub fn dot(u: Vec3, v: Vec3) -> f32 {
    (u.e * v.e).reduce_sum()
}

/// Computes the cross product of two vectors.
#[inline]
pub fn cross(u: Vec3, v: Vec3) -> Vec3 {
    let x = u.e[1].mul_add(v.e[2], -(u.e[2] * v.e[1]));
    let y = u.e[2].mul_add(v.e[0], -(u.e[0] * v.e[2]));
    let z = u.e[0].mul_add(v.e[1], -(u.e[1] * v.e[0]));
    Vec3::new(x, y, z)
}

/// Computes the unit vector (normalized vector) of `v`.
///
/// Returns a new vector pointing in the same direction as `v` but with a length of 1.
///
/// # Panics
///
/// If the length of `v` is zero, this function will produce NaNs.
#[inline]
pub fn unit_vector(v: Vec3) -> Vec3 {
    // We calculate length, splat it, and divide the vector by it
    // Note: If length is 0, this produces NaNs, just like the original code.
    Vec3::from_simd(v.e / f32x4::splat(v.length()))
}

/// Generates a random `Vec3` point inside a unit sphere.
///
/// This is achieved by rejection sampling: generating random points in a
/// cube and rejecting those that fall outside the sphere.
pub fn random_in_unit_sphere() -> Vec3 {
    loop {
        let p = Vec3::random_range(-1.0, 1.0);
        if p.length_squared() < 1.0 {
            return p;
        }
    }
}

/// Generates a random `Vec3` point inside a unit disk in the XY plane.
///
/// This is achieved by rejection sampling in 2D. The z-component is always zero.
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

/// Calculates the reflection of a vector `v` about a surface normal `n`.
///
/// # Arguments
///
/// * `v` - The incoming vector.
/// * `n` - The surface normal vector. Assumed to be a unit vector.
#[inline]
pub fn reflect(v: Vec3, n: Vec3) -> Vec3 {
    v - n * 2.0 * dot(v, n)
}

/// Computes the refraction of a vector `uv` using Snell's Law.
///
/// # Arguments
///
/// * `uv` - The incoming unit vector.
/// * `n` - The surface normal unit vector.
/// * `etail_over_etat` - The ratio of refractive indices (η / η').
pub fn refract(uv: Vec3, n: Vec3, etail_over_etat: f32) -> Vec3 {
    let cos_theta = f32::min(dot(-uv, n), 1.0);
    let r_out_perp = (uv + n * cos_theta) * etail_over_etat;
    let r_out_parallel = n * -f32::sqrt(f32::abs(1.0 - r_out_perp.length_squared()));
    r_out_perp + r_out_parallel
}

/// Generates a random unit vector.
///
/// This is done by picking a random point within the unit sphere and normalizing it.
#[inline]
pub fn random_unit_vector() -> Vec3 {
    unit_vector(random_in_unit_sphere())
}
