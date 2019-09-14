use super::{HitRecord, Hittable};
use crate::{aabb::Aabb, materials::Material, Ray, Vector};

#[derive(Debug)]
pub struct Translate {
    pub hittable: Box<dyn Hittable>,
    pub offset: Vector,
}

impl Hittable for Translate {
    fn hit(
        &self,
        r: Ray,
        t_min: f32,
        t_max: f32,
    ) -> Option<(HitRecord, &Box<dyn Material>)> {
        let moved_r = Ray::new(r.origin() - self.offset, r.dir(), r.time());

        self.hittable.hit(moved_r, t_min, t_max).map(
            |(mut hit_record, material)| {
                hit_record.p += self.offset;
                (hit_record, material)
            },
        )
    }

    fn bounding_box(&self, t0: f32, t1: f32) -> Option<Aabb> {
        self.hittable.bounding_box(t0, t1).map(|mut aabb| {
            aabb.min += self.offset;
            aabb.max += self.offset;
            aabb
        })
    }
}
