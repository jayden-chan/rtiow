use crate::{Ray, Vector};

pub struct Aabb {
    min: Vector,
    max: Vector,
}

impl Aabb {
    pub fn min(&self) -> Vector {
        self.min
    }

    pub fn max(&self) -> Vector {
        self.max
    }

    pub fn hit(&self, r: Ray, t_min: f32, t_max: f32) -> bool {
        for a in 0..3 {
            let t0 = f32::min(
                (self.min[a] - r.origin()[a]) / r.dir()[a],
                (self.max[a] - r.origin()[a]) / r.dir()[a],
            );

            let t1 = f32::max(
                (self.min[a] - r.origin()[a]) / r.dir()[a],
                (self.max[a] - r.origin()[a]) / r.dir()[a],
            );

            let t_min = f32::max(t0, t_min);
            let t_max = f32::min(t1, t_max);

            if t_max <= t_min {
                return false;
            }
        }

        true
    }
}

impl Aabb {
    pub fn new(a: Vector, b: Vector) -> Self {
        Self { min: a, max: b }
    }

    pub fn surrounding_box(box0: Self, box1: Self) -> Self {
        let small = Vector::new(
            f32::min(box0.min().x, box1.min().x),
            f32::min(box0.min().y, box1.min().y),
            f32::min(box0.min().z, box1.min().z),
        );

        let big = Vector::new(
            f32::max(box0.max().x, box1.max().x),
            f32::max(box0.max().y, box1.max().y),
            f32::max(box0.max().z, box1.max().z),
        );

        Self {
            min: small,
            max: big,
        }
    }
}
