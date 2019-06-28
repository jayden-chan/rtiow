use crate::util::fast_inv_sqrt;

#[derive(Debug)]
pub struct Vector {
    pub x: f32,
    pub y: f32,
    pub z: f32,
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
}
