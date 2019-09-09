use crate::aabb::Aabb;
use crate::materials::Material;
use crate::Ray;
use crate::{HitRecord, Hittable};

use rand::prelude::*;
use std::cmp::Ordering;
use std::f32;

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
            match (
                self.left.hit(r, t_min, t_max),
                self.right.hit(r, t_min, t_max),
            ) {
                (None, None) => None,
                (Some((rec_left, mat_left)), None) => {
                    Some((rec_left, mat_left))
                }
                (None, Some((rec_right, mat_right))) => {
                    Some((rec_right, mat_right))
                }
                (Some((rec_left, mat_left)), Some((rec_right, mat_right))) => {
                    return if rec_left.t < rec_right.t {
                        Some((rec_left, mat_left))
                    } else {
                        Some((rec_right, mat_right))
                    };
                }
            };
        }

        None
    }

    fn bounding_box(&self, _t0: f32, _t1: f32) -> Option<Aabb> {
        Some(self.bounding_box)
    }
}

impl Bvh {
    pub fn new(
        objects: &mut Vec<Box<dyn Hittable>>,
        t0: f32,
        t1: f32,
    ) -> Box<dyn Hittable> {
        let axis = (3f32 * random::<f32>()) as u32;

        match axis {
            0 => {
                objects.sort_unstable_by(|a, b| {
                    let box_left = a.bounding_box(0.0, 0.0).unwrap();
                    let box_right = b.bounding_box(0.0, 0.0).unwrap();

                    if box_left.min().x - box_right.min().x < 0.0 {
                        return Ordering::Less;
                    } else {
                        return Ordering::Greater;
                    }
                });
            }
            1 => {
                objects.sort_unstable_by(|a, b| {
                    let box_left = a.bounding_box(0.0, 0.0).unwrap();
                    let box_right = b.bounding_box(0.0, 0.0).unwrap();

                    if box_left.min().y - box_right.min().y < 0.0 {
                        return Ordering::Less;
                    } else {
                        return Ordering::Greater;
                    }
                });
            }
            _ => {
                objects.sort_unstable_by(|a, b| {
                    let box_left = a.bounding_box(0.0, 0.0).unwrap();
                    let box_right = b.bounding_box(0.0, 0.0).unwrap();

                    if box_left.min().y - box_right.min().y < 0.0 {
                        return Ordering::Less;
                    } else {
                        return Ordering::Greater;
                    }
                });
            }
        }

        let left: Box<dyn Hittable>;
        let right: Box<dyn Hittable>;

        match objects.len() {
            0 => panic!("wrong bvh length"),
            1 => return objects.remove(0),
            l => {
                let l_vec = objects;
                let mut r_vec = l_vec.split_off(l / 2);
                let leftb = Self::new(l_vec, t0, t1);
                let rightb = Self::new(&mut r_vec, t0, t1);

                left = leftb;
                right = rightb;
            }
        };

        let box_left = left.bounding_box(t0, t1).unwrap();
        let box_right = right.bounding_box(t0, t1).unwrap();

        let bounding_box = Aabb::surrounding_box(box_left, box_right);
        return Box::new(Self {
            left: left,
            right: right,
            bounding_box,
        });
    }
}
