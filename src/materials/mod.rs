use crate::{HitRecord, Ray, Vector};

use std::fmt::Debug;

mod labertian;
pub use labertian::*;

mod metal;
pub use metal::*;

mod dielectric;
pub use dielectric::*;

pub trait Material: Debug + Send + Sync {
    /// Returns: Whether a ray was scattered, the attenuation, and scattered ray
    fn scatter(&self, r_in: Ray, hit_record: HitRecord) -> (bool, Vector, Ray);
}
