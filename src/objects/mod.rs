use crate::materials::Material;
use crate::{Ray, Vector};

use std::fmt::Debug;

mod scene;
mod sphere;

pub use scene::*;
pub use sphere::*;

#[derive(Debug, Copy, Clone)]
pub struct HitRecord {
    pub t: f32,
    pub p: Vector,
    pub normal: Vector,
}

impl HitRecord {
    pub fn new(t: f32, p: Vector, normal: Vector) -> Self {
        Self { t, p, normal }
    }
}

pub trait Hittable: Debug + Send + Sync {
    fn hit(
        &self,
        r: Ray,
        t_min: f32,
        t_max: f32,
    ) -> (bool, Option<(HitRecord, &Box<Material>)>);
}
