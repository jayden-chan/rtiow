use crate::util::{random_cosine_dir, random_on_unit_sphere};
use crate::{HitRecord, Ray, Vector};

use super::Material;
use crate::onb::Onb;
use crate::textures::Texture;
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
    ) -> Option<(Vector, Ray, f32)> {
        let uvw = Onb::build_from_w(hit_record.normal);
        let dir = uvw.local(random_cosine_dir());

        let scattered = Ray::new(hit_record.p, dir.normalize(), r_in.time());

        Some((
            self.texture.value(hit_record.u, hit_record.v, hit_record.p),
            scattered,
            Vector::dot(uvw.w(), scattered.dir()) / PI,
        ))
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
