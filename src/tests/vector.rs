use core::vector::Vector3;
use types::Real;
use std::f64::consts::PI;
use assert_approx_eq::assert_approx_eq;

#[test]
fn test_magnitude() {
    let test_vector = Vector3::new(2.0, -2.0, -2.0);
    assert_eq!(test_vector.magnitude(), 2.0 * 3.0_f64.sqrt());
}

#[test]
fn test_direction() {
    let test_vector = Vector3::new(2.0, -2.0, -2.0);
    let normalized_test_vector = test_vector.normalize();
    let denominator: Real = 3.0;
    let sqrt_denomiator = denominator.sqrt();
    let answer_vector = Vector3::new(1.0 / sqrt_denomiator, -1.0 / sqrt_denomiator, -1.0 / sqrt_denomiator);
    assert_eq!(normalized_test_vector.x, answer_vector.x);
    assert_eq!(normalized_test_vector.y, answer_vector.y);
    assert_eq!(normalized_test_vector.z, answer_vector.z);
}

#[test]
fn test_dot() {
    let test_vector_left = Vector3::new(3.0, 1.0, 2.0);
    let test_vector_right = Vector3::new(0.0, 2.0, -1.0);
    assert_eq!(test_vector_left * test_vector_right, 0.0);
}

#[test]
fn test_duplicate_dot() {
    let test_vector_left = Vector3::new(1.0, 1.0, 1.0);
    let test_vector_right = test_vector_left.normalize_clone();
    assert_approx_eq!(test_vector_left * test_vector_right, 3.0_f64.sqrt());
}

#[test]
fn test_find_angle() {
    let test_vec_one = Vector3::new(0.0, 1.0, 1.0);
    let test_vec_two = Vector3::new(0.0, -1.0, 0.0);
    assert_eq!(test_vec_one.find_angle(&test_vec_two), (3.0 * PI) / 4.0);
}

#[test]
fn test_2_6() {
    let a = (1.0 / 2.0_f64.sqrt()) * Vector3::new(0.0, 1.0, 1.0);
    let b = Vector3::new(1.0, 2.0, 3.0);
    let c = a.clone() * b.clone();
    assert_eq!(c, 5.0 / 2.0_f64.sqrt());
    let d = b.clone() - c * a.clone();
    assert_eq!(d.x, 1.0);
    assert_approx_eq!(d.y, -1.0 / 2.0);
    assert_approx_eq!(d.z, 1.0 / 2.0);
}
