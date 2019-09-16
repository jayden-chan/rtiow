use crate::Vector;
use std::ops;

#[derive(Debug)]
pub struct Onb {
    axis: [Vector; 3],
}

impl Onb {
    pub fn u(&self) -> Vector {
        self.axis[0]
    }

    pub fn v(&self) -> Vector {
        self.axis[1]
    }

    pub fn w(&self) -> Vector {
        self.axis[2]
    }

    pub fn local(&self, a: Vector) -> Vector {
        a.x * self.u() + a.y * self.v() + a.z * self.w()
    }
}

impl Onb {
    pub fn build_from_w(n: Vector) -> Self {
        let mut axis = [Vector::zeros(); 3];
        axis[2] = n.normalize();

        let a = if f32::abs(axis[2].x) > 0.9 {
            Vector::new(0.0, 1.0, 0.0)
        } else {
            Vector::new(1.0, 0.0, 0.0)
        };

        axis[1] = Vector::cross(axis[2], a).normalize();
        axis[0] = Vector::cross(axis[2], axis[1]);

        Self { axis }
    }
}

impl ops::Index<usize> for Onb {
    type Output = Vector;
    fn index(&self, idx: usize) -> &Self::Output {
        &self.axis[idx]
    }
}
