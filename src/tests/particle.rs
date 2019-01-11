use core::particle::Particle;
use core::particle::force_generators::DefaultForceGenerator;
use core::particle::force_generators::DefaultTemporaryForceGenerator;
use core::vector::Vector3;

#[test]
fn test_particle_update() {
    let position = Vector3::new(1.0, 2.0, 3.0);
    let velocity = Vector3::new(1.0, -1.0, 2.0);
    let acceleration = Vector3::new(0.0, 1.0, -1.0);
    let force_generator = DefaultForceGenerator::new(vec![acceleration.clone()]);
    let mut my_particle = Particle::new(position.clone(), velocity.clone(), acceleration.clone(), Some(1.0));
    my_particle.set_force(String::from("test force"), force_generator);
    my_particle.update(5.0);

    // Test Position
    let test_one_position = &position + &(&velocity * 5.0) + 0.5 * &acceleration * 25.0;
    assert_eq!(my_particle.get_position().x, test_one_position.x);
    assert_eq!(my_particle.get_position().y, test_one_position.y);
    assert_eq!(my_particle.get_position().z, test_one_position.z);

    // Test Velocity
    let test_one_velocity = &velocity + &(&acceleration * 5.0);
    assert_eq!(my_particle.get_velocity().x, test_one_velocity.x);
    assert_eq!(my_particle.get_velocity().y, test_one_velocity.y);
    assert_eq!(my_particle.get_velocity().z, test_one_velocity.z);

    my_particle.update(4.0);

    // Test Position
    let test_two_position = &position + &(&velocity * 9.0) + 0.5 * &acceleration * 81.0;
    assert_eq!(my_particle.get_position().x, test_two_position.x);
    assert_eq!(my_particle.get_position().y, test_two_position.y);
    assert_eq!(my_particle.get_position().z, test_two_position.z);

    // Test Velocity
    let test_two_velocity = &velocity + &(&acceleration * 9.0);
    assert_eq!(my_particle.get_velocity().x, test_two_velocity.x);
    assert_eq!(my_particle.get_velocity().y, test_two_velocity.y);
    assert_eq!(my_particle.get_velocity().z, test_two_velocity.z);
}

#[test]
fn test_particle_ttl() {
    let position = Vector3::new(1.0, 2.0, 3.0);
    let velocity = Vector3::new(1.0, -1.0, 2.0);
    let acceleration = Vector3::new(0.0, 1.0, -1.0);
    let force_generator = DefaultTemporaryForceGenerator::new(vec![acceleration.clone()], 2.5);
    let mut my_particle = Particle::new(position.clone(), velocity.clone(), acceleration.clone(), Some(1.0));
    my_particle.set_force(String::from("test force"), force_generator);
    my_particle.update(5.0);

    let default_vector = Vector3::default();
    assert_eq!(my_particle.sum_forces().x, default_vector.x);
    assert_eq!(my_particle.sum_forces().y, default_vector.y);
    assert_eq!(my_particle.sum_forces().z, default_vector.z);
}

#[test]
fn test_momentum() {
    let position = Vector3::new(1.0, 2.0, 3.0);
    let velocity = Vector3::new(1.0, -1.0, 2.0);
    let acceleration = Vector3::new(0.0, 1.0, -1.0);
    let mass = 25.0;
    let new_particle = Particle::new(position.clone(), velocity.clone(), acceleration.clone(), Some(mass));

    let particle_momentum = new_particle.get_momentum();
    let test_momentum = &velocity * mass;
    assert_eq!(particle_momentum.x, test_momentum.x);
    assert_eq!(particle_momentum.y, test_momentum.y);
    assert_eq!(particle_momentum.z, test_momentum.z);
}
