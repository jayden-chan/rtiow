extern crate rand;

mod camera;
mod image;
mod materials;
mod objects;
mod ray;
mod util;
mod vector3;

use std::f32;
use std::io::{self, Write};

use rand::prelude::*;

use camera::Camera;
use image::{gen_ppm, Pixel};
use materials::{Lambertian, Metal};
use objects::{HitRecord, Hittable, ObjectList, Sphere};
use ray::Ray;
use vector3::Vector;

const IMG_WIDTH: usize = 400;
const IMG_HEIGHT: usize = 200;
const SAMPLES: usize = 500;
const MAX_RECURSIVE_DEPTH: usize = 50;

fn main() {
    let mut image = Vec::with_capacity(IMG_HEIGHT);

    let world = ObjectList::from_objects(vec![
        Box::new(Sphere::new(
            Vector::new(0.0, 0.0, -1.0),
            0.5,
            Box::new(Lambertian::new(0.8, 0.3, 0.3)),
        )),
        Box::new(Sphere::new(
            Vector::new(0.0, -100.5, -1.0),
            100.0,
            Box::new(Lambertian::new(0.5, 0.5, 0.5)),
        )),
        Box::new(Sphere::new(
            Vector::new(1.0, 0.0, -1.0),
            0.5,
            Box::new(Metal::new(0.8, 0.6, 0.2)),
        )),
        Box::new(Sphere::new(
            Vector::new(-1.0, 0.0, -1.0),
            0.5,
            Box::new(Metal::new(0.8, 0.8, 0.8)),
        )),
    ]);

    let camera = Camera::new(IMG_WIDTH as f32, IMG_HEIGHT as f32);

    for j in (0..IMG_HEIGHT).rev() {
        image.push(Vec::with_capacity(IMG_WIDTH));

        for i in 0..IMG_WIDTH {
            let mut pixel = Vector::zero();
            for _ in 0..SAMPLES {
                let u = (i as f32 + random::<f32>()) / IMG_WIDTH as f32;
                let v = (j as f32 + random::<f32>()) / IMG_HEIGHT as f32;

                let r = camera.get_ray(u, v);

                pixel += color(r, &world, 0);
            }

            let done_pixels = image.len() * IMG_WIDTH + i;
            if done_pixels % 1000 == 0 {
                print!(
                    "Rendering... {:4.1}%\r",
                    (done_pixels as f32 / (IMG_HEIGHT * IMG_WIDTH) as f32)
                        * 100.0
                );
                io::stdout().flush().unwrap();
            }

            pixel /= SAMPLES as f32;
            image[(IMG_HEIGHT - 1) - j].push(Pixel {
                r: (255.0 * pixel.x) as u8,
                g: (255.0 * pixel.y) as u8,
                b: (255.0 * pixel.z) as u8,
            });
        }
    }

    println!();

    gen_ppm(image);
}

fn color(r: Ray, world: &ObjectList, depth: usize) -> Vector {
    let (hit, result) = world.hit(r, 0.00001, f32::MAX);

    if hit {
        let (hit_record, material) = result.unwrap();
        let (did_scatter, attenuation, scattered) =
            material.scatter(r, hit_record);

        if depth < MAX_RECURSIVE_DEPTH && did_scatter {
            return attenuation * color(scattered, world, depth + 1);
        } else {
            return Vector::zero();
        }
    } else {
        let unit_direction = r.dir().normalize();
        let t = 0.5 * (unit_direction.y + 1.0);

        Vector::ones() * (1.0 - t) + Vector::new(0.5, 0.7, 1.0) * t
    }
}
