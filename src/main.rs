extern crate rand;

mod image;
mod ray;
mod util;
mod vector3;

use std::f32;

use image::{gen_ppm, Pixel};
use ray::Ray;
use vector3::Vector;

const IMG_WIDTH: usize = 200;
const IMG_HEIGHT: usize = 100;

fn main() {
    let mut image = Vec::new();

    let lower_left_corner = Vector::new(-2.0, -1.0, -1.0);
    let horizontal = Vector::new(4.0, 0.0, 0.0);
    let vertical = Vector::new(0.0, 2.0, 0.0);
    let origin = Vector::zero();

    for j in (0..IMG_HEIGHT).rev() {
        image.push(Vec::new());
        for i in 0..IMG_WIDTH {
            let u = i as f32 / IMG_WIDTH as f32;
            let v = j as f32 / IMG_HEIGHT as f32;

            let r = Ray::new(
                origin,
                lower_left_corner + (horizontal * u) + (vertical * v),
            );

            let color = color(r);
            image[(IMG_HEIGHT - 1) - j].push(Pixel {
                r: (255 as f32 * color.x) as u8,
                g: (255 as f32 * color.y) as u8,
                b: (255 as f32 * color.z) as u8,
            });
        }
    }

    gen_ppm(image);
}

fn color(r: Ray) -> Vector {
    let t = ray_intersects_sphere(Vector::new(0.0, 0.0, -1.0), 0.5, r);
    match t {
        Some(t) => {
            let N = (r.point_at_parameter(t) - Vector::new(0.0, 0.0, -1.0))
                .normalize();

            return 0.5 * (N + 1.0);
        }
        None => {
            let unit_direction = r.dir().normalize();
            let t = 0.5 * (unit_direction.y + 1.0);

            return Vector::unit() * (1.0 - t) + Vector::new(0.5, 0.7, 1.0) * t;
        }
    }
}

fn ray_intersects_sphere(center: Vector, radius: f32, r: Ray) -> Option<f32> {
    let oc = r.origin() - center;

    let a = Vector::dot(r.dir(), r.dir());
    let b = Vector::dot(oc, r.dir()) * 2.0;
    let c = Vector::dot(oc, oc) - radius * radius;
    let discriminant = b * b - 4.0 * a * c;

    if discriminant < 0.0 {
        None
    } else {
        return Some((-b - f32::sqrt(discriminant)) / (2.0 * a));
    }
}
