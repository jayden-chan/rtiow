use crate::util::fast_inv_sqrt;

#[derive(Debug)]
pub struct Vector {
    x: f32,
    y: f32,
    z: f32,
}

impl Vector {
    /// Set the length of the vector to 1, maintaining its direction
    pub fn normalize(self) -> Self {
        let mag_inv = fast_inv_sqrt(
            (self.x * self.x) + (self.y * self.y) + (self.z * self.z),
        );

        Self {
            x: self.x * mag_inv,
            y: self.y * mag_inv,
            z: self.z * mag_inv,
        }
    }

    /// Create a new vector
    pub fn new(x: f32, y: f32, z: f32) -> Self {
        Self { x, y, z }
    }
}
