use crate::Vector;

#[derive(Debug, Clone)]
pub struct ConstantTexture {
    color: Vector,
}

use super::Texture;

impl Texture for ConstantTexture {
    fn value(&self, _u: f32, _v: f32, _p: Vector) -> Vector {
        self.color
    }
}

impl ConstantTexture {
    pub fn new(color: Vector) -> Self {
        Self { color }
    }
}
