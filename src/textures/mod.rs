use crate::Vector;
use std::fmt::Debug;

mod constant;
pub use constant::*;

pub trait Texture: Debug + Send + Sync {
    fn value(&self, u: f32, v: f32, p: Vector) -> Vector;
}
