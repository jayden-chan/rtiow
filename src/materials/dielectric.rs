use crate::{
    materials::{Material, ScatterRecord},
    util::{vector_reflect, vector_refract},
    HitRecord, Ray, Vector,
};

use rand::prelude::*;

use std::f32;

/// The Dielectric material type. This material partially
/// reflects and refracts rays that interact with it.
#[derive(Debug, Copy, Clone)]
pub struct Dielectric {
    ref_idx: f32,
}

impl Dielectric {
    pub fn new(ref_idx: f32) -> Self {
        Self { ref_idx }
    }
}

/// Provides an approximation for the contribution of the Fresnel factor
/// during reflection calculations
///
/// [More info](https://en.wikipedia.org/wiki/Schlick%27s_approximation)
fn schlick(cosine: f32, ref_idx: f32) -> f32 {
    let r0 = (1.0 - ref_idx) / (1.0 + ref_idx);
    let r0 = r0 * r0;

    r0 + (1.0 - r0) * f32::powi(1.0 - cosine, 5)
}

impl Material for Dielectric {
    fn scatter(
        &self,
        r_in: Ray,
        hit_record: HitRecord,
    ) -> Option<ScatterRecord> {
        let reflected = vector_reflect(r_in.dir(), hit_record.normal);

        let (outward_normal, ni_over_nt, cosine) =
            if Vector::dot(r_in.dir(), hit_record.normal) > 0.0 {
                (
                    -hit_record.normal,
                    self.ref_idx,
                    self.ref_idx
                        * Vector::dot(r_in.dir(), hit_record.normal)
                        * r_in.dir().inv_mag(),
                )
            } else {
                (
                    hit_record.normal,
                    1.0 / self.ref_idx,
                    -Vector::dot(r_in.dir(), hit_record.normal)
                        * r_in.dir().inv_mag(),
                )
            };

        let refracted = vector_refract(r_in.dir(), outward_normal, ni_over_nt);

        let reflect_probability = if refracted.is_some() {
            schlick(cosine, self.ref_idx)
        } else {
            1.0
        };

        if random::<f32>() >= reflect_probability {
            Some(ScatterRecord {
                specular_ray: Ray::new(
                    hit_record.p,
                    refracted.unwrap(),
                    r_in.time(),
                ),
                attenuation: Vector::ones(),
                pdf: None,
            })
        } else {
            Some(ScatterRecord {
                specular_ray: Ray::new(hit_record.p, reflected, r_in.time()),
                attenuation: Vector::ones(),
                pdf: None,
            })
        }
    }
}
