use super::Pdf;
use crate::{Hittable, Vector};

#[derive(Debug)]
pub struct HittablePDF {
    pub o: Vector,
    pub inner: Box<dyn Hittable>,
}

impl Pdf for HittablePDF {
    fn value(&self, dir: Vector) -> f32 {
        self.inner.pdf_value(self.o, dir)
    }

    fn generate(&self) -> Vector {
        self.inner.random(self.o)
    }
}
