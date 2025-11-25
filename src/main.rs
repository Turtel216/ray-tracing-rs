#![feature(portable_simd)]
#![allow(clippy::cast_precision_loss)]
#![allow(clippy::many_single_char_names)]
#![allow(clippy::cast_sign_loss)]
#![allow(clippy::cast_possible_truncation)]

mod camera;
mod color;
mod hittable;
mod hittable_list;
mod material;
mod ray;
mod sphere;
mod util;
mod vec;

use core::f32;
use std::{fs, sync::Arc};

use indicatif::ProgressBar;
use rayon::prelude::*;

use crate::{
    camera::Camera,
    color::Color,
    hittable::Hittable,
    hittable_list::HittableList,
    material::Dielectric,
    material::{Lambertian, Metal},
    ray::Ray,
    sphere::Sphere,
    vec::{Point3, Vec3},
};

#[allow(dead_code)]
fn random_scene() -> HittableList {
    let mut world = HittableList::new();

    let ground_material = Arc::new(Lambertian::new(Color::new(0.5, 0.5, 0.5)));
    add_to_world!(world, ground_material, 0.0, -1000.0, 0.0, 1000.0);

    for a in -11..11 {
        for b in -11..11 {
            let choose_mat = util::random_double();
            let center = Point3::new(
                0.9f32.mul_add(util::random_double(), a as f32),
                0.2,
                0.9f32.mul_add(util::random_double(), b as f32),
            );

            if (center - Point3::new(4.0, 0.2, 0.0)).length() > 0.9 {
                if choose_mat < 0.8 {
                    // Diffuse
                    let albedo = Color::random() * Color::random();
                    let sphere_material = Arc::new(Lambertian::new(albedo));
                    add_to_world!(world, sphere_material, center, 0.2);
                } else if choose_mat < 0.95 {
                    // Metal
                    let albedo = Color::random_range(0.5, 1.0);
                    let fuzz = util::random_double_range(0.0, 0.5);
                    let sphere_material = Arc::new(Metal::new(albedo, fuzz));
                    add_to_world!(world, sphere_material, center, 0.02);
                } else {
                    // Glass
                    let sphere_material = Arc::new(Dielectric::new(1.5));
                    add_to_world!(world, sphere_material, center, 0.2);
                }
            }
        }
    }

    let material1 = Arc::new(Dielectric::new(1.5));
    add_to_world!(world, material1, 0.0, 1.0, 0.0, 1.0);

    let material2 = Arc::new(Lambertian::new(Color::new(0.4, 0.2, 0.1)));
    add_to_world!(world, material2, -4.0, 1.0, 0.0, 1.0);

    let material3 = Arc::new(Metal::new(Color::new(0.7, 0.6, 0.5), 0.0));
    add_to_world!(world, material3, 4.0, 1.0, 0.0, 1.0);

    world
}

/// Generates the scene used in the perfomance benchmarks
#[allow(dead_code)]
fn benchmark_scene() -> HittableList {
    let mut world = HittableList::new();

    let ground_material = Arc::new(Lambertian::new(Color::new(0.5, 0.5, 0.5)));
    add_to_world!(world, ground_material, 0.0, -1000.0, 0.0, 1000.0);

    let material1 = Arc::new(Dielectric::new(1.5));
    add_to_world!(world, material1, 0.0, 1.0, 0.0, 1.0);

    let material2 = Arc::new(Lambertian::new(Color::new(0.4, 0.2, 0.1)));
    world.add(Box::new(Sphere::new(
        Point3::new(-4.0, 1.0, 0.0),
        1.0,
        material2,
    )));

    let material3 = Arc::new(Metal::new(Color::new(0.7, 0.6, 0.5), 0.0));
    world.add(Box::new(Sphere::new(
        Point3::new(4.0, 1.0, 0.0),
        1.0,
        material3,
    )));

    world
}

fn ray_color(r: &Ray, world: &dyn Hittable, depth: i32) -> Color {
    // Make sure there is no stack overflow
    if depth <= 0 {
        return Color::new(0.0, 0.0, 0.0);
    }

    if let Some(hit_rec) = world.hit(r, 0.001, f32::INFINITY) {
        if let Some(scatter_rec) = hit_rec.mat.scatter(r, &hit_rec) {
            return scatter_rec.attenuation * ray_color(&scatter_rec.scattered, world, depth - 1);
        }
        return Color::new(0.0, 0.0, 0.0);
    }

    let unit_direction = vec::unit_vector(r.direction());
    let t = 0.5 * (unit_direction.y() + 1.0);
    (1.0 - t) * Color::new(1.0, 1.0, 1.0) + t * Color::new(0.5, 0.7, 1.0)
}

fn main() {
    // Image

    const ASPECT_RATIO: f32 = 3.0 / 2.0;
    const IMAGE_WIDTH: i32 = 1200;
    const IMAGE_HEIGHT: i32 = (IMAGE_WIDTH as f32 / ASPECT_RATIO) as i32;
    const SAMPLES_PER_PIXEL: i32 = 100;
    const MAX_DEPTH: i32 = 50;

    // World
    let world = random_scene();

    // Camera
    let from = Point3::new(13.0, 2.0, 3.0);
    let lookat = Point3::new(0.0, 0.0, 0.0);
    let vup = Point3::new(0.0, 1.0, 0.0);
    let dist_to_focus = 10.0;
    let aperture = 0.1;

    let cam = Camera::new(
        from,
        lookat,
        vup,
        20.0,
        ASPECT_RATIO,
        aperture,
        dist_to_focus,
    );

    let viewport_height = 2.0;
    let viewport_width = ASPECT_RATIO * viewport_height;
    let focal_length = 1.0;

    let origin = Point3::new(0.0, 0.0, 0.0);
    let horizontal = Vec3::new(viewport_width, 0.0, 0.0);
    let vertical = Vec3::new(0.0, viewport_height, 0.0);
    let _lower_left_corner =
        origin - horizontal / 2.0 - vertical / 2.0 - Vec3::new(0.0, 0.0, focal_length);

    // Render

    let total_scanlines = 0..IMAGE_HEIGHT;
    let bar = ProgressBar::new(total_scanlines.len() as u64);
    let mut ppm_bytes: Vec<u8> = Vec::new();

    ppm_bytes.append(&mut format!("P3\n{IMAGE_WIDTH} {IMAGE_HEIGHT}\n255\n").into_bytes());

    println!("Image Rendering");

    for j in total_scanlines.rev() {
        bar.inc(1);
        let pixel_colors: Vec<_> = (0..IMAGE_WIDTH)
            .into_par_iter()
            .map(|i| {
                let mut pixel_color = Color::new(0.0, 0.0, 0.0);
                for _ in 0..SAMPLES_PER_PIXEL {
                    let u = ((i as f32) + util::random_double()) / (IMAGE_WIDTH - 1) as f32;
                    let v = ((j as f32) + util::random_double()) / (IMAGE_HEIGHT - 1) as f32;
                    let r = cam.get_ray(u, v);
                    pixel_color += ray_color(&r, &world, MAX_DEPTH);
                }
                pixel_color
            })
            .collect();
        for pixel_color in pixel_colors {
            ppm_bytes.append(&mut color::write_color(pixel_color, SAMPLES_PER_PIXEL));
        }
    }

    bar.finish();
    println!("\n\nFinished Rendering");

    println!("Saving Image to image.png");
    fs::write("image.ppm", ppm_bytes).expect("Could not write to file image.ppm");
    println!("Image saved");
}
