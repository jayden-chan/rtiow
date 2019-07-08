use crate::Vector;

/// A Ray represents a single beam of light
#[derive(Debug, Copy, Clone)]
#[allow(non_snake_case)]
pub struct Ray {
    A: Vector,
    B: Vector,
    time: f32,
}

impl Ray {
    #[allow(non_snake_case)]
    pub fn new(A: Vector, B: Vector, time: f32) -> Self {
        Self { A, B, time }
    }
}

impl Ray {
    #[inline]
    pub fn origin(self) -> Vector {
        self.A
    }

    #[inline]
    pub fn dir(self) -> Vector {
        self.B
    }

    #[inline]
    pub fn point_at_parameter(self, t: f32) -> Vector {
        self.A + self.B * t
    }

    #[inline]
    pub fn time(self) -> f32 {
        self.time
    }
}
