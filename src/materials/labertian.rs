use crate::util::random_in_unit_sphere;
use crate::{HitRecord, Ray, Vector};

use super::Material;

pub struct Lambertian {
    albedo: Vector,
}

impl Lambertian {
    pub fn new(albedo: Vector) -> Self {
        Self { albedo }
    }
}

impl Material for Lambertian {
    fn scatter(
        &self,
        _r_in: Ray,
        hit_record: HitRecord,
    ) -> (bool, Vector, Ray) {
        let target = hit_record.p + hit_record.normal + random_in_unit_sphere();

        return (
            true,
            self.albedo,
            Ray::new(hit_record.p, target - hit_record.p),
        );
    }
}
