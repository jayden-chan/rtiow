//! A simple Sphere object

use super::{HitRecord, Hittable};
use crate::materials::Material;
use crate::{Ray, Vector};

use std::f32;

#[derive(Debug)]
pub struct Sphere {
    center: Vector,
    radius: f32,
    material: Box<Material>,
}

impl Hittable for Sphere {
    fn hit(
        &self,
        r: Ray,
        t_min: f32,
        t_max: f32,
    ) -> Option<(HitRecord, &Box<Material>)> {
        let oc = r.origin() - self.center;

        let a = Vector::dot(r.dir(), r.dir());
        let b = Vector::dot(oc, r.dir());
        let c = Vector::dot(oc, oc) - self.radius * self.radius;
        let discriminant = b * b - a * c;

        if discriminant > 0.0 {
            let mut q_eq = (-b - f32::sqrt(discriminant)) / a;

            // If the minus variant is out of range try the plus one
            if q_eq >= t_max || q_eq <= t_min {
                q_eq = (-b + f32::sqrt(discriminant)) / a;
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
}

impl Sphere {
    pub fn new(center: Vector, radius: f32, material: Box<Material>) -> Self {
        Self {
            center,
            radius,
            material,
        }
    }
}
