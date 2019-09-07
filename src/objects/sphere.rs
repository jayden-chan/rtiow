//! A simple Sphere object

use super::{HitRecord, Hittable};
use crate::aabb::Aabb;
use crate::materials::Material;
use crate::{Ray, Vector};

#[derive(Debug)]
pub struct Sphere {
    center: Vector,
    radius: f32,
    material: Box<dyn Material>,
}

impl Hittable for Sphere {
    fn hit(
        &self,
        r: Ray,
        t_min: f32,
        t_max: f32,
    ) -> Option<(HitRecord, &Box<dyn Material>)> {
        let oc = r.origin() - self.center;

        let a = Vector::dot(r.dir(), r.dir());
        let b = Vector::dot(oc, r.dir());
        let c = Vector::dot(oc, oc) - self.radius * self.radius;
        let discriminant = b * b - a * c;

        if discriminant > 0.0 {
            let mut q_eq = (-b - discriminant.sqrt()) / a;

            // If the minus variant is out of range try the plus one
            if q_eq >= t_max || q_eq <= t_min {
                q_eq = (-b + discriminant.sqrt()) / a;
            }

            if q_eq < t_max && q_eq > t_min {
                let point_at_parameter = r.point_at_parameter(q_eq);
                return Some((
                    HitRecord::new(
                        q_eq,
                        point_at_parameter,
                        (point_at_parameter - self.center) / self.radius,
                    ),
                    &self.material,
                ));
            }
        }

        None
    }

    fn bounding_box(&self, _t0: f32, _t1: f32) -> Option<Aabb> {
        Some(Aabb::new(
            self.center - Vector::new(self.radius, self.radius, self.radius),
            self.center + Vector::new(self.radius, self.radius, self.radius),
        ))
    }
}

impl Sphere {
    pub fn new(
        center: Vector,
        radius: f32,
        material: Box<dyn Material>,
    ) -> Self {
        Self {
            center,
            radius,
            material,
        }
    }
}
