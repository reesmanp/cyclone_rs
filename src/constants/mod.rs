use crate::core::vector::Vector3;

/// Average force from gravity on Earth
pub const EARTH_G: Vector3 = Vector3 {
    x: 0.0,
    y: -9.81,
    z: 0.0,
    pad: 0.0
};
