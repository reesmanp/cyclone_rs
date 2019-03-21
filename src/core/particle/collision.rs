use core::particle::Particle;
use core::vector::Vector3;

/// Struct to determine collisions
struct Collision;

/// Implements `Collision`
impl Collision {
    pub fn closing_velocity(a: &Particle, b: &Particle) -> Vector3 {
        let a_velocity = a.get_velocity();
        let b_velocity = b.get_velocity();
        -1.0 * (a_velocity.magnitude() - b_velocity.magnitude()) * (a_velocity - b_velocity).normalize()
    }
}
