use crate::materials::Material;
use crate::{Ray, Vector};

mod sphere;
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

pub trait Hittable {
    fn hit(
        &self,
        r: Ray,
        t_min: f32,
        t_max: f32,
    ) -> (bool, Option<(HitRecord, &Box<Material>)>);
}

pub struct ObjectList<T: Hittable> {
    objects: Vec<T>,
}

impl<T: Hittable> ObjectList<T> {
    pub fn from_objects(objects: Vec<T>) -> Self {
        Self { objects }
    }

    pub fn new() -> Self {
        Self {
            objects: Vec::new(),
        }
    }

    pub fn push(&mut self, object: T) {
        self.objects.push(object);
    }
}

impl<T: Hittable> Hittable for ObjectList<T> {
    fn hit(
        &self,
        r: Ray,
        t_min: f32,
        t_max: f32,
    ) -> (bool, Option<(HitRecord, &Box<Material>)>) {
        let mut result = None;
        let mut hit_anything = false;
        let mut closest_so_far = t_max;

        for i in &self.objects {
            let (hit, res) = i.hit(r, t_min, closest_so_far);
            if hit {
                let (hr, mat) = res.unwrap();
                result = Some((hr, mat));
                hit_anything = true;
                closest_so_far = hr.t;
            }
        }

        (hit_anything, result)
    }
}
