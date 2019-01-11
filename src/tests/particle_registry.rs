use core::particle::Particle;
use core::particle::force_generators::{DefaultForceGenerator, DefaultGravityForceGenerator};
use core::particle::registry::ParticleRegistry;
use core::vector::Vector3;

#[test]
fn test_registry() {
    // Particle 1
    let position_1 = Vector3::new(0.0, 0.0, 0.0);
    let velocity_1 = Vector3::new(0.0, 0.0, 0.0);
    let acceleration_1 = Vector3::new(5.0, 5.0, 0.0);
    let mass_1 = 1.0;
    let force_generator_1 = DefaultForceGenerator::new(vec![acceleration_1.clone()]);
    let mut particle_1 = Particle::new(position_1, velocity_1, acceleration_1, Some(mass_1));
    particle_1.set_force(String::from("default force"), force_generator_1);

    // Particle 2
    let position_2 = Vector3::new(0.0, 0.0, 0.0);
    let velocity_2 = Vector3::new(0.0, 0.0, 0.0);
    let acceleration_2 = Vector3::new(10.0, 10.0, 0.0);
    let mass_2 = 10.0;
    let force_generator_2 = DefaultGravityForceGenerator::new();
    let mut particle_2 = Particle::new(position_2, velocity_2, acceleration_2, Some(mass_2));
    particle_2.set_force(String::from("default force"), force_generator_2);

    // Create Registry
    let mut registry = ParticleRegistry::new();
    registry.add_particle(String::from("first particle"), particle_1);
    registry.add_particle(String::from("second particle"), particle_2);

    // Test Registry
    assert_eq!(registry.exists(String::from("first particle")), true);
    assert_eq!(registry.exists(String::from("second particle")), true);
    assert_eq!(registry.exists(String::from("third particle")), false);
    assert_eq!(registry.count(), 2);

    registry.update(5.0);

    // Test Mass Aggregate Update
    let new_particle_1 = registry.get_particle(String::from("first particle")).unwrap();
    let test_particle_1_position = Vector3::new(62.5, 62.5, 0.0);
    let new_particle_2 = registry.get_particle(String::from("second particle")).unwrap();
    let test_particle_2_position = Vector3::new(125.0, 125.0, 0.0);
    assert_eq!(new_particle_1.get_position().x, test_particle_1_position.x);
    assert_eq!(new_particle_1.get_position().y, test_particle_1_position.y);
    assert_eq!(new_particle_1.get_position().z, test_particle_1_position.z);
    assert_eq!(new_particle_2.get_position().x, test_particle_2_position.x);
    assert_eq!(new_particle_2.get_position().y, test_particle_2_position.y);
    assert_eq!(new_particle_2.get_position().z, test_particle_2_position.z);
}
