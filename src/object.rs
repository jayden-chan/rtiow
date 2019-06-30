use crate::{Ray, Vector};

#[derive(Debug, Copy, Clone)]
pub struct HitRecord {
    pub t: Option<f32>,
    pub p: Option<Vector>,
    pub normal: Option<Vector>,
}

impl HitRecord {
    pub fn empty() -> Self {
        Self {
            t: None,
            p: None,
            normal: None,
        }
    }
}

pub trait Hittable {
    fn hit(
        &self,
        r: Ray,
        t_min: f32,
        t_max: f32,
        hit_record: &mut HitRecord,
    ) -> bool;
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
}

impl<T: Hittable> Hittable for ObjectList<T> {
    fn hit(
        &self,
        r: Ray,
        t_min: f32,
        t_max: f32,
        hit_record: &mut HitRecord,
    ) -> bool {
        let mut temp_rec = HitRecord::empty();
        let mut hit_anything = false;
        let mut closest_so_far = t_max;

        for i in &self.objects {
            if i.hit(r, t_min, closest_so_far, &mut temp_rec) {
                hit_anything = true;
                closest_so_far = temp_rec.t.unwrap();
                *hit_record = temp_rec;
            }
        }

        return hit_anything;
    }
}
