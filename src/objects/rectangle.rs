use super::{HitRecord, Hittable};
use crate::{aabb::Aabb, materials::Material, Ray, Vector};

pub enum RectPlane {
    XY = 0,
    YZ = 1,
    ZX = 2,
}

#[derive(Debug)]
pub struct Rectangle<const P: RectPlane> {
    pub x0: f32,
    pub x1: f32,
    pub y0: f32,
    pub y1: f32,
    pub k: f32,
    pub norm: f32,
    pub material: Box<dyn Material>,
}

impl<const P: RectPlane> Hittable for Rectangle<{ P }> {
    fn hit(
        &self,
        r: Ray,
        t_min: f32,
        t_max: f32,
    ) -> Option<(HitRecord, &Box<dyn Material>)> {
        let (k_ax, a_ax, b_ax) = match P {
            RectPlane::XY => (2, 0, 1),
            RectPlane::YZ => (0, 1, 2),
            RectPlane::ZX => (1, 0, 2),
        };

        let t = (self.k - r.origin()[k_ax]) / r.dir()[k_ax];
        if t < t_min || t > t_max {
            return None;
        }

        let x = r.origin()[a_ax] + t * r.dir()[a_ax];
        let y = r.origin()[b_ax] + t * r.dir()[b_ax];

        if x < self.x0 || x > self.x1 || y < self.y0 || y > self.y1 {
            return None;
        }

        let normal = match P {
            RectPlane::XY => Vector::new(0.0, 0.0, 1.0),
            RectPlane::YZ => Vector::new(1.0, 0.0, 0.0),
            RectPlane::ZX => Vector::new(0.0, 1.0, 0.0),
        };

        Some((
            HitRecord {
                t,
                u: (x - self.x0) / (self.x1 - self.x0),
                v: (y - self.y0) / (self.y1 - self.y0),
                p: r.point_at_parameter(t),
                normal: normal * self.norm,
            },
            &self.material,
        ))
    }

    fn bounding_box(&self, _t0: f32, _t1: f32) -> Option<Aabb> {
        match P {
            RectPlane::XY => Some(Aabb::new(
                Vector::new(self.x0, self.y0, self.k - 0.0001),
                Vector::new(self.x0, self.y0, self.k + 0.0001),
            )),
            RectPlane::YZ => Some(Aabb::new(
                Vector::new(self.k - 0.0001, self.x0, self.y0),
                Vector::new(self.k + 0.0001, self.x0, self.y0),
            )),
            RectPlane::ZX => Some(Aabb::new(
                Vector::new(self.x0, self.k - 0.0001, self.y0),
                Vector::new(self.x0, self.k + 0.0001, self.y0),
            )),
        }
    }
}
