use crate::util::random_in_unit_disk;
use crate::{Ray, Vector};

use std::f32;

#[derive(Debug)]
pub struct Camera {
    lower_left_corner: Vector,
    horizontal: Vector,
    vertical: Vector,
    origin: Vector,
    lens_radius: f32,
    u: Vector,
    v: Vector,
    w: Vector,
}

impl Camera {
    pub fn new(
        look_from: Vector,
        look_at: Vector,
        vup: Vector,
        vfov: f32,
        aspect_r: f32,
        aperture: f32,
        focus_dist: f32,
    ) -> Self {
        let theta = vfov * f32::consts::PI / 180.0;
        let half_height = f32::tan(theta / 2.0);
        let half_width = aspect_r * half_height;

        let w = (look_from - look_at).normalize();
        let u = Vector::cross(vup, w).normalize();
        let v = Vector::cross(w, u);

        Self {
            u,
            v,
            w,
            lower_left_corner: look_from
                - half_width * focus_dist * u
                - half_height * focus_dist * v
                - focus_dist * w,
            horizontal: 2.0 * half_width * focus_dist * u,
            vertical: 2.0 * half_height * focus_dist * v,
            origin: look_from,
            lens_radius: aperture / 2.0,
        }
    }

    pub fn default(aspect_r: f32) -> Self {
        Self::new(
            Vector::zeros(),
            Vector::new(0.0, 0.0, -1.0),
            Vector::new(0.0, 1.0, 0.0),
            90.0,
            aspect_r,
            0.0,
            1.0,
        )
    }
}

impl Camera {
    pub fn get_ray(&self, u: f32, v: f32) -> Ray {
        let rd = self.lens_radius * random_in_unit_disk();
        let offset = self.u * rd.x + self.v * rd.y;

        Ray::new(
            self.origin + offset,
            self.lower_left_corner + self.horizontal * u + self.vertical * v
                - self.origin
                - offset,
        )
    }
}
