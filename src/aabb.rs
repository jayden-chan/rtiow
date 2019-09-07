use crate::{Ray, Vector};

pub struct Aabb {
    min: Vector,
    max: Vector,
}

impl Aabb {
    fn min(&self) -> Vector {
        self.min
    }

    fn max(&self) -> Vector {
        self.max
    }

    fn hit(&self, r: Ray, t_min: f32, t_max: f32) -> bool {
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
}
