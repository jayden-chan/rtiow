use crate::Vector;
use rand::prelude::*;

// TODO: This function should be replaced with RSQRT SIMD
#[inline(always)]
#[allow(clippy::all)] // Pointer casts in here cause warnings
pub fn fast_inv_sqrt(x: f32) -> f32 {
    let i: u32 = unsafe { std::mem::transmute(x) };
    let j = 0x5F3759DF - (i >> 1);
    let y: f32 = unsafe { std::mem::transmute(j) };
    y * (1.5 - 0.5 * x * y * y)
}

#[inline(always)]
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
