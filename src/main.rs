extern crate rand;
extern crate rayon;
extern crate serde;

mod camera;
mod image;
mod materials;
mod objects;
mod ray;
mod util;
mod vector3;

use std::f32;

use rand::prelude::*;
use rayon::prelude::*;

use std::path::Path;

use camera::Camera;
use image::{gen_ppm, Pixel};
use objects::{HitRecord, Hittable, Scene};
use ray::Ray;
use vector3::Vector;

const IMG_WIDTH: usize = 300;
const IMG_HEIGHT: usize = 150;
const SAMPLES: usize = 100;
const MAX_RECURSIVE_DEPTH: usize = 50;

fn main() {
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

    let path = Path::new("./scenes/spheres_2.json");
    let scene = Scene::from_json(path);

    match scene {
        Ok(scene) => {
            println!(
                "Scene loaded from {}, rendering ({} x {} @ {} samples)",
                path.display(),
                IMG_WIDTH,
                IMG_HEIGHT,
                SAMPLES
            );

            println!("This may take a while...");
            let camera = Camera::new(IMG_WIDTH as f32, IMG_HEIGHT as f32);

            image.iter_mut().for_each(|row| {
                row.par_iter_mut().for_each(|pixel| {
                    let mut curr_pixel = Vector::zero();

                    for _ in 0..SAMPLES {
                        let u = (pixel.x as f32 + random::<f32>())
                            / IMG_WIDTH as f32;
                        let v = (pixel.y as f32 + random::<f32>())
                            / IMG_HEIGHT as f32;

                        let r = camera.get_ray(u, v);

                        curr_pixel += color(r, &scene, 0);
                    }

                    curr_pixel /= SAMPLES as f32;
                    pixel.r = (255.0 * f32::sqrt(curr_pixel.x)) as u8;
                    pixel.g = (255.0 * f32::sqrt(curr_pixel.y)) as u8;
                    pixel.b = (255.0 * f32::sqrt(curr_pixel.z)) as u8;
                });
            });

            gen_ppm(image);
        }
        Err(e) => {
            panic!(e);
        }
    }
}

fn color(r: Ray, scene: &Scene, depth: usize) -> Vector {
    let (hit, result) = scene.hit(r, 0.00001, f32::MAX);

    if hit {
        let (hit_record, material) = result.unwrap();
        let (did_scatter, attenuation, scattered) =
            material.scatter(r, hit_record);

        if depth < MAX_RECURSIVE_DEPTH && did_scatter {
            return attenuation * color(scattered, scene, depth + 1);
        } else {
            return Vector::zero();
        }
    } else {
        let unit_direction = r.dir().normalize();
        let t = 0.5 * (unit_direction.y + 1.0);

        Vector::ones() * (1.0 - t) + Vector::new(0.5, 0.7, 1.0) * t
    }
}
