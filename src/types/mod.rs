#[cfg(features = "single")]
/// Real type using 32 bits
/// Compiled via `single` argument
pub type Real = f32;

#[cfg(not(features = "single"))]
/// Real type using 64 bits
/// Compiled by default
pub type Real = f64;
