use crate::{HitRecord, Ray, Vector};

use std::fmt::Debug;

mod lambertian;
pub use lambertian::*;

mod metal;
pub use metal::*;

mod dielectric;
pub use dielectric::*;

mod diffuse_light;
pub use diffuse_light::*;

pub trait Material: Debug + Send + Sync {
    /// Returns: Whether a ray was scattered, the attenuation, and scattered ray
    fn scatter(
        &self,
        r_in: Ray,
        hit_record: HitRecord,
    ) -> Option<(Vector, Ray)>;

    /// Return the amount of light emitted by this material
    fn emitted(&self, _u: f32, _v: f32, _p: Vector) -> Vector {
        Vector::zeros()
    }
}
