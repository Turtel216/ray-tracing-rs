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
    /// * `rec` - A mutable `HitRecord` that will be populated with intersection data if a hit occurs.
    ///
    /// # Returns
    ///
    /// `true` if the ray hits the sphere within the interval `[t_min, t_max]`, and `rec`
    /// is updated with the hit details. Otherwise, returns `false`.
    fn hit(&self, r: &Ray, t_min: f32, t_max: f32, rec: &mut HitRecord) -> bool {
        // Deriving the quadratic equation for ray-sphere intersection:
        // (ray_origin + t*ray_direction - sphere_center) · (ray_origin + t*ray_direction - sphere_center) = radius^2
        // Let oc = ray_origin - sphere_center.
        // (t*ray_direction + oc) · (t*ray_direction + oc) = radius^2
        // t^2*(D·D) + 2*t*(D·oc) + (oc·oc) - r^2 = 0
        // Where D is ray_direction.
        // This is a quadratic equation at^2 + bt + c = 0, with:
        // a = D·D
        // b = 2*(D·oc)
        // c = (oc·oc) - r^2
        // We use a simplified form with half_b = D·oc.
        let oc = r.origin() - self.center;
        let a = r.direction().length_squared();
        let half_b = vec::dot(oc, r.direction());
        let c = self.radius.mul_add(-self.radius, oc.length_squared());
        let discriminant = half_b.mul_add(half_b, -(a * c));

        // If the discriminant is negative, there are no real roots, so no intersection.
        if discriminant < 0.0 {
            return false;
        }

        let sqrt_d = discriminant.sqrt();

        // Find the nearest root that lies in the acceptable range [t_min, t_max].
        // Check the smaller root first.
        let mut root = (-half_b - sqrt_d) / a;
        if root <= t_min || t_max <= root {
            // Smaller root is outside the range, check the larger root.
            root = (-half_b + sqrt_d) / a;
            if root <= t_min || t_max <= root {
                // Both roots are outside the range.
                return false;
            }
        }

        // A valid intersection was found. Populate the HitRecord.
        rec.t = root;
        rec.p = r.at(rec.t);
        let outward_normal = (rec.p - self.center) / self.radius;
        rec.set_face_normal(r, outward_normal);
        rec.mat = Some(self.mat.clone());

        true
    }
}
