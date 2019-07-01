use crate::{Ray, Vector};

pub struct Camera {
    lower_left_corner: Vector,
    horizontal: Vector,
    vertical: Vector,
    origin: Vector,
}

impl Camera {
    pub fn new() -> Self {
        Self {
            lower_left_corner: Vector::new(-2.0, -1.0, -1.0),
            horizontal: Vector::new(4.0, 0.0, 0.0),
            vertical: Vector::new(0.0, 2.0, 0.0),
            origin: Vector::new(0.0, 0.0, 0.0),
        }
    }
}

impl Camera {
    pub fn get_ray(&self, u: f32, v: f32) -> Ray {
        Ray::new(
            self.origin,
            self.lower_left_corner + self.horizontal * u + self.vertical * v
                - self.origin,
        )
    }
}
