use super::{HitRecord, Hittable};
use crate::materials::Material;
use crate::{Ray, Vector};
use std::f32;

#[derive(Debug, Copy, Clone)]
pub struct Sphere<T: Material> {
    center: Vector,
    radius: f32,
    material: T,
}

impl<T: Material> Hittable for Sphere<T> {
    fn hit(&self, r: Ray, t_min: f32, t_max: f32) -> (bool, Option<HitRecord>) {
        let oc = r.origin() - self.center;

        let a = Vector::dot(r.dir(), r.dir());
        let b = Vector::dot(oc, r.dir());
        let c = Vector::dot(oc, oc) - self.radius * self.radius;
        let discriminant = b * b - a * c;

        if discriminant > 0.0 {
            let q_eq = (-b - f32::sqrt(discriminant)) / a;

            if q_eq < t_max && q_eq > t_min {
                let point_at_parameter = r.point_at_parameter(q_eq);
                return (
                    true,
                    Some(HitRecord::new(
                        q_eq,
                        point_at_parameter,
                        (point_at_parameter - self.center) / self.radius,
                    )),
                );
            }

            let q_eq = (-b + f32::sqrt(discriminant)) / a;

            if q_eq < t_max && q_eq > t_min {
                let point_at_parameter = r.point_at_parameter(q_eq);
                return (
                    true,
                    Some(HitRecord::new(
                        q_eq,
                        point_at_parameter,
                        (point_at_parameter - self.center) / self.radius,
                    )),
                );
            }
        }

        (false, None)
    }
}

impl<T: Material> Sphere<T> {
    pub fn new(center: Vector, radius: f32, material: T) -> Self {
        Self {
            center,
            radius,
            material,
        }
    }
}
