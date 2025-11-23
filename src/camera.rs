//! A module for camera simulation in a ray tracing environment.
//!
//! This module provides the `Camera` struct, which is responsible for generating
//! rays that are cast into the scene. It simulates a thin-lens camera model,
//! allowing for features like depth of field and configurable field of view.

use crate::ray::Ray;
use crate::vec;
use crate::vec::{Point3, Vec3};

/// Represents a virtual camera for generating rays in the scene.
///
/// The camera is defined by its position, orientation, and lens properties.
/// It creates rays that originate from the camera's lens and travel through
/// a virtual viewport, simulating a pinhole or thin-lens camera model.
pub struct Camera {
    /// The origin point of the camera, representing the center of the lens.
    origin: Point3,
    /// The bottom-left corner of the virtual viewport in world space.
    lower_left_corner: Point3,
    /// A vector representing the width of the viewport.
    horizontal: Vec3,
    /// A vector representing the height of the viewport.
    vertical: Vec3,
    /// The orthonormal basis vector for the camera's horizontal orientation.
    u: Vec3,
    /// The orthonormal basis vector for the camera's vertical orientation.
    v: Vec3,
    /// The radius of the camera's thin lens, used for simulating depth of field.
    lens_radius: f32,
}

impl Camera {
    /// Creates and configures a new `Camera`.
    ///
    /// This constructor sets up the camera's geometry based on its position,
    /// the point it's looking at, and its field of view. It also configures
    /// the lens for depth-of-field effects.
    ///
    /// # Arguments
    ///
    /// * `from` - The position of the camera in world space (`Point3`).
    /// * `lookat` - The point in world space that the camera is looking at (`Point3`).
    /// * `vup` - The "view up" vector, defining the camera's upward orientation (e.g., `Vec3::new(0.0, 1.0, 0.0)`).
    /// * `vfov` - The vertical field-of-view, in degrees.
    /// * `aspect_ratio` - The aspect ratio of the viewport (width / height).
    /// * `aperture` - The diameter of the camera's aperture. A larger aperture creates a more pronounced depth-of-field effect.
    /// * `focus_dist` - The distance from the camera to the focal plane. Objects at this distance will be perfectly in focus.
    ///
    /// # Returns
    ///
    /// A new `Camera` instance configured with the specified parameters.
    pub fn new(
        from: Point3,
        lookat: Point3,
        vup: Vec3,
        vfov: f32, // vertical field-of-view in degrees
        aspect_ratio: f32,
        aperture: f32,
        focus_dist: f32,
    ) -> Self {
        // Calculate viewport dimensions from the vertical FOV
        let theta = vfov.to_radians();
        let h = f32::tan(theta / 2.0);
        let viewport_height = 2.0 * h;
        let viewport_width = aspect_ratio * viewport_height;

        // Create an orthonormal basis (u, v, w) for the camera's orientation
        // w is the backward-pointing vector (from lookat to camera)
        let w = vec::unit_vector(from - lookat);
        // u is the side-pointing vector, found via cross product of up vector and w
        let u = vec::unit_vector(vec::cross(vup, w));
        // v is the new "up" vector, perpendicular to w and u
        let v = vec::cross(w, u);

        let origin = from;
        // The horizontal and vertical vectors span the focal plane
        let horizontal = focus_dist * viewport_width * u;
        let vertical = focus_dist * viewport_height * v;
        // The lower-left corner of the focal plane
        let lower_left_corner = origin - horizontal / 2.0 - vertical / 2.0 - focus_dist * w;

        let lens_radius = aperture / 2.0;

        Self {
            origin,
            lower_left_corner,
            horizontal,
            vertical,
            u,
            v,
            lens_radius,
        }
    }

    /// Generates a ray from the camera through a point on its virtual viewport.
    ///
    /// This function simulates a thin lens by originating the ray from a random
    /// point on the camera's aperture. The direction is set to a point on the
    /// focal plane, creating a depth-of-field effect.
    ///
    /// # Arguments
    ///
    /// * `s` - The horizontal coordinate on the viewport, in the range `[0, 1]`.
    /// * `t` - The vertical coordinate on the viewport, in the range `[0, 1]`.
    ///
    /// # Returns
    ///
    /// A `Ray` that originates from the camera's lens and travels into the scene.
    pub fn get_ray(&self, s: f32, t: f32) -> Ray {
        // Find a random point on the lens disk to simulate a thin lens
        let rd = self.lens_radius * vec::random_in_unit_disk();
        // Calculate the offset on the lens from the camera's origin
        let offset = self.u * rd.x() + self.v * rd.y();

        Ray::new(
            // Ray origin: camera origin + lens offset
            self.origin + offset,
            // Ray direction: vector from the lens point to the focal plane point
            self.lower_left_corner + s * self.horizontal + t * self.vertical - self.origin - offset,
        )
    }
}
