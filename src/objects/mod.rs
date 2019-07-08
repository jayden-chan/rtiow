use crate::materials::Material;
use crate::{Ray, Vector};

use std::fmt::Debug;

mod moving_sphere;
mod scene;
mod sphere;

pub use moving_sphere::*;
pub use scene::*;
pub use sphere::*;

/// A HitRecord describes an interaction between an incoming [`Ray`]
/// and an object.
///
/// [`Ray`]: struct.Ray.html
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

/// The Hittable trait describes any object in the scene that a light
/// ray can interact with. All items rendered on the screen implement the
/// Hittable trait.
pub trait Hittable: Debug + Send + Sync {
    /// A method for returning whether a given ray intersects this object
    ///
    /// Returns whether the given ray intersects this object. If it does,
    /// a tuple containing the [`HitRecord`] for the
    /// intersection as well as the material of the object that was hit.
    ///
    /// [`HitRecord`]: trait.HitRecord.html
    fn hit(
        &self,
        r: Ray,
        t_min: f32,
        t_max: f32,
    ) -> Option<(HitRecord, &Box<Material>)>;
}
