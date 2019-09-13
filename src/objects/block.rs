use super::{HitRecord, Hittable, RectPlane, Rectangle, Scene};
use crate::{aabb::Aabb, materials::Material, Ray, Vector};

#[derive(Debug)]
pub struct Block {
    p_min: Vector,
    p_max: Vector,
    sides: Scene,
}

impl Hittable for Block {
    fn hit(
        &self,
        r: Ray,
        t_min: f32,
        t_max: f32,
    ) -> Option<(HitRecord, &Box<dyn Material>)> {
        self.sides.hit(r, t_min, t_max)
    }

    fn bounding_box(&self, _t0: f32, _t1: f32) -> Option<Aabb> {
        Some(Aabb::new(self.p_min, self.p_max))
    }
}

impl Block {
    pub fn new(p0: Vector, p1: Vector, material: Box<dyn Material>) -> Self {
        let objects: Vec<Box<dyn Hittable>> = vec![
            Box::new(Rectangle::<{ RectPlane::XY }> {
                a0: p0.x,
                a1: p1.x,
                b0: p0.y,
                b1: p1.y,
                k: p1.z,
                norm: 1.0,
                material: material.clone(),
            }),
            Box::new(Rectangle::<{ RectPlane::XY }> {
                a0: p0.x,
                a1: p1.x,
                b0: p0.y,
                b1: p1.y,
                k: p0.z,
                norm: -1.0,
                material: material.clone(),
            }),
            Box::new(Rectangle::<{ RectPlane::XZ }> {
                a0: p0.x,
                a1: p1.x,
                b0: p0.z,
                b1: p1.z,
                k: p1.y,
                norm: 1.0,
                material: material.clone(),
            }),
            Box::new(Rectangle::<{ RectPlane::XZ }> {
                a0: p0.x,
                a1: p1.x,
                b0: p0.z,
                b1: p1.z,
                k: p0.y,
                norm: -1.0,
                material: material.clone(),
            }),
            Box::new(Rectangle::<{ RectPlane::YZ }> {
                a0: p0.y,
                a1: p1.y,
                b0: p0.z,
                b1: p1.z,
                k: p1.x,
                norm: 1.0,
                material: material.clone(),
            }),
            Box::new(Rectangle::<{ RectPlane::YZ }> {
                a0: p0.y,
                a1: p1.y,
                b0: p0.z,
                b1: p1.z,
                k: p0.x,
                norm: -1.0,
                material: material.clone(),
            }),
        ];

        Self {
            p_min: p0,
            p_max: p1,
            sides: Scene::from_objects(objects, 0.0),
        }
    }
}
