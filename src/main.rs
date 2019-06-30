extern crate rand;

mod image;
mod object;
mod objects;
mod ray;
mod util;
mod vector3;

use std::f32;

use image::{gen_ppm, Pixel};
use object::{HitRecord, Hittable, ObjectList};
use objects::Sphere;
use ray::Ray;
use vector3::Vector;

const IMG_WIDTH: usize = 200;
const IMG_HEIGHT: usize = 100;

fn main() {
    let mut image = Vec::new();

    let lower_left_corner = Vector::new(
        -(IMG_WIDTH as f32 / 100.0),
        -(IMG_HEIGHT as f32 / 100.0),
        -1.0,
    );
    let horizontal = Vector::new(IMG_WIDTH as f32 / 50.0, 0.0, 0.0);
    let vertical = Vector::new(0.0, IMG_HEIGHT as f32 / 50.0, 0.0);
    let origin = Vector::zero();

    let world = ObjectList::from_objects(vec![
        Sphere::new(Vector::new(0.0, 0.0, -1.0), 0.5),
        Sphere::new(Vector::new(0.0, -100.5, -1.0), 100.0),
    ]);

    for j in (0..IMG_HEIGHT).rev() {
        image.push(Vec::new());
        for i in 0..IMG_WIDTH {
            let u = i as f32 / IMG_WIDTH as f32;
            let v = j as f32 / IMG_HEIGHT as f32;

            let r = Ray::new(
                origin,
                lower_left_corner + (horizontal * u) + (vertical * v),
            );

            let color = color(r, &world);
            image[(IMG_HEIGHT - 1) - j].push(Pixel {
                r: (255 as f32 * color.x) as u8,
                g: (255 as f32 * color.y) as u8,
                b: (255 as f32 * color.z) as u8,
            });
        }
    }

    gen_ppm(image);
}

fn color<T: Hittable>(r: Ray, world: &ObjectList<T>) -> Vector {
    let mut rec = HitRecord::empty();
    let hit = world.hit(r, 0.0, f32::MAX, &mut rec);

    if hit {
        return 0.5 * (rec.normal.unwrap() + 1.0);
    } else {
        let unit_direction = r.dir().normalize();
        let t = 0.5 * (unit_direction.y + 1.0);

        return Vector::ones() * (1.0 - t) + Vector::new(0.5, 0.7, 1.0) * t;
    }
}
