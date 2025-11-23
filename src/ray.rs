//! Defines a ray, a fundamental component in a ray tracer.
//!
//! A ray can be thought of as a function `P(t) = A + t*B`, where `A` is the
//! ray's origin and `B` is its direction. The parameter `t` is a real number
//! that defines a point along the ray's path. Positive values of `t` correspond
//! to points in front of the origin, along the ray's direction.

use crate::vec::{Point3, Vec3};

/// Represents a 3D ray with an origin and a direction.
///
/// This struct is used to model the path of light in the scene. It consists
/// of a starting point (`orig`) and a vector (`dir`) indicating its direction
/// of travel.
#[derive(Default, Debug, Copy, Clone)]
pub struct Ray {
    /// The starting point of the ray in 3D space.
    orig: Point3,
    /// The direction vector of the ray. This is not required to be a unit vector.
    dir: Vec3,
}

impl Ray {
    /// Creates a new `Ray` with a specified origin and direction.
    ///
    /// # Arguments
    ///
    /// * `origin` - The starting point of the ray (`Point3`).
    /// * `direction` - The direction vector of the ray (`Vec3`).
    ///
    /// # Examples
    ///
    /// ```
    /// use crate::vec::{Point3, Vec3};
    /// use crate::ray::Ray;
    ///
    /// let origin = Point3::new(0.0, 0.0, 0.0);
    /// let direction = Vec3::new(1.0, 0.0, 0.0);
    /// let r = Ray::new(origin, direction);
    ///
    /// assert_eq!(r.origin(), origin);
    /// assert_eq!(r.direction(), direction);
    /// ```
    #[inline]
    pub const fn new(origin: Point3, direction: Vec3) -> Self {
        Self {
            orig: origin,
            dir: direction,
        }
    }

    /// Returns the origin point of the ray.
    #[inline]
    pub const fn origin(&self) -> Point3 {
        self.orig
    }

    /// Returns the direction vector of the ray.
    #[inline]
    pub const fn direction(&self) -> Vec3 {
        self.dir
    }

    /// Calculates a point along the ray at a given distance `t`.
    ///
    /// The point is calculated using the formula `P(t) = origin + t * direction`.
    ///
    /// # Arguments
    ///
    /// * `t` - The distance parameter along the ray.
    ///
    /// # Returns
    ///
    /// A `Point3` representing the location at distance `t` along the ray.
    ///
    /// # Examples
    ///
    /// ```
    /// use crate::vec::{Point3, Vec3};
    /// use crate::ray::Ray;
    ///
    /// let origin = Point3::new(1.0, 2.0, 3.0);
    /// let direction = Vec3::new(10.0, 20.0, 30.0);
    /// let r = Ray::new(origin, direction);
    ///
    /// let p = r.at(0.5);
    /// assert_eq!(p.x(), 1.0 + 0.5 * 10.0); // 6.0
    /// assert_eq!(p.y(), 2.0 + 0.5 * 20.0); // 12.0
    /// assert_eq!(p.z(), 3.0 + 0.5 * 30.0); // 18.0
    /// ```
    #[inline]
    pub fn at(&self, t: f32) -> Point3 {
        self.orig + self.dir * t
    }
}
