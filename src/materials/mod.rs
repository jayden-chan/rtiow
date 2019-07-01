use crate::{HitRecord, Ray, Vector};

mod labertian;
pub use labertian::*;

pub trait Material {
    /// Returns: Whether a ray was scattered, the attenuation, and scattered ray
    fn scatter(&self, r_in: Ray, hit_record: HitRecord) -> (bool, Vector, Ray);
}
