use crate::materials::Material;
use crate::{Ray, Vector};

mod sphere;
pub use sphere::*;

#[derive(Debug, Copy, Clone)]
pub struct HitRecord<T: Material> {
    pub t: f32,
    pub p: Vector,
    pub normal: Vector,
    pub material: T,
}

impl<T: Material> HitRecord<T> {
    pub fn new(t: f32, p: Vector, normal: Vector, material: T) -> Self {
        Self {
            t,
            p,
            normal,
            material,
        }
    }
}

pub trait Hittable<T: Material> {
    fn hit(
        &self,
        r: Ray,
        t_min: f32,
        t_max: f32,
    ) -> (bool, Option<HitRecord<T>>);
}

pub struct ObjectList<M: Material, T: Hittable<M>> {
    objects: Vec<T>,
}

impl<M: Material, T: Hittable<M>> ObjectList<M, T> {
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

impl<M: Material, T: Hittable<M>> Hittable<M> for ObjectList<M, T> {
    fn hit(
        &self,
        r: Ray,
        t_min: f32,
        t_max: f32,
    ) -> (bool, Option<HitRecord<M>>) {
        let mut hit_record = None;
        let mut hit_anything = false;
        let mut closest_so_far = t_max;

        for i in &self.objects {
            let (hit, hr) = i.hit(r, t_min, closest_so_far);
            if hit {
                hit_record = Some(hr.unwrap());
                hit_anything = true;
                closest_so_far = hr.unwrap().t;
            }
        }

        (hit_anything, hit_record)
    }
}
