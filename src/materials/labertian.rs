use crate::util::random_in_unit_sphere;
use crate::{HitRecord, Ray, Vector};

use super::Material;

/// Lambertian material impl - an ideal diffuse reflector
#[derive(Debug, Copy, Clone)]
pub struct Lambertian {
    albedo: Vector,
}

impl Lambertian {
    pub fn new(r: f32, g: f32, b: f32) -> Self {
        Self {
            albedo: Vector::new(r, g, b),
        }
    }
}

impl Material for Lambertian {
    fn scatter(
        &self,
        _r_in: Ray,
        hit_record: HitRecord,
    ) -> (bool, Vector, Ray) {
        let target = hit_record.p + hit_record.normal + random_in_unit_sphere();

        (
            true,
            self.albedo,
            Ray::new(hit_record.p, target - hit_record.p),
        )
    }
}
