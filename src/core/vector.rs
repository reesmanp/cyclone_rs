use crate::types::Real;
use std::ops::{Add, Mul, Div, Sub};
use std::clone::Clone;

#[allow(dead_code)]
/// Struct representing 3-Dimensional vectors
pub struct Vector3 {
    pub x: Real,
    pub y: Real,
    pub z: Real,
    pub pad: Real
}

/// Implement `Vector3`
impl Vector3 {
    pub fn new(x: Real, y: Real, z: Real) -> Self {
        Self {
            x,
            y,
            z,
            pad: 0.0
        }
    }

    /**
     * Takes ownership of self
     * Returns new instance of self that is inverted
     */
    pub fn invert(self) -> Self {
        Self {
            x: self.x * -1.0,
            y: self.y * -1.0,
            z: self.z * -1.0,
            pad: 0.0
        }
    }

    /**
     * Mutates self
     * Inverts self without returning
     */
    pub fn invert_inplace(&mut self) {
        self.x = self.x * -1.0;
        self.y = self.y * -1.0;
        self.z = self.z * -1.0;
    }

    /**
     * Does not take ownership of self
     * Returns a new instance that is inverted from self
     */
    pub fn invert_clone(&self) -> Self {
        Self {
            x: self.x * -1.0,
            y: self.y * -1.0,
            z: self.z * -1.0,
            pad: 0.0
        }
    }

    pub fn magnitude(&self) -> Real {
        let mag: Real = self.square_magnitude();
        mag.sqrt()
    }

    pub fn square_magnitude(&self) -> Real {
        self.x * self.x + self.y * self.y + self.z * self.z
    }

    pub fn normalize(self) -> Self {
        let mag = self.magnitude();
        Self {
            x: self.x / mag,
            y: self.y / mag,
            z: self.z / mag,
            pad: 0.0
        }
    }

    pub fn normalize_inplace(&mut self) {
        let mag = self.magnitude();
        self.x = self.x / mag;
        self.y = self.y / mag;
        self.z = self.z - mag;
    }

    pub fn normalize_clone(&self) -> Self {
        let mag = self.magnitude();
        Self {
            x: self.x / mag,
            y: self.y / mag,
            z: self.z / mag,
            pad: 0.0
        }
    }

    pub fn cross(self, rhs: Self) -> Self {
        Self {
            x: self.y * rhs.z - self.z * rhs.y,
            y: self.z * rhs.x - self.z * rhs.z,
            z: self.x * rhs.y - self.y * rhs.x,
            pad: 0.0
        }
    }

    pub fn cross_clone(&self, rhs: &Self) -> Self {
        Self {
            x: self.y * rhs.z - self.z * rhs.y,
            y: self.z * rhs.x - self.z * rhs.z,
            z: self.x * rhs.y - self.y * rhs.x,
            pad: 0.0
        }
    }

    pub fn dot(self, rhs: Self) -> Real {
        self * rhs
    }

    pub fn create_orthonormal_basis(&self, b: &Self) -> Option<(Self, Self)> {
        let a = self.normalize_clone();
        let c = a.cross_clone(&b);

        if c.magnitude() == 0.0 {
            None
        } else {
            let c_norm = c.normalize();
            let b_norm = c_norm.cross_clone(&a);
            Some((b_norm, c_norm))
        }
    }

    pub fn find_angle(&self, b: &Self) -> Real {
        let a = self.clone();
        let a_length = a.magnitude();
        let b_length = b.magnitude();
        ((&a * &b) / (a_length * b_length)).acos()
    }
}

/// Implements the `Default` trait for `Vector3`
impl Default for Vector3 {
    fn default() -> Self {
        Self {
            x: 0.0,
            y: 0.0,
            z: 0.0,
            pad: 0.0
        }
    }
}

/// Implements the `Add` trait for `Vector3` with `Self`
impl Add<Self> for Vector3 {
    type Output = Self;

    fn add(self, rhs: Self) -> Self {
        Self {
            x: self.x + rhs.x,
            y: self.y + rhs.y,
            z: self.z + rhs.z,
            pad: 0.0
        }
    }
}

/// Implements the `Add` trait for `&Vector3` with `Self`
impl<'a> Add<Self> for &'a Vector3 {
    type Output = Vector3;

    fn add(self, rhs: Self) -> Vector3 {
        Vector3 {
            x: self.x + rhs.x,
            y: self.y + rhs.y,
            z: self.z + rhs.z,
            pad: 0.0
        }
    }
}

/// Implements the `Sub` trait for `Vector3` with `Self`
impl Sub<Self> for Vector3 {
    type Output = Self;

    fn sub(self, rhs: Self) -> Self {
        let rhs_inverted = rhs.invert();
        self + rhs_inverted
    }
}

/// Implements the `Mul` trait for `Vector3` with `Real`
impl Mul<Real> for Vector3 {
    type Output = Self;

    fn mul(self, rhs: Real) -> Self {
        Self {
            x: self.x * rhs,
            y: self.y * rhs,
            z: self.z * rhs,
            pad: 0.0
        }
    }
}

/// Implements the `Mul` trait for `&Vector3` with `Real`
impl<'a> Mul<Real> for &'a Vector3 {
    type Output = Vector3;

    fn mul(self, rhs: Real) -> Vector3 {
        Vector3 {
            x: self.x * rhs,
            y: self.y * rhs,
            z: self.z * rhs,
            pad: 0.0
        }
    }
}

/// Implements the `Mul` trait for `Real` with `Vector3`
impl Mul<Vector3> for Real {
    type Output = Vector3;

    fn mul(self, rhs: Vector3) -> Vector3 {
        Vector3 {
            x: self * rhs.x,
            y: self * rhs.y,
            z: self * rhs.z,
            pad: 0.0
        }
    }
}

/// Implements the `Mul` trait for `Real` with `&Vector3`
impl<'a> Mul<&'a Vector3> for Real {
    type Output = Vector3;

    fn mul(self, rhs: &'a Vector3) -> Vector3 {
        Vector3 {
            x: self * rhs.x,
            y: self * rhs.y,
            z: self * rhs.z,
            pad: 0.0
        }
    }
}

/// Implements the `Mul` trait for `Vector3` with `Self`
/// Dot Product
/// `|a||b|cos {theta}`
impl Mul<Self> for Vector3 {
    type Output = Real;

    fn mul(self, rhs: Self) -> Real {
        self.x * rhs.x + self.y * rhs.y + self.z * rhs.z
    }
}

/// Implements the `Mul` trait for `&Vector3` with `&Self`
/// Dot Product
/// `|a||b|cos {theta}`
impl<'a> Mul<&'a Self> for &'a Vector3 {
    type Output = Real;

    fn mul(self, rhs: &'a Self) -> Real {
        self.x * rhs.x + self.y * rhs.y + self.z * rhs.z
    }
}

/// Implements the `Div` trait for `Vector3` with `Real`
impl Div<Real> for Vector3 {
    type Output = Self;

    fn div(self, rhs: Real) -> Self {
        let rhs_norm = 1.0 / rhs;
        self * rhs_norm
    }
}

/// Implements the `Clone` trait for `Vector3`
impl Clone for Vector3 {
    fn clone(&self) -> Self {
        Self {
            x: self.x,
            y: self.y,
            z: self.z,
            pad: 0.0
        }
    }
}
