use super::Pdf;
use crate::{onb::Onb, util::random_cosine_dir, Vector};
use std::f32::consts::PI;

#[derive(Debug)]
pub struct Cosine {
    uvw: Onb,
}

impl Pdf for Cosine {
    fn value(&self, dir: Vector) -> f32 {
        let dot = Vector::dot(dir.normalize(), self.uvw.w());
        return if dot > 0.0 { dot / PI } else { 0.0 };
    }

    fn generate(&self) -> Vector {
        self.uvw.local(random_cosine_dir())
    }
}

impl Cosine {
    pub fn new(w: Vector) -> Self {
        Self {
            uvw: Onb::build_from_w(w),
        }
    }
}
