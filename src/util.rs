//! This module contains some (mostly mathematical)
//! utility functions that are used by some of the other modules.
use crate::Vector;

use rand::prelude::*;

use std::f32::consts::PI;
use std::io::stdout;
use std::io::Write;

#[allow(unused)]
pub const fn sixteen_by_nine(width: usize) -> usize {
    (width * 9) / 16
}

#[allow(unused)]
pub const fn two_by_one(width: usize) -> usize {
    width / 2
}

#[allow(unused)]
pub const fn one_by_one(width: usize) -> usize {
    width
}

// TODO: This function should be replaced with RSQRT SIMD
#[allow(clippy::all)] // Pointer casts in here cause warnings
pub fn fast_inv_sqrt(x: f32) -> f32 {
    let i: u32 = unsafe { std::mem::transmute(x) };
    let j = 0x5F3759DF - (i >> 1);
    let y: f32 = unsafe { std::mem::transmute(j) };
    y * (1.5 - 0.5 * x * y * y)
}

pub fn random_in_unit_disk() -> Vector {
    let mut p = 2.0 * Vector::new(random::<f32>(), random::<f32>(), 0.0)
        - Vector::new(1.0, 1.0, 0.0);

    while Vector::dot(p, p) >= 1.0 {
        p = 2.0 * Vector::new(random::<f32>(), random::<f32>(), 0.0)
            - Vector::new(1.0, 1.0, 0.0);
    }

    p
}

pub fn random_in_unit_sphere() -> Vector {
    let mut p = 2.0 * Vector::rand() - Vector::ones();

    while Vector::dot(p, p) >= 1.0 {
        p = 2.0 * Vector::rand() - Vector::ones();
    }

    p
}

pub fn random_on_unit_sphere() -> Vector {
    random_in_unit_sphere().normalize()
}

pub fn random_cosine_dir() -> Vector {
    let r1 = random::<f32>();
    let r2 = random::<f32>();

    let phi = 2.0 * PI * r1;
    let x = f32::cos(phi) * f32::sqrt(r2);
    let y = f32::sin(phi) * f32::sqrt(r2);
    let z = f32::sqrt(1.0 - r2);

    Vector::new(x, y, z)
}

/// Computes the u and v values for a sphere
pub fn sphere_uv(p: Vector) -> (f32, f32) {
    let phi = f32::atan2(p.z, p.x);
    let theta = f32::asin(p.y);
    ((1.0 - (phi + PI) / (2.0 * PI)), ((theta + PI / 2.0) / PI))
}

pub fn vector_reflect(v: Vector, n: Vector) -> Vector {
    v - 2.0 * Vector::dot(v, n) * n
}

pub fn vector_refract(v: Vector, n: Vector, ni_over_nt: f32) -> Option<Vector> {
    let v = v.normalize();
    let dt = Vector::dot(v, n);
    let discriminant = 1.0 - ni_over_nt * ni_over_nt * (1.0 - dt * dt);

    if discriminant > 0.0 {
        Some(ni_over_nt * (v - n * dt) - n * discriminant.sqrt())
    } else {
        None
    }
}

/// Renders a progress bar on the command line
pub fn progress_bar(
    curr: usize,
    total: usize,
    width: usize,
    text: &'static str,
) -> bool {
    let done_chars = (curr as f32 / total as f32) * width as f32;
    let blank_chars = width - done_chars as usize;

    print!(
        "    {:10} [{}{}] {:.2}%\r",
        text,
        "=".repeat(done_chars as usize),
        " ".repeat(blank_chars),
        (curr as f32 / total as f32) * 100.0
    );

    stdout().flush().unwrap();

    curr == total
}
