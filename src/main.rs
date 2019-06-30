extern crate rand;

mod image;
mod ray;
mod util;
mod vector3;

use image::{gen_ppm, Pixel};
use vector3::Vector;

use ray::Ray;

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
    if ray_intersects_sphere(Vector::new(0.0, 0.0, -1.0), 0.5, r) {
        return Vector::new(1.0, 0.0, 0.0);
    }

    let unit_direction = r.dir().normalize();
    let t = 0.5 * (unit_direction.y + 1.0);

    Vector::unit() * (1.0 - t) + Vector::new(0.5, 0.7, 1.0) * t
}

fn ray_intersects_sphere(center: Vector, radius: f32, r: Ray) -> bool {
    let oc = r.origin() - center;

    let a = Vector::dot(r.dir(), r.dir());
    let b = Vector::dot(oc, r.dir()) * 2.0;
    let c = Vector::dot(oc, oc) - radius * radius;

    b * b - 4.0 * a * c > 0.0
}
