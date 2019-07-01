extern crate rand;
extern crate rayon;

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

use camera::Camera;
use image::{gen_ppm, Pixel};
use materials::{Lambertian, Metal};
use objects::{HitRecord, Hittable, ObjectList, Sphere};
use ray::Ray;
use vector3::Vector;

const IMG_WIDTH: usize = 1920;
const IMG_HEIGHT: usize = 1080;
const SAMPLES: usize = 2000;
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

    let world = ObjectList::from_objects(vec![
        Box::new(Sphere::new(
            Vector::new(0.0, 0.0, -1.0),
            0.5,
            Box::new(Lambertian::new(0.9, 0.1, 0.1)),
        )),
        Box::new(Sphere::new(
            Vector::new(0.0, -100.5, -1.0),
            100.0,
            Box::new(Lambertian::new(0.2, 0.2, 0.2)),
        )),
        Box::new(Sphere::new(
            Vector::new(1.0, 0.0, -1.0),
            0.5,
            Box::new(Metal::new(0.8, 0.6, 0.2, 0.0)),
        )),
        Box::new(Sphere::new(
            Vector::new(-1.0, 0.0, -1.0),
            0.5,
            Box::new(Metal::new(0.1, 0.4, 0.9, 0.5)),
        )),
    ]);

    let camera = Camera::new(IMG_WIDTH as f32, IMG_HEIGHT as f32);

    image.iter_mut().for_each(|row| {
        row.par_iter_mut().for_each(|pixel| {
            let mut curr_pixel = Vector::zero();

            for _ in 0..SAMPLES {
                let u = (pixel.x as f32 + random::<f32>()) / IMG_WIDTH as f32;
                let v = (pixel.y as f32 + random::<f32>()) / IMG_HEIGHT as f32;

                let r = camera.get_ray(u, v);

                curr_pixel += color(r, &world, 0);
            }

            curr_pixel /= SAMPLES as f32;
            pixel.r = (255.0 * f32::sqrt(curr_pixel.x)) as u8;
            pixel.g = (255.0 * f32::sqrt(curr_pixel.y)) as u8;
            pixel.b = (255.0 * f32::sqrt(curr_pixel.z)) as u8;
        });
    });

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
