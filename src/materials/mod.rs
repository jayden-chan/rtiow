use crate::{HitRecord, Ray, Vector};

use std::fmt::Debug;

mod labertian;
pub use labertian::*;

mod metal;
pub use metal::*;

pub trait Material: Debug {
    /// Returns: Whether a ray was scattered, the attenuation, and scattered ray
    fn scatter(&self, r_in: Ray, hit_record: HitRecord) -> (bool, Vector, Ray);
}
