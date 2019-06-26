#[inline(always)]
#[allow(clippy::all)] // Pointer casts in here cause warnings
pub fn fast_inv_sqrt(x: f32) -> f32 {
    let i: u32 = unsafe {
        std::mem::transmute(x)
    };

    let j = 0x5F3759DF - (i >> 1);

    let y: f32 = unsafe {
        std::mem::transmute(j)
    };

    // Two iterations of Newton's method for now
    let y = y * (1.5 - 0.5 * x * y * y);
    y * (1.5 - 0.5 * x * y * y)
}
