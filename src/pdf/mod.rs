use crate::Vector;

use std::fmt::Debug;

mod cosine;
pub use cosine::*;

mod hittable;
pub use hittable::*;

mod mixture;
pub use mixture::*;

pub trait Pdf: Debug + Send + Sync {
    fn value(&self, dir: Vector) -> f32;
    fn generate(&self) -> Vector;
}

impl Pdf for Box<dyn Pdf> {
    fn value(&self, dir: Vector) -> f32 {
        (**self).value(dir)
    }
    fn generate(&self) -> Vector {
        (**self).generate()
    }
}
