use super::Material;
use crate::{textures::Texture, HitRecord, Ray, Vector};

#[derive(Debug)]
pub struct DiffuseLight {
    emit: Box<dyn Texture>,
}

impl Material for DiffuseLight {
    fn scatter(
        &self,
        r_in: Ray,
        hit_record: HitRecord,
    ) -> Option<(Vector, Ray)> {
        None
    }

    fn emitted(&self, u: f32, v: f32, p: Vector) -> Vector {
        self.emit.value(u, v, p)
    }
}

impl DiffuseLight {
    pub fn new(emit: Box<dyn Texture>) -> Self {
        Self { emit }
    }
}
