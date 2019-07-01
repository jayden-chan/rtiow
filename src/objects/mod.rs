use crate::materials::Material;
use crate::{Ray, Vector};

use std::fmt::Debug;

mod sphere;
mod world;

pub use sphere::*;
pub use world::*;

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
