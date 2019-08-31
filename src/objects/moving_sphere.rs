//! A simple moving Sphere object

use super::{HitRecord, Hittable};
use crate::materials::Material;
use crate::{Ray, Vector};

#[derive(Debug)]
pub struct MovingSphere {
    center0: Vector,
    center1: Vector,
    t0: f32,
    t1: f32,
    radius: f32,
    material: Box<dyn Material>,
}

impl Hittable for MovingSphere {
    fn hit(
        &self,
        r: Ray,
        t_min: f32,
        t_max: f32,
    ) -> Option<(HitRecord, &Box<dyn Material>)> {
        let oc = r.origin() - self.center(r.time());

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
                        (point_at_parameter - self.center(r.time()))
                            / self.radius,
                    ),
                    &self.material,
                ));
            }
        }

        None
    }
}

impl MovingSphere {
    pub fn new(
        center0: Vector,
        center1: Vector,
        t0: f32,
        t1: f32,
        radius: f32,
        material: Box<dyn Material>,
    ) -> Self {
        Self {
            center0,
            center1,
            t0,
            t1,
            radius,
            material,
        }
    }

    fn center(&self, time: f32) -> Vector {
        self.center0
            + ((time - self.t0) / (self.t1 - self.t0))
                * (self.center1 - self.center0)
    }
}
