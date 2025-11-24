//! Defines the core components for ray-object intersection in the ray tracer.
//!
//! This module provides the `Hittable` trait, which is a fundamental abstraction for any
//! object in the scene that can be intersected by a ray (e.g., spheres, planes). It also
//! provides the `HitRecord` struct, which stores detailed information about such an
//! intersection, including the point of contact, surface normal, and material.

use std::sync::Arc;

use crate::material::Material;
use crate::ray::Ray;
use crate::vec::{self, Point3, Vec3};

/// A struct to store information about a ray-object intersection.
///
/// When a ray hits a `Hittable` object, a `HitRecord` is populated with details
/// about the hit, such as the intersection point, the surface normal, and the
/// material of the object.
#[derive(Clone)]
pub struct HitRecord {
    /// The point in 3D space where the intersection occurred.
    pub p: Point3,
    /// The surface normal vector at the point of intersection.
    /// This normal always points against the incident ray.
    pub normal: Vec3,
    /// A reference to the material of the object that was hit.
    pub mat: Arc<dyn Material>,
    /// The parameter `t` along the ray where the intersection occurred, such that `p = ray.origin() + t * ray.direction()`.
    pub t: f32,
    /// A boolean indicating whether the ray hit the front face of the surface.
    /// `true` if the ray strikes the object from the outside, `false` otherwise.
    pub front_face: bool,
}

impl HitRecord {
    /// Sets the surface normal based on the ray's direction.
    ///
    /// This method ensures the normal vector in the `HitRecord` always points
    /// against the incoming ray. It determines whether the ray hit the front or
    /// back face of the surface and flips the `outward_normal` if necessary.
    ///
    /// # Arguments
    ///
    /// * `r` - The ray that intersected the surface.
    /// * `outward_normal` - The geometric normal of the surface, assuming it points "outward".
    ///   This vector is assumed to have unit length.
    pub fn set_face_normal(&mut self, r: &Ray, outward_normal: Vec3) {
        // The dot product tells us if the ray and outward_normal are in opposite directions.
        // If dot < 0, the ray is hitting the front face.
        self.front_face = vec::dot(r.direction(), outward_normal) < 0.0;
        self.normal = if self.front_face {
            outward_normal
        } else {
            // Ray is inside the object; reverse the normal so it points against the ray.
            -outward_normal
        };
    }
}

/// A trait for objects that can be intersected by a ray.
///
/// Any object in the scene, such as a sphere or a collection of objects,
/// must implement this trait to be rendered. The `Send + Sync` bounds are
/// required to allow for safe multi-threaded rendering.
pub trait Hittable: Send + Sync {
    /// Determines if a ray intersects with the object.
    ///
    /// # Arguments
    ///
    /// * `ray` - The ray to test for intersection.
    /// * `t_min` - The minimum valid value for the ray parameter `t`.
    /// * `t_max` - The maximum valid value for the ray parameter `t`.
    /// * `rec` - A mutable `HitRecord` to be populated with intersection data if a hit occurs.
    ///
    /// # Returns
    ///
    /// `Some` `HitRecord` populated with intersection data if a hit occurs.
    ///  Otherwise, returns `None`.
    fn hit(&self, ray: &Ray, t_min: f32, t_max: f32) -> Option<HitRecord>;
}
