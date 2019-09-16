//! This module contains the 3D Vector implementation used
//! by pretty much everything in the program. Obviously this
//! is kind of boilerplate-ey and I could have just used an external
//! crate, but I wanted to minimize the number of dependencies
//! since this is a learning project.

use crate::util::fast_inv_sqrt;

use rand::prelude::*;

use std::f32;
use std::ops;

/// A 3 dimentional vector.
#[derive(Debug, Copy, Clone)]
pub struct Vector {
    pub x: f32,
    pub y: f32,
    pub z: f32,
}

/// Constructor-like
impl Vector {
    /// Creates a new vector with the provided x, y, and z values
    pub fn new(x: f32, y: f32, z: f32) -> Self {
        Self { x, y, z }
    }

    /// Creates a vector with all values as zero
    pub fn zeros() -> Self {
        Self {
            x: 0.0,
            y: 0.0,
            z: 0.0,
        }
    }

    /// Creates a vector with all values as one
    pub fn ones() -> Self {
        Self {
            x: 1.0,
            y: 1.0,
            z: 1.0,
        }
    }

    /// Creates a buffer with all random values (between 0 and 1)
    pub fn rand() -> Self {
        Self {
            x: random::<f32>(),
            y: random::<f32>(),
            z: random::<f32>(),
        }
    }
}

/// Vector math implementations
impl Vector {
    /// Return a vector with the same direction but with length 1
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

    /// Returns the reciprocal of the square root of the vector's magnitude
    pub fn inv_mag(self) -> f32 {
        fast_inv_sqrt((self.x * self.x) + (self.y * self.y) + (self.z * self.z))
    }

    /// Return the length of the vector
    pub fn length(self) -> f32 {
        f32::sqrt(self.x * self.x + self.y * self.y + self.z * self.z)
    }

    /// Compute the dot product of two vectors
    pub fn dot(lhs: Vector, rhs: Vector) -> f32 {
        lhs.x * rhs.x + lhs.y * rhs.y + lhs.z * rhs.z
    }

    /// Compute the cross product of two vectors
    pub fn cross(lhs: Vector, rhs: Vector) -> Self {
        Self {
            x: lhs.y * rhs.z - lhs.z * rhs.y,
            y: lhs.z * rhs.x - lhs.x * rhs.z,
            z: lhs.x * rhs.y - lhs.y * rhs.x,
        }
    }
}

/// Add two vectors together
impl ops::Add<Vector> for Vector {
    type Output = Vector;

    fn add(self, rhs: Vector) -> Vector {
        Vector {
            x: self.x + rhs.x,
            y: self.y + rhs.y,
            z: self.z + rhs.z,
        }
    }
}

/// Add-assign two vectors together
impl ops::AddAssign<Vector> for Vector {
    fn add_assign(&mut self, rhs: Vector) {
        self.x += rhs.x;
        self.y += rhs.y;
        self.z += rhs.z;
    }
}

/// Add a float to all fields in vector
impl ops::Add<f32> for Vector {
    type Output = Vector;

    fn add(self, rhs: f32) -> Vector {
        Vector {
            x: self.x + rhs,
            y: self.y + rhs,
            z: self.z + rhs,
        }
    }
}

/// Subtract two vectors
impl ops::Sub<Vector> for Vector {
    type Output = Vector;

    fn sub(self, rhs: Vector) -> Vector {
        Vector {
            x: self.x - rhs.x,
            y: self.y - rhs.y,
            z: self.z - rhs.z,
        }
    }
}

/// Multiply two vectors together element-wise (NOT dot product OR cross product)
impl ops::Mul<Vector> for Vector {
    type Output = Vector;

    fn mul(self, rhs: Vector) -> Vector {
        Vector {
            x: self.x * rhs.x,
            y: self.y * rhs.y,
            z: self.z * rhs.z,
        }
    }
}

/// Multiply each element of a vector by a float
impl ops::Mul<f32> for Vector {
    type Output = Vector;

    fn mul(self, rhs: f32) -> Vector {
        Vector {
            x: self.x * rhs,
            y: self.y * rhs,
            z: self.z * rhs,
        }
    }
}

/// Commutative property for Vector * f32
impl ops::Mul<Vector> for f32 {
    type Output = Vector;

    fn mul(self, rhs: Vector) -> Vector {
        Vector {
            x: rhs.x * self,
            y: rhs.y * self,
            z: rhs.z * self,
        }
    }
}

/// Multiply-asign each element of a vector by a float
impl ops::MulAssign<f32> for Vector {
    fn mul_assign(&mut self, rhs: f32) {
        self.x *= rhs;
        self.y *= rhs;
        self.z *= rhs;
    }
}

/// Divide all fields by a float
impl ops::Div<f32> for Vector {
    type Output = Vector;

    fn div(self, rhs: f32) -> Vector {
        Vector {
            x: self.x / rhs,
            y: self.y / rhs,
            z: self.z / rhs,
        }
    }
}

/// Divide-assign all fields by a float
impl ops::DivAssign<f32> for Vector {
    fn div_assign(&mut self, rhs: f32) {
        self.x /= rhs;
        self.y /= rhs;
        self.z /= rhs;
    }
}

/// Allow -vector
impl ops::Neg for Vector {
    type Output = Vector;
    fn neg(self) -> Vector {
        Vector {
            x: -self.x,
            y: -self.y,
            z: -self.z,
        }
    }
}

// Allow indexing into the vector's values with the v[idx] syntax
impl ops::Index<usize> for Vector {
    type Output = f32;
    fn index(&self, idx: usize) -> &Self::Output {
        match idx {
            0 => &self.x,
            1 => &self.y,
            2 => &self.z,
            _ => unreachable!("Index out of bounds!"),
        }
    }
}

// Allow indexing into the vector's values with the v[idx] syntax
impl ops::IndexMut<usize> for Vector {
    fn index_mut(&mut self, idx: usize) -> &mut f32 {
        match idx {
            0 => &mut self.x,
            1 => &mut self.y,
            2 => &mut self.z,
            _ => unreachable!("Index out of bounds!"),
        }
    }
}
