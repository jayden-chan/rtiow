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
