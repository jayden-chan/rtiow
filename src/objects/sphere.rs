//! A simple Sphere object

use super::{HitRecord, Hittable};
use crate::{
    aabb::Aabb, materials::Material, onb::Onb, util::sphere_uv, Ray, Vector,
};

use rand::prelude::*;
use std::f32;

#[derive(Debug)]
pub struct Sphere {
    center: Vector,
    radius: f32,
    material: Box<dyn Material>,
}

fn random_to_sphere(radius: f32, dist_squared: f32) -> Vector {
    let mut rng = rand::thread_rng();
    let r1: f32 = rng.gen();
    let r2: f32 = rng.gen();

    let z = 1.0 + r2 * (f32::sqrt(1.0 - radius * radius / dist_squared) - 1.0);
    let phi = 2.0 * f32::consts::PI * r1;
    let x = f32::cos(phi) * f32::sqrt(1.0 - z * z);
    let y = f32::sin(phi) * f32::sqrt(1.0 - z * z);

    Vector::new(x, y, z)
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
                let (u, v) =
                    sphere_uv((point_at_parameter - self.center) / self.radius);
                return Some((
                    HitRecord {
                        u,
                        v,
                        t: q_eq,
                        p: point_at_parameter,
                        normal: (point_at_parameter - self.center)
                            / self.radius,
                    },
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

    fn pdf_value(&self, o: Vector, v: Vector) -> f32 {
        if let Some((_, _)) = self.hit(Ray::new(o, v, 0.0), 0.001, f32::MAX) {
            let cos_theta_max = f32::sqrt(
                1.0 - self.radius * self.radius
                    / (self.center - o).length_squared(),
            );
            let solid_angle = 2.0 * f32::consts::PI * (1.0 - cos_theta_max);

            return 1.0 / solid_angle;
        } else {
            return 0.0;
        }
    }

    fn random(&self, o: Vector) -> Vector {
        let dir = self.center - o;
        let dist_squared = dir.length_squared();
        let uvw = Onb::build_from_w(dir);
        uvw.local(random_to_sphere(self.radius, dist_squared))
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
