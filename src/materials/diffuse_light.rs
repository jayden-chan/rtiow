use super::Material;
use crate::{textures::Texture, HitRecord, Ray, Vector};

#[derive(Debug, Clone)]
pub struct DiffuseLight {
    emit: Box<dyn Texture>,
}

impl Material for DiffuseLight {
    fn scatter(
        &self,
        _r_in: Ray,
        _hit_record: HitRecord,
    ) -> Option<(Vector, Ray, f32)> {
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
