#[cfg(features = "single")]
pub type Real = f32;

#[cfg(not(features = "single"))]
pub type Real = f64;
