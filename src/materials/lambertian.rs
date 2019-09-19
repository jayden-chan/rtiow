use crate::{
    materials::{Material, ScatterRecord},
    onb::Onb,
    pdf::Cosine,
    textures::Texture,
    util::random_cosine_dir,
    HitRecord, Ray, Vector,
};

use std::f32::consts::PI;

/// Lambertian material impl - an ideal diffuse reflector
#[derive(Debug, Clone)]
pub struct Lambertian {
    texture: Box<dyn Texture>,
}

impl Lambertian {
    pub fn new(texture: Box<dyn Texture>) -> Self {
        Self { texture }
    }
}

impl Material for Lambertian {
    fn scatter(
        &self,
        r_in: Ray,
        hit_record: HitRecord,
    ) -> Option<ScatterRecord> {
        let uvw = Onb::build_from_w(hit_record.normal);
        let dir = uvw.local(random_cosine_dir());

        let scattered = Ray::new(hit_record.p, dir.normalize(), r_in.time());

        Some(ScatterRecord {
            specular_ray: scattered,
            attenuation: self.texture.value(
                hit_record.u,
                hit_record.v,
                hit_record.p,
            ),
            pdf: Some(Box::new(Cosine::new(hit_record.normal))),
        })
    }

    fn scattering_pdf(
        &self,
        _r_in: Ray,
        hit_record: HitRecord,
        scattered: Ray,
    ) -> f32 {
        let dot = Vector::dot(hit_record.normal, scattered.dir().normalize());
        return if dot < 0.0 { 0.0 } else { dot / PI };
    }
}
