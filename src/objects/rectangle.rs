use super::{HitRecord, Hittable};
use crate::{aabb::Aabb, materials::Material, Ray, Vector};

#[derive(Debug)]
pub struct Rectangle {
    pub x0: f32,
    pub x1: f32,
    pub y0: f32,
    pub y1: f32,
    pub k: f32,
    pub norm: f32,
    pub material: Box<dyn Material>,
}

impl Hittable for Rectangle {
    fn hit(
        &self,
        r: Ray,
        t_min: f32,
        t_max: f32,
    ) -> Option<(HitRecord, &Box<dyn Material>)> {
        let t = (self.k - r.origin().z) / r.dir().z;
        if t < t_min || t > t_max {
            return None;
        }

        let x = r.origin().x + t * r.dir().x;
        let y = r.origin().y + t * r.dir().y;

        if x < self.x0 || x > self.x1 || y < self.y0 || y > self.y1 {
            return None;
        }

        Some((
            HitRecord {
                t,
                u: (x - self.x0) / (self.x1 - self.x0),
                v: (y - self.y0) / (self.y1 - self.y0),
                p: r.point_at_parameter(t),
                normal: Vector::new(0.0, 0.0, 1.0) * self.norm,
            },
            &self.material,
        ))
    }

    fn bounding_box(&self, _t0: f32, _t1: f32) -> Option<Aabb> {
        Some(Aabb::new(
            Vector::new(self.x0, self.y0, self.k - 0.0001),
            Vector::new(self.x0, self.y0, self.k + 0.0001),
        ))
    }
}
