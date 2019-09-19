use crate::{
    materials::{Material, ScatterRecord},
    textures::Texture,
    HitRecord, Ray, Vector,
};

#[derive(Debug, Clone)]
pub struct DiffuseLight {
    emit: Box<dyn Texture>,
}

impl Material for DiffuseLight {
    fn scatter(
        &self,
        _r_in: Ray,
        _hit_record: HitRecord,
    ) -> Option<ScatterRecord> {
        None
    }

    fn emitted(&self, r_in: Ray, hit_record: HitRecord) -> Vector {
        let (u, v, p) = (hit_record.u, hit_record.v, hit_record.p);
        if Vector::dot(hit_record.normal, r_in.dir()) < 0.0 {
            self.emit.value(u, v, p)
        } else {
            Vector::zeros()
        }
    }
}

impl DiffuseLight {
    pub fn new(emit: Box<dyn Texture>) -> Self {
        Self { emit }
    }
}
