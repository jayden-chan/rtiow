use crate::{Ray, Vector};

pub struct Camera {
    lower_left_corner: Vector,
    horizontal: Vector,
    vertical: Vector,
    origin: Vector,
}

impl Camera {
    pub fn new(width: f32, height: f32) -> Self {
        Self {
            lower_left_corner: Vector::new(
                -width / 100.0,
                -height / 100.0,
                -1.0,
            ),
            horizontal: Vector::new(width / 50.0, 0.0, 0.0),
            vertical: Vector::new(0.0, height / 50.0, 0.0),
            origin: Vector::zero(),
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
