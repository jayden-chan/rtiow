use super::{HitRecord, Hittable};
use crate::{aabb::Aabb, materials::Material, Ray, Vector};
use std::f32;

pub enum RotationAxis {
    X,
    Y,
    Z,
}

#[derive(Debug)]
pub struct Rotate<const A: RotationAxis> {
    hittable: Box<dyn Hittable>,
    sin_theta: f32,
    cos_theta: f32,
    bounding_box: Option<Aabb>,
}

impl<const A: RotationAxis> Hittable for Rotate<{ A }> {
    fn hit(
        &self,
        r: Ray,
        t_min: f32,
        t_max: f32,
    ) -> Option<(HitRecord, &Box<dyn Material>)> {
        let (_, a_axis, b_axis) = match A {
            RotationAxis::X => (0, 1, 2),
            RotationAxis::Y => (1, 2, 0),
            RotationAxis::Z => (2, 0, 1),
        };

        let mut origin = r.origin();
        let mut dir = r.dir();

        origin[a_axis] = self.sin_theta * r.origin()[b_axis]
            + self.cos_theta * r.origin()[a_axis];
        origin[b_axis] = self.cos_theta * r.origin()[b_axis]
            - self.sin_theta * r.origin()[a_axis];

        dir[a_axis] =
            self.sin_theta * r.dir()[b_axis] + self.cos_theta * r.dir()[a_axis];
        dir[b_axis] =
            self.cos_theta * r.dir()[b_axis] - self.sin_theta * r.dir()[a_axis];

        let rotated_r = Ray::new(origin, dir, r.time());

        self.hittable.hit(rotated_r, t_min, t_max).map(
            |(mut hit_record, material)| {
                let mut p = hit_record.p;
                let mut normal = hit_record.normal;

                p[a_axis] = self.cos_theta * hit_record.p[b_axis]
                    + self.sin_theta * hit_record.p[a_axis];
                p[b_axis] = -self.sin_theta * hit_record.p[b_axis]
                    + self.cos_theta * hit_record.p[a_axis];

                normal[a_axis] = self.cos_theta * hit_record.normal[b_axis]
                    + self.sin_theta * hit_record.normal[a_axis];
                normal[b_axis] = -self.sin_theta * hit_record.normal[b_axis]
                    - self.cos_theta * hit_record.normal[a_axis];

                hit_record.p = p;
                hit_record.normal = normal;
                (hit_record, material)
            },
        )
    }

    fn bounding_box(&self, _t0: f32, _t1: f32) -> Option<Aabb> {
        self.bounding_box
    }
}

impl<const A: RotationAxis> Rotate<{ A }> {
    pub fn new(hittable: Box<dyn Hittable>, angle: f32) -> Self {
        let rads = (f32::consts::PI / 180.0) * angle;
        let sin_theta = f32::sin(rads);
        let cos_theta = f32::cos(rads);

        let (r_axis, a_axis, b_axis) = match A {
            RotationAxis::X => (0, 1, 2),
            RotationAxis::Y => (1, 2, 0),
            RotationAxis::Z => (2, 0, 1),
        };

        let bounding_box = hittable.bounding_box(0.0, 1.0).map(|mut bbox| {
            let mut min = Vector::new(f32::MAX, f32::MAX, f32::MAX);
            let mut max = Vector::new(-f32::MAX, -f32::MAX, -f32::MAX);
            for i in 0..2 {
                for j in 0..2 {
                    for k in 0..2 {
                        let b = i as f32 * bbox.max[b_axis]
                            + (1 - i) as f32 * bbox.min[b_axis];
                        let r = j as f32 * bbox.max[r_axis]
                            + (1 - j) as f32 * bbox.min[r_axis];
                        let a = k as f32 * bbox.max[a_axis]
                            + (1 - k) as f32 * bbox.min[a_axis];

                        let new_b = cos_theta * b + sin_theta * a;
                        let new_a = -sin_theta * b + cos_theta * a;

                        if new_a < min[a_axis] {
                            min[a_axis] = new_a
                        }
                        if new_b < min[b_axis] {
                            min[b_axis] = new_b
                        }
                        if r < min[r_axis] {
                            min[r_axis] = r
                        }

                        if new_a > max[a_axis] {
                            max[a_axis] = new_a
                        }
                        if new_b > max[b_axis] {
                            max[b_axis] = new_b
                        }
                        if r > max[r_axis] {
                            max[r_axis] = r
                        }
                    }
                }
            }
            bbox.min = min;
            bbox.max = max;
            bbox
        });

        Self {
            hittable,
            sin_theta,
            cos_theta,
            bounding_box,
        }
    }
}
