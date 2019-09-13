use crate::util::random_in_unit_sphere;
use crate::{HitRecord, Ray, Vector};

use super::Material;
use crate::textures::Texture;

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
    ) -> Option<(Vector, Ray)> {
        let target = hit_record.p + hit_record.normal + random_in_unit_sphere();
        Some((
            self.texture.value(hit_record.u, hit_record.v, hit_record.p),
            Ray::new(hit_record.p, target - hit_record.p, r_in.time()),
        ))
    }
}
