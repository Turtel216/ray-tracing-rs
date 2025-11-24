//! A module providing a `Sphere` geometric primitive for the ray tracer.
//!
//! This module defines the `Sphere` struct, which represents a sphere in 3D space,
//! and implements the `Hittable` trait for it. This allows rays to intersect
//! with spheres in the scene.

use std::sync::Arc;

use crate::hittable::{HitRecord, Hittable};
use crate::material::Material;
use crate::ray::Ray;
use crate::vec::{self, Point3};

/// Represents a sphere object in 3D space.
///
/// A `Sphere` is defined by its center point, a radius, and the material
/// that determines its appearance and how it interacts with light.
pub struct Sphere {
    /// The center point of the sphere in world coordinates.
    center: Point3,
    /// The radius of the sphere.
    radius: f32,
    /// A reference to the material of the sphere's surface.
    mat: Arc<dyn Material>,
}

impl Sphere {
    /// Creates a new `Sphere`.
    ///
    /// # Arguments
    ///
    /// * `cen` - The center `Point3` of the sphere.
    /// * `r` - The `f32` radius of the sphere.
    /// * `material` - An `Arc<dyn Material>` that defines the sphere's surface properties.
    ///
    /// # Returns
    ///
    /// A new `Sphere` instance.
    pub fn new(cen: Point3, r: f32, material: Arc<dyn Material>) -> Self {
        Self {
            center: cen,
            radius: r,
            mat: material,
        }
    }
}

impl Hittable for Sphere {
    /// Determines if a ray intersects with the sphere.
    ///
    /// This method calculates the intersection by solving the quadratic equation
    /// that results from the formula of a ray `P(t) = O + t*D` and the formula of a
    /// sphere `(P - C) · (P - C) = r^2`.
    ///
    /// # Arguments
    ///
    /// * `r` - The ray to test for intersection.
    /// * `t_min` - The minimum valid value for the ray parameter `t`.
    /// * `t_max` - The maximum valid value for the ray parameter `t`.
    ///
    /// # Returns
    ///
    /// `Some` `HitRecord` populated with intersection data if a hit occurs.
    ///  Otherwise, returns `None`.
    fn hit(&self, r: &Ray, t_min: f32, t_max: f32) -> Option<HitRecord> {
        let oc = r.origin() - self.center;
        let a = r.direction().length_squared();
        let half_b = vec::dot(oc, r.direction());
        let c = self.radius.mul_add(-self.radius, oc.length_squared());
        let discriminant = half_b.mul_add(half_b, -(a * c));

        // If the discriminant is negative, there are no real roots, so no intersection.
        if discriminant < 0.0 {
            return None;
        }

        let sqrt_d = discriminant.sqrt();

        let mut root = (-half_b - sqrt_d) / a;
        if root <= t_min || t_max <= root {
            // Smaller root is outside the range, check the larger root.
            root = (-half_b + sqrt_d) / a;
            if root <= t_min || t_max <= root {
                // Both roots are outside the range.
                return None;
            }
        }

        let mut rec = HitRecord {
            t: root,
            p: r.at(root),
            mat: self.mat.clone(),
            normal: Default::default(),
            front_face: Default::default(),
        };
        let outward_normal = (rec.p - self.center) / self.radius;
        rec.set_face_normal(r, outward_normal);
        Some(rec)
    }
}
