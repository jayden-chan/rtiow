use crate::materials::Material;
use crate::Ray;

use super::{HitRecord, Hittable};

pub struct World {
    objects: Vec<Box<Hittable>>,
}

impl World {
    pub fn from_objects(objects: Vec<Box<Hittable>>) -> Self {
        World { objects }
    }
}

impl Hittable for World {
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
