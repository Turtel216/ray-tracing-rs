//! Defines the materials that determine how rays interact with object surfaces.
//!
//! This module provides the `Material` trait, which defines a common interface
//! for all materials. When a ray hits an object, the object's material determines
//! if the ray is scattered, reflected, or refracted, and how its color is attenuated.
//!
//! Three common materials are implemented:
//! - `Lambertian`: A diffuse material that scatters light uniformly in all directions.
//! - `Metal`: A reflective material that simulates specular reflections, with a configurable "fuzziness".
//! - `Dielectric`: A transparent material, like glass or water, that refracts and reflects light.

use crate::color::Color;
use crate::hittable::HitRecord;
use crate::ray::Ray;
use crate::{util, vec};

pub struct ScatterRecord {
    pub attenuation: Color,
    pub scattered: Ray,
}

/// A trait for materials that define how light rays interact with a surface.
///
/// The `Send + Sync` bounds are required to allow materials to be shared
/// safely across multiple threads during rendering.
pub trait Material: Send + Sync {
    /// Computes how a ray is scattered upon hitting a material's surface.
    ///
    /// This function determines if a ray is scattered, absorbed, or reflected. If the ray
    /// is scattered, it calculates the new direction of the ray and the color attenuation.
    ///
    /// # Arguments
    ///
    /// * `r_in` - The incoming ray that hit the surface.
    /// * `rec` - The `HitRecord` containing information about the intersection point.
    /// * `attenuation` - A mutable `Color` reference that will be updated with the material's albedo (its intrinsic color).
    /// * `scattered` - A mutable `Ray` reference that will be updated with the new scattered ray.
    ///
    /// # Returns
    ///
    /// `true` if the ray is scattered and not absorbed. `false` if the ray is absorbed by the material.
    fn scatter(&self, r_in: &Ray, rec: &HitRecord) -> Option<ScatterRecord>;
}

/// A diffuse (matte) material that scatters light uniformly.
///
/// This material models a perfectly diffuse surface, where incoming light is scattered
/// with equal probability in all directions around the surface normal. This is a common
/// model for non-shiny surfaces like matte paint, plaster, or unpolished stone.
pub struct Lambertian {
    /// The intrinsic color of the material, also known as its albedo.
    albedo: Color,
}

impl Lambertian {
    /// Creates a new `Lambertian` material with a given albedo.
    pub const fn new(a: Color) -> Self {
        Self { albedo: a }
    }
}

impl Material for Lambertian {
    fn scatter(&self, _r_in: &Ray, rec: &HitRecord) -> Option<ScatterRecord> {
        // The scatter direction is a random unit vector added to the surface normal.
        // This produces a random direction within a hemisphere oriented around the normal.
        let mut scatter_direction = rec.normal + vec::random_unit_vector();

        // Catch a degenerate scatter direction where the random vector is exactly
        // opposite the normal, resulting in a zero vector.
        if scatter_direction.near_zero() {
            scatter_direction = rec.normal;
        }

        Some(ScatterRecord {
            attenuation: self.albedo,
            scattered: Ray::new(rec.p, scatter_direction),
        })
    }
}

/// A reflective, metallic material.
///
/// This material simulates a metal surface by reflecting rays. It includes a `fuzz`
/// parameter to control the blurriness of the reflection, from a perfect mirror (`fuzz`=0)
/// to a heavily frosted or brushed metal (`fuzz` > 0).
pub struct Metal {
    /// The color of the metal surface.
    albedo: Color,
    /// The "fuzziness" of the reflection, controlling its blurriness.
    /// Clamped to the range `[0.0, 1.0]`.
    fuzz: f32,
}

impl Metal {
    /// Creates a new `Metal` material.
    ///
    /// # Arguments
    ///
    /// * `a` - The color (albedo) of the metal.
    /// * `f` - The fuzziness of the reflection. Values greater than 1.0 are clamped to 1.0.
    pub const fn new(a: Color, f: f32) -> Self {
        Self {
            albedo: a,
            fuzz: f.clamp(0.0, 1.0),
        }
    }
}

impl Material for Metal {
    fn scatter(&self, r_in: &Ray, rec: &HitRecord) -> Option<ScatterRecord> {
        let reflected = vec::reflect(vec::unit_vector(r_in.direction()), rec.normal);
        let scattered = Ray::new(rec.p, reflected + self.fuzz * vec::random_in_unit_sphere());

        if vec::dot(scattered.direction(), rec.normal) > 0.0 {
            Some(ScatterRecord {
                attenuation: self.albedo,
                scattered,
            })
        } else {
            None
        }
    }
}

/// A transparent material that refracts and reflects light, like glass or water.
pub struct Dielectric {
    /// The index of refraction of the material.
    ir: f32,
}

impl Dielectric {
    /// Creates a new `Dielectric` material.
    ///
    /// # Arguments
    ///
    /// * `index_of_refraction` - The index of refraction (e.g., ~1.5 for glass, 1.33 for water).
    pub const fn new(index_of_refraction: f32) -> Self {
        Self {
            ir: index_of_refraction,
        }
    }

    /// Calculates reflectance using Schlick's approximation.
    ///
    /// This determines the probability of reflection versus refraction based on the
    /// angle of incidence (`cosine`) and the material's refractive index.
    fn reflectance(cosine: f32, ref_idx: f32) -> f32 {
        let mut r0 = (1.0 - ref_idx) / (1.0 + ref_idx);
        r0 = r0 * r0;
        (1.0 - r0).mul_add(f32::powf(1.0 - cosine, 5.0), r0)
    }
}

impl Material for Dielectric {
    fn scatter(&self, r_in: &Ray, rec: &HitRecord) -> Option<ScatterRecord> {
        // Determine the ratio of refractive indices based on whether the ray
        // is entering or exiting the material.
        let refraction_ratio = if rec.front_face {
            1.0 / self.ir
        } else {
            self.ir
        };

        let unit_direction = vec::unit_vector(r_in.direction());
        let cos_theta = f32::min(vec::dot(-unit_direction, rec.normal), 1.0);
        let sin_theta = f32::sqrt(1.0 - cos_theta * cos_theta);

        // Check for total internal reflection. This occurs when the ray cannot exit
        // a denser medium into a less dense one at a shallow angle.
        let cannot_refract = refraction_ratio * sin_theta > 1.0;

        // The final direction is either a reflection or a refraction.
        // The choice is made based on total internal reflection or, probabilistically,
        // using Schlick's approximation for reflectance.
        let direction = if cannot_refract
            || Self::reflectance(cos_theta, refraction_ratio) > util::random_double()
        {
            vec::reflect(unit_direction, rec.normal)
        } else {
            vec::refract(unit_direction, rec.normal, refraction_ratio)
        };

        Some(ScatterRecord {
            attenuation: Color::new(1.0, 1.0, 1.0),
            scattered: Ray::new(rec.p, direction),
        })
    }
}
