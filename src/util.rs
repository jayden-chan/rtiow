use crate::Vector;
use rand::prelude::*;

use std::f32;

// TODO: This function should be replaced with RSQRT SIMD
#[inline(always)]
#[allow(clippy::all)] // Pointer casts in here cause warnings
pub fn fast_inv_sqrt(x: f32) -> f32 {
    let i: u32 = unsafe { std::mem::transmute(x) };
    let j = 0x5F3759DF - (i >> 1);
    let y: f32 = unsafe { std::mem::transmute(j) };
    y * (1.5 - 0.5 * x * y * y)
}

#[inline]
pub fn random_in_unit_sphere() -> Vector {
    let mut p = 2.0
        * Vector::new(random::<f32>(), random::<f32>(), random::<f32>())
        - Vector::ones();

    while Vector::dot(p, p) >= 1.0 {
        p = 2.0
            * Vector::new(random::<f32>(), random::<f32>(), random::<f32>())
            - Vector::ones();
    }

    p
}

#[inline(always)]
pub fn vector_reflect(v: Vector, n: Vector) -> Vector {
    v - 2.0 * Vector::dot(v, n) * n
}

#[inline]
pub fn vector_refract(
    v: Vector,
    n: Vector,
    ni_over_nt: f32,
) -> (bool, Option<Vector>) {
    let v = v.normalize();
    let dt = Vector::dot(v, n);
    let discriminant = 1.0 - ni_over_nt * ni_over_nt * (1.0 - dt * dt);

    if discriminant > 0.0 {
        (
            true,
            Some(ni_over_nt * (v - n * dt) - n * f32::sqrt(discriminant)),
        )
    } else {
        (false, None)
    }
}
