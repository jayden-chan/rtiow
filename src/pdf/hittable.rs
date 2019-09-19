use super::Pdf;
use crate::{Hittable, Vector};

#[derive(Debug)]
pub struct HittablePDF<'a> {
    pub o: Vector,
    pub inner: &'a dyn Hittable,
}

impl<'a> Pdf for HittablePDF<'a> {
    fn value(&self, dir: Vector) -> f32 {
        self.inner.pdf_value(self.o, dir)
    }

    fn generate(&self) -> Vector {
        self.inner.random(self.o)
    }
}
