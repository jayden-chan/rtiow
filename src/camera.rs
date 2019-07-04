use crate::{Ray, Vector};

use std::f32;

#[derive(Debug)]
pub struct Camera {
    lower_left_corner: Vector,
    horizontal: Vector,
    vertical: Vector,
    origin: Vector,
}

impl Camera {
    pub fn new(
        look_from: Vector,
        look_at: Vector,
        vup: Vector,
        vfov: f32,
        aspect_r: f32,
    ) -> Self {
        let theta = vfov * f32::consts::PI / 180.0;
        let half_height = f32::tan(theta / 2.0);
        let half_width = aspect_r * half_height;

        let w = (look_from - look_at).normalize();
        let u = Vector::cross(vup, w).normalize();
        let v = Vector::cross(w, u);

        Self {
            lower_left_corner: look_from - half_width * u - half_height * v - w,
            horizontal: 2.0 * half_width * u,
            vertical: 2.0 * half_height * v,
            origin: look_from,
        }
    }

    pub fn default(aspect_r: f32) -> Self {
        Self {
            lower_left_corner: Vector::new(-aspect_r, -1.0, -1.0),
            horizontal: Vector::new(2.0 * aspect_r, 0.0, 0.0),
            vertical: Vector::new(0.0, 2.0, 0.0),
            origin: Vector::new(0.0, 0.0, 0.0),
        }
    }
}

impl Camera {
    pub fn get_ray(&self, u: f32, v: f32) -> Ray {
        Ray::new(
            self.origin,
            self.lower_left_corner + self.horizontal * u + self.vertical * v
                - self.origin,
        )
    }
}
