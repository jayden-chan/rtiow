use crate::object::{HitRecord, Hittable};
use crate::{Ray, Vector};
use std::f32;

#[derive(Debug, Copy, Clone)]
pub struct Sphere {
    center: Vector,
    radius: f32,
}

impl Hittable for Sphere {
    fn hit(
        &self,
        r: Ray,
        t_min: f32,
        t_max: f32,
        hit_record: &mut HitRecord,
    ) -> bool {
        let oc = r.origin() - self.center;

        let a = Vector::dot(r.dir(), r.dir());
        let b = Vector::dot(oc, r.dir());
        let c = Vector::dot(oc, oc) - self.radius * self.radius;
        let discriminant = b * b - a * c;

        if discriminant > 0.0 {
            let q_eq = (-b - f32::sqrt(discriminant)) / a;

            if q_eq < t_max && q_eq > t_min {
                let point_at_parameter = r.point_at_parameter(q_eq);
                hit_record.t = Some(q_eq);
                hit_record.p = Some(point_at_parameter);
                hit_record.normal =
                    Some((point_at_parameter - self.center) / self.radius);

                return true;
            }

            let q_eq = (-b + f32::sqrt(discriminant)) / a;

            if q_eq < t_max && q_eq > t_min {
                let point_at_parameter = r.point_at_parameter(q_eq);
                hit_record.t = Some(q_eq);
                hit_record.p = Some(point_at_parameter);
                hit_record.normal =
                    Some((point_at_parameter - self.center) / self.radius);

                return true;
            }
        }

        return false;
    }
}

impl Sphere {
    pub fn new(center: Vector, radius: f32) -> Self {
        Self { center, radius }
    }
}
