use crate::util::random_in_unit_disk;
use crate::{Ray, Vector};

use std::f32;

use rand::prelude::*;

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
    t0: f32,
    t1: f32,
}

pub struct CameraConstructor {
    pub look_from: Vector,
    pub look_at: Vector,
    pub vup: Vector,
    pub vfov: f32,
    pub aspect_r: f32,
    pub aperture: f32,
    pub focus_dist: f32,
    pub t0: f32,
    pub t1: f32,
}

impl Camera {
    pub fn new(vals: CameraConstructor) -> Self {
        let theta = vals.vfov * f32::consts::PI / 180.0;
        let half_height = f32::tan(theta / 2.0);
        let half_width = vals.aspect_r * half_height;

        let w = (vals.look_from - vals.look_at).normalize();
        let u = Vector::cross(vals.vup, w).normalize();
        let v = Vector::cross(w, u);

        Self {
            u,
            v,
            w,
            lower_left_corner: vals.look_from
                - half_width * vals.focus_dist * u
                - half_height * vals.focus_dist * v
                - vals.focus_dist * w,
            horizontal: 2.0 * half_width * vals.focus_dist * u,
            vertical: 2.0 * half_height * vals.focus_dist * v,
            origin: vals.look_from,
            lens_radius: vals.aperture / 2.0,
            t0: vals.t0,
            t1: vals.t1,
        }
    }

    pub fn default(aspect_r: f32) -> Self {
        let defaults = CameraConstructor {
            look_from: Vector::zeros(),
            look_at: Vector::new(0.0, 0.0, -1.0),
            vup: Vector::new(0.0, 1.0, 0.0),
            vfov: 90.0,
            aspect_r,
            aperture: 0.0,
            focus_dist: 1.0,
            t0: 0.0,
            t1: 0.0,
        };

        Self::new(defaults)
    }
}

impl Camera {
    pub fn get_ray(&self, u: f32, v: f32) -> Ray {
        let rd = self.lens_radius * random_in_unit_disk();
        let offset = self.u * rd.x + self.v * rd.y;
        let time = self.t0 + random::<f32>() * (self.t1 - self.t0);

        Ray::new(
            self.origin + offset,
            self.lower_left_corner + self.horizontal * u + self.vertical * v
                - self.origin
                - offset,
            time,
        )
    }
}
