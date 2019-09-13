#![feature(const_generics)]

mod aabb;
mod bvh;
mod camera;
mod image;
mod materials;
mod objects;
mod ray;
mod textures;
mod util;
mod vector3;

use image::{gen_ppm, Pixel};
use rand::prelude::*;
use rayon::prelude::*;

use std::env;
use std::f32;
use std::path::Path;
use std::time;

use objects::{HitRecord, Hittable, Scene};
use ray::Ray;
use vector3::Vector;

#[allow(unused_imports)]
use util::{one_by_one, progress_bar, sixteen_by_nine, two_by_one};

const IMG_WIDTH: usize = 480;
const IMG_HEIGHT: usize = sixteen_by_nine(IMG_WIDTH);
const SAMPLES: usize = 50;

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
        "Scene loaded from {}, rendering ({} x {} @ {} samples)",
        path.file_name().and_then(|p| p.to_str()).unwrap(),
        IMG_WIDTH,
        IMG_HEIGHT,
        SAMPLES,
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
        let emitted =
            material.emitted(hit_record.u, hit_record.v, hit_record.p);
        if depth < MAX_RECURSIVE_DEPTH {
            if let Some((attenuation, scattered)) =
                material.scatter(r, hit_record)
            {
                return emitted
                    + attenuation * color(scattered, scene, depth + 1);
            }
        }

        emitted
    } else {
        // let unit_direction = r.dir().normalize();
        // let t = 0.5 * (unit_direction.y + 1.0);

        // Vector::ones() * (1.0 - t) + Vector::new(0.5, 0.7, 1.0) * t
        Vector::zeros()
    }
}
