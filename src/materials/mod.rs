use crate::{pdf::Pdf, HitRecord, Ray, Vector};

use std::fmt::Debug;

mod lambertian;
pub use lambertian::*;

mod metal;
pub use metal::*;

mod dielectric;
pub use dielectric::*;

mod diffuse_light;
pub use diffuse_light::*;

pub trait MaterialClone {
    fn clone_box(&self) -> Box<dyn Material>;
}

impl<T> MaterialClone for T
where
    T: 'static + Material + Clone,
{
    fn clone_box(&self) -> Box<dyn Material> {
        Box::new(self.clone())
    }
}

impl Clone for Box<dyn Material> {
    fn clone(&self) -> Box<dyn Material> {
        self.clone_box()
    }
}

#[derive(Debug)]
pub struct ScatterRecord {
    pub specular_ray: Ray,
    pub attenuation: Vector,
    pub pdf: Option<Box<dyn Pdf>>,
}

pub trait Material: Debug + Send + Sync + MaterialClone {
    /// Returns: Whether a ray was scattered, the attenuation, scattered ray, and pdf value
    fn scatter(
        &self,
        _r_in: Ray,
        _hit_record: HitRecord,
    ) -> Option<ScatterRecord> {
        None
    }

    /// Return the amount of light emitted by this material
    fn emitted(&self, _r_in: Ray, _hit_record: HitRecord) -> Vector {
        Vector::zeros()
    }

    /// Returns the value of the material's scattering pdf for the given incident ray and hit
    /// record.
    fn scattering_pdf(
        &self,
        _r_in: Ray,
        _hit_record: HitRecord,
        _scattered: Ray,
    ) -> f32 {
        1.0
    }
}
