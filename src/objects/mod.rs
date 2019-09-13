use crate::aabb::Aabb;
use crate::materials::Material;
use crate::{Ray, Vector};

use std::fmt::Debug;

mod block;
mod moving_sphere;
mod rectangle;
mod scene;
mod sphere;

pub use block::*;
pub use moving_sphere::*;
pub use rectangle::*;
pub use scene::*;
pub use sphere::*;

/// A HitRecord describes an interaction between an incoming [`Ray`]
/// and an object.
///
/// [`Ray`]: struct.Ray.html
#[derive(Debug, Copy, Clone)]
pub struct HitRecord {
    pub u: f32,
    pub v: f32,
    pub t: f32,
    pub p: Vector,
    pub normal: Vector,
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
    ) -> Option<(HitRecord, &Box<dyn Material>)>;

    /// A method for returning whether the hittable object can be
    /// contained within a bounding box. If it can, the [`AABB`] is returned.
    ///
    /// [`AABB`]: trait.Aabb.html
    fn bounding_box(&self, t0: f32, t1: f32) -> Option<Aabb>;
}
