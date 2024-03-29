use crate::{
    materials::{Material, ScatterRecord},
    util::{random_in_unit_sphere, vector_reflect},
    HitRecord, Ray, Vector,
};

/// Metal - a surface that simply reflects all light
#[derive(Debug, Copy, Clone)]
pub struct Metal {
    albedo: Vector,
    fuzz: f32,
}

impl Metal {
    pub fn new(r: f32, g: f32, b: f32, fuzz: f32) -> Self {
        Self {
            albedo: Vector::new(r, g, b),
            fuzz,
        }
    }
}

impl Material for Metal {
    fn scatter(
        &self,
        r_in: Ray,
        hit_record: HitRecord,
    ) -> Option<ScatterRecord> {
        let reflected =
            vector_reflect(r_in.dir().normalize(), hit_record.normal);
        let scattered = Ray::new(
            hit_record.p,
            reflected + self.fuzz * random_in_unit_sphere(),
            r_in.time(),
        );

        if Vector::dot(scattered.dir(), hit_record.normal) > 0.0 {
            Some(ScatterRecord {
                specular_ray: scattered,
                attenuation: self.albedo,
                pdf: None,
            })
        } else {
            None
        }
    }
}
