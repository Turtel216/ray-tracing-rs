//! A module for managing a collection of `Hittable` objects.
//!
//! This module provides `HittableList`, a container that can hold multiple objects
//! that implement the `Hittable` trait. The list itself also implements `Hittable`,
//! allowing a group of objects (representing a scene) to be treated as a single
//! entity for intersection tests.

use crate::hittable::{HitRecord, Hittable};
use crate::ray::Ray;

/// A container for a list of `Hittable` objects, representing a scene.
///
/// This struct holds a vector of `Box<dyn Hittable>`, allowing it to store
/// different types of objects (e.g., `Sphere`, `Plane`) in a single collection.
/// When a ray intersection test is performed on the list, it checks against all
/// objects it contains and returns the closest hit.
#[derive(Default)]
pub struct HittableList {
    /// A vector of boxed objects that can be intersected by rays.
    objects: Vec<Box<dyn Hittable>>,
}

impl HittableList {
    /// Creates a new, empty `HittableList`.
    ///
    /// # Returns
    ///
    /// An empty `HittableList` instance.
    pub fn new() -> Self {
        Self::default()
    }

    /// Adds a `Hittable` object to the list.
    ///
    /// # Arguments
    ///
    /// * `object` - A `Box<dyn Hittable>` representing the object to be added to the scene.
    ///   The object is moved into the list.
    pub fn add(&mut self, object: Box<dyn Hittable>) {
        self.objects.push(object);
    }
}

impl Hittable for HittableList {
    /// Determines if a ray intersects with any object in the list.
    ///
    /// This method iterates through all objects in the `HittableList` and performs
    /// an intersection test for each one. It keeps track of the closest intersection
    /// found so far and updates the `HitRecord` accordingly. This ensures that only
    /// the hit nearest to the ray's origin within the `[t_min, t_max]` range is returned.
    ///
    /// # Arguments
    ///
    /// * `ray` - The ray to test for intersection.
    /// * `t_min` - The minimum valid value for the ray parameter `t`.
    /// * `t_max` - The maximum valid value for the ray parameter `t`.
    /// * `rec` - A mutable `HitRecord` to be populated with the closest intersection data if a hit occurs.
    ///
    /// # Returns
    ///
    /// `true` if the ray hits any object in the list within the valid range, and `rec` is
    /// updated with the details of the closest hit. Otherwise, returns `false`.
    fn hit(&self, ray: &Ray, t_min: f32, t_max: f32, rec: &mut HitRecord) -> bool {
        let mut temp_rec = HitRecord::new();
        let mut hit_anything = false;
        let mut closest_so_far = t_max;

        // Iterate through all objects in the list to find the closest hit.
        for object in &self.objects {
            // Check for a hit on the current object. The `closest_so_far` value is used
            // as the new `t_max` for each subsequent test. This ensures we only find
            // hits that are closer than any previous ones.
            if object.hit(ray, t_min, closest_so_far, &mut temp_rec) {
                hit_anything = true;
                closest_so_far = temp_rec.t;
                // Since we found a closer hit, we update the main HitRecord.
                // We clone temp_rec because it will be reused in the next iteration.
                *rec = temp_rec.clone();
            }
        }

        hit_anything
    }
}
