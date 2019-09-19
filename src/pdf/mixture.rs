use super::Pdf;
use crate::Vector;
use rand::prelude::*;

#[derive(Debug)]
pub struct Mixture<'a> {
    pub pdf1: &'a dyn Pdf,
    pub pdf2: &'a dyn Pdf,
}

impl<'a> Pdf for Mixture<'a> {
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
