use crate::Vector;
use std::fmt::Debug;

mod constant;
pub use constant::*;

pub trait TextureClone {
    fn clone_box(&self) -> Box<dyn Texture>;
}

impl<T> TextureClone for T
where
    T: 'static + Texture + Clone,
{
    fn clone_box(&self) -> Box<dyn Texture> {
        Box::new(self.clone())
    }
}

impl Clone for Box<dyn Texture> {
    fn clone(&self) -> Box<dyn Texture> {
        self.clone_box()
    }
}

pub trait Texture: Debug + Send + Sync + TextureClone {
    fn value(&self, u: f32, v: f32, p: Vector) -> Vector;
}
