#![feature(const_generics)]

mod aabb;
mod bvh;
mod camera;
mod image;
mod materials;
mod objects;
mod onb;
mod ray;
mod textures;
mod util;
mod vector3;

// Crates
use image::{gen_ppm, Pixel};
use rand::prelude::*;
use rayon::prelude::*;

use std::env;
use std::f32;
use std::path::Path;
use std::time;

use crate::objects::{HitRecord, Hittable, Scene};
use crate::ray::Ray;
use crate::vector3::Vector;

#[allow(unused_imports)]
use util::{one_by_one, progress_bar, sixteen_by_nine, two_by_one};

const IMG_WIDTH: usize = 300;
const IMG_HEIGHT: usize = one_by_one(IMG_WIDTH);
const SAMPLES: usize = 150;

const MAX_RECURSIVE_DEPTH: usize = 50;
const T_MIN: f32 = 0.005;

const PROG_BAR_WIDTH: usize = 80;

fn main() -> Result<(), String> {
    let start_time = time::Instant::now();

    let mut image = Vec::with_capacity(IMG_HEIGHT);

    for y in 0..IMG_HEIGHT {
        image.push(Vec::with_capacity(IMG_WIDTH));

        for x in 0..IMG_WIDTH {
            image[y].push(Pixel {
                r: 0,
                g: 0,
                b: 0,
                x,
                y: IMG_HEIGHT - 1 - y,
            });
        }
    }

    let scene_file = env::args()
        .nth(1)
        .unwrap_or_else(|| String::from("./scenes/1.json"));

    let output_file = env::args()
        .nth(2)
        .unwrap_or_else(|| String::from("./out/image.ppm"));

    let path = Path::new(&scene_file);
    let scene = Scene::from_json(path, IMG_WIDTH as f32 / IMG_HEIGHT as f32)?;

    println!(
        "Scene loaded from {}, rendering {} x {} @ {} samples ({} rays)",
        path.file_name().and_then(|p| p.to_str()).unwrap(),
        IMG_WIDTH,
        IMG_HEIGHT,
        SAMPLES,
        IMG_HEIGHT * IMG_WIDTH * SAMPLES,
    );

    let mut completed_rows = 0;
    image.iter_mut().for_each(|row| {
        progress_bar(completed_rows, IMG_HEIGHT, PROG_BAR_WIDTH, "Rendering");

        row.par_iter_mut().for_each(|pixel| {
            let mut curr_pixel = Vector::zeros();

            for _ in 0..SAMPLES {
                let u = (pixel.x as f32 + random::<f32>()) / IMG_WIDTH as f32;
                let v = (pixel.y as f32 + random::<f32>()) / IMG_HEIGHT as f32;

                let r = scene.camera.get_ray(u, v);

                curr_pixel += color(r, &scene, 0);
            }

            curr_pixel /= SAMPLES as f32;

            // Treat overflowed pixels as max value
            if curr_pixel.x > 1.0 {
                curr_pixel.x = 1.0;
            }
            if curr_pixel.y > 1.0 {
                curr_pixel.y = 1.0;
            }
            if curr_pixel.z > 1.0 {
                curr_pixel.z = 1.0;
            }

            let r = 255.99 * curr_pixel.x.sqrt();
            let g = 255.99 * curr_pixel.y.sqrt();
            let b = 255.99 * curr_pixel.z.sqrt();

            pixel.r = r as u8;
            pixel.g = g as u8;
            pixel.b = b as u8;
        });

        completed_rows += 1;
    });

    println!("\nCompleted rendering in {:#?}", start_time.elapsed());
    gen_ppm(image, output_file)
}

fn color(r: Ray, scene: &Scene, depth: usize) -> Vector {
    if let Some((hit_record, material)) = scene.hit(r, T_MIN, f32::MAX) {
        let emitted = material.emitted(r, hit_record);

        if depth < MAX_RECURSIVE_DEPTH {
            if let Some((attenuation, scattered, pdf)) =
                material.scatter(r, hit_record)
            {
                let on_light = Vector::new(
                    213.0 + random::<f32>() * (343.0 - 213.0),
                    554.0,
                    227.0 + random::<f32>() * (332.0 - 227.0),
                );

                let to_light = on_light - hit_record.p;

                let dist_squared = to_light.length_squared();
                let to_light = to_light.normalize();

                if Vector::dot(to_light, hit_record.normal) < 0.0 {
                    return emitted;
                }

                let light_area = (343.0 - 213.0) * (332.0 - 227.0);
                let light_cosine = f32::abs(to_light.y);

                if light_cosine < 0.000001 {
                    return emitted;
                }

                let pdf = dist_squared / (light_cosine * light_area);
                let scattered = Ray::new(hit_record.p, to_light, r.time());

                return emitted
                    + attenuation
                        * material.scattering_pdf(r, hit_record, scattered)
                        * color(scattered, scene, depth + 1)
                        / pdf;
            }
        }

        emitted
    } else {
        Vector::zeros()
    }
}
