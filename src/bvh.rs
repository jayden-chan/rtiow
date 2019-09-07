use crate::aabb::Aabb;
use crate::materials::Material;
use crate::Ray;
use crate::{HitRecord, Hittable};

#[derive(Debug)]
pub struct Bvh {
    left: Box<dyn Hittable>,
    right: Box<dyn Hittable>,
    bounding_box: Aabb,
}

impl Hittable for Bvh {
    fn hit(
        &self,
        r: Ray,
        t_min: f32,
        t_max: f32,
    ) -> Option<(HitRecord, &Box<dyn Material>)> {
        if self.bounding_box.hit(r, t_min, t_max) {
            let hit_left = self.left.hit(r, t_min, t_max);
            let hit_right = self.right.hit(r, t_min, t_max);

            if hit_left.is_some() && hit_right.is_some() {
                let (rec_left, mat_left) = hit_left.unwrap();
                let (rec_right, mat_right) = hit_right.unwrap();

                if rec_left.t < rec_right.t {
                    return Some((rec_left, mat_left));
                } else {
                    return Some((rec_right, mat_right));
                }
            } else if hit_left.is_some() {
                let (rec_left, mat_left) = hit_left.unwrap();
                return Some((rec_left, mat_left));
            } else if hit_right.is_some() {
                let (rec_right, mat_right) = hit_right.unwrap();
                return Some((rec_right, mat_right));
            }

            return None;
        }

        None
    }

    fn bounding_box(&self, _t0: f32, _t1: f32) -> Option<Aabb> {
        Some(self.bounding_box)
    }
}
