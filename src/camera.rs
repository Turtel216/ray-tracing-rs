use crate::ray::Ray;
use crate::vec::{Point3, Vec3};
use crate::{util, vec};

pub struct Camera {
    origin: Point3,
    lower_left_corner: Point3,
    horizontal: Vec3,
    vertical: Vec3,
}

impl Camera {
    pub fn new(from: Point3, lookat: Point3, vup: Vec3, vfov: f32, aspect_ratio: f32) -> Camera {
        let theta = util::degrees_to_radians(vfov);
        let h = f32::tan(theta / 2.0);
        let viewport_height = 2.0 * h;
        let viewport_width = aspect_ratio * viewport_height;

        let w = vec::unit_vector(from - lookat);
        let u = vec::unit_vector(vec::cross(vup, w));
        let v = vec::cross(w, u);

        let origin = from;
        let horizontal = viewport_width * u;
        let vertical = viewport_height * v;
        let lower_left_corner = origin - horizontal / 2.0 - vertical / 2.0 - w;

        Camera {
            origin,
            lower_left_corner,
            horizontal,
            vertical,
        }
    }

    pub fn get_ray(&self, s: f32, t: f32) -> Ray {
        Ray::new(
            self.origin,
            self.lower_left_corner + s * self.horizontal + t * self.vertical - self.origin,
        )
    }
}
