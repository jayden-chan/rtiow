use crate::materials::Material;
use crate::util::{vector_reflect, vector_refract};
use crate::{HitRecord, Ray, Vector};

use std::f32;

#[derive(Debug, Copy, Clone)]
pub struct Dielectric {
    ref_idx: f32,
}

impl Dielectric {
    pub fn new(ref_idx: f32) -> Self {
        Self { ref_idx }
    }
}

impl Material for Dielectric {
    fn scatter(&self, r_in: Ray, hit_record: HitRecord) -> (bool, Vector, Ray) {
        let reflected = vector_reflect(r_in.dir(), hit_record.normal);

        let (outward_normal, ni_over_nt) =
            if Vector::dot(r_in.dir(), hit_record.normal) > 0.0 {
                (-hit_record.normal, self.ref_idx)
            } else {
                (hit_record.normal, 1.0 / self.ref_idx)
            };

        let (did_refract, refracted) =
            vector_refract(r_in.dir(), outward_normal, ni_over_nt);

        if did_refract {
            (
                true,
                Vector::ones(),
                Ray::new(hit_record.p, refracted.unwrap()),
            )
        } else {
            (false, Vector::ones(), Ray::new(hit_record.p, reflected))
        }
    }
}
