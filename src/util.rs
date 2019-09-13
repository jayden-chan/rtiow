//! This module contains some (mostly mathematical)
//! utility functions that are used by some of the other modules.
//! They are mostly small and inlined
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
#[inline]
#[allow(clippy::all)] // Pointer casts in here cause warnings
pub fn fast_inv_sqrt(x: f32) -> f32 {
    let i: u32 = unsafe { std::mem::transmute(x) };
    let j = 0x5F3759DF - (i >> 1);
    let y: f32 = unsafe { std::mem::transmute(j) };
    y * (1.5 - 0.5 * x * y * y)
}

#[inline]
pub fn random_in_unit_disk() -> Vector {
    let mut p = 2.0 * Vector::new(random::<f32>(), random::<f32>(), 0.0)
        - Vector::new(1.0, 1.0, 0.0);

    while Vector::dot(p, p) >= 1.0 {
        p = 2.0 * Vector::new(random::<f32>(), random::<f32>(), 0.0)
            - Vector::new(1.0, 1.0, 0.0);
    }

    p
}

#[inline]
pub fn random_in_unit_sphere() -> Vector {
    let mut p = 2.0 * Vector::rand() - Vector::ones();

    while Vector::dot(p, p) >= 1.0 {
        p = 2.0 * Vector::rand() - Vector::ones();
    }

    p
}

#[inline]
pub fn sphere_uv(p: Vector) -> (f32, f32) {
    let phi = f32::atan2(p.z, p.x);
    let theta = f32::asin(p.y);
    ((1.0 - (phi + PI) / (2.0 * PI)), ((theta + PI / 2.0) / PI))
}

#[inline]
pub fn vector_reflect(v: Vector, n: Vector) -> Vector {
    v - 2.0 * Vector::dot(v, n) * n
}

#[inline]
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
