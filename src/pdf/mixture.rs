use super::Pdf;
use crate::Vector;
use rand::prelude::*;

#[derive(Debug)]
pub struct Mixture<T: Pdf, U: Pdf> {
    pub pdf1: T,
    pub pdf2: U,
}

impl<T: Pdf, U: Pdf> Pdf for Mixture<T, U> {
    fn value(&self, dir: Vector) -> f32 {
        0.5 * self.pdf1.value(dir) + 0.5 * self.pdf2.value(dir)
    }

    fn generate(&self) -> Vector {
        if random::<f32>() < 0.5 {
            self.pdf1.generate()
        } else {
            self.pdf2.generate()
        }
    }
}
