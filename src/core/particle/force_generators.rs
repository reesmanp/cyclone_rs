use core::vector::Vector3;
use std::vec::Vec;
use std::clone::Clone;
use types::Real;
use core::particle::Particle;

/// Main trait to attach to forces
pub trait ForceGenerator {
    fn update(&mut self, forces: Vec<Vector3>) -> Vector3;
    fn get_forces(&self) -> Vec<Vector3>;
    fn sum_forces(&self) -> Vector3;
    fn time_elapsed(&mut self, time: Real) -> bool;

    fn clone_box(&self) -> Box<ForceGenerator + 'static>;
}

/// Implements the `Clone` trait for a boxed `ForceGenerator`
impl Clone for Box<ForceGenerator + 'static> {
    fn clone(&self) -> Self {
        self.clone_box()
    }
}

/// A basic force generator used for a simple case
pub struct DefaultForceGenerator {
    force: Vector3
}

/// Implementation of functions specific to instantiation of `DefaultForceGenerator`
impl DefaultForceGenerator {
    pub fn new(forces: Vec<Vector3>) -> Self {
        match forces.is_empty() {
            true => Self {
                force: Vector3::default()
            },
            false => Self {
                force: Self::sum_vec_forces(forces)
            }
        }
    }

    fn sum_vec_forces(force_vec: Vec<Vector3>) -> Vector3 {
        let mut forces = Vector3::default();

        for force in force_vec.iter() {
            forces = &forces + force;
        }

        forces
    }
}

/// Implements the `ForceGenerator` trait for `DefaultForceGenerator`
impl ForceGenerator for DefaultForceGenerator {
    fn update(&mut self, forces: Vec<Vector3>) -> Vector3 {
        match forces.is_empty() {
            true => self.force.clone(),
            false => {
                self.force = &self.force + &Self::sum_vec_forces(forces);
                self.force.clone()
            }
        }
    }

    fn get_forces(&self) -> Vec<Vector3> {
        vec![self.force.clone()]
    }

    fn sum_forces(&self) -> Vector3 {
        self.force.clone()
    }

    fn time_elapsed(&mut self, _time: Real) -> bool {
        true
    }

    fn clone_box(&self) -> Box<ForceGenerator + 'static> {
        Box::new(Self { force: self.force.clone() })
    }
}

/// Basic force using Earths gravity
pub struct DefaultGravityForceGenerator;

/// Instantiation of `DefaultGravityForceGenerator`
impl DefaultGravityForceGenerator {
    pub fn new() -> DefaultForceGenerator {
        DefaultForceGenerator::new(vec![Vector3::new(0.0, -9.81, 0.0)])
    }
}

// TODO: Add `impl ForceGenerator for DefaultGravityForceGenerator`

/// Basic implementation of a force generator that has a time to live
/// Same as `DefaultForceGenerator` except for the ttl
pub struct DefaultTemporaryForceGenerator {
    force: Vector3,
    ttl: Real
}

/// Instantiation for `DefaultTemporaryForceGenerator`
impl DefaultTemporaryForceGenerator {
    pub fn new(forces: Vec<Vector3>, ttl: Real) -> Self {
        match forces.is_empty() {
            true => Self {
                force: Vector3::default(),
                ttl
            },
            false => Self {
                force: Self::sum_vec_forces(forces),
                ttl
            }
        }
    }

    fn sum_vec_forces(force_vec: Vec<Vector3>) -> Vector3 {
        let mut forces = Vector3::default();

        for force in force_vec.iter() {
            forces = &forces + force;
        }

        forces
    }
}

/// Implements the `ForceGenerator` trait for `DefaultTemporaryForceGenerator`
impl ForceGenerator for DefaultTemporaryForceGenerator {
    fn update(&mut self, forces: Vec<Vector3>) -> Vector3 {
        match forces.is_empty() {
            true => self.force.clone(),
            false => {
                self.force = &self.force + &Self::sum_vec_forces(forces);
                self.force.clone()
            }
        }
    }

    fn get_forces(&self) -> Vec<Vector3> {
        vec![self.force.clone()]
    }

    fn sum_forces(&self) -> Vector3 {
        self.force.clone()
    }

    fn time_elapsed(&mut self, time: Real) -> bool {
        self.ttl = self.ttl - time;

        if self.ttl <= 0.0 {
            false
        } else {
            true
        }
    }

    fn clone_box(&self) -> Box<ForceGenerator + 'static> {
        Box::new(Self { force: self.force.clone(), ttl: self.ttl })
    }
}

/// Implements a basic spring particle force generator
/// TODO: Test `other_particle` reference is correct
pub struct DefaultSpringForceGenerator {
    stiffness: Real,
    resting: Real,
    particle: &'static Particle,
    other_particle: &'static Particle

}

/// Instantiation for `DefaultSpringForceGenerator`
impl DefaultSpringForceGenerator {
    pub fn new(stiffness: Real, resting: Real, particle: &'static Particle, other_particle: &'static Particle) -> Self {
        Self {
            stiffness,
            resting,
            particle,
            other_particle
        }
    }
}

/// Implements the `ForceGenerator` trait for `DefaultSpringForceGenerator`
impl ForceGenerator for DefaultSpringForceGenerator {
    fn update(&mut self, _forces: Vec<Vector3>) -> Vector3 {
        Vector3::default()
    }

    fn get_forces(&self) -> Vec<Vector3> {
        vec![self.sum_forces()]
    }

    fn sum_forces(&self) -> Vector3 {
        let d = self.particle.get_position() - self.other_particle.get_position();
        self.stiffness * (d.magnitude() - self.resting) * d.normalize()
    }

    fn time_elapsed(&mut self, _time: Real) -> bool {
        true
    }

    fn clone_box(&self) -> Box<ForceGenerator + 'static> {
        Box::new(Self {
            stiffness: self.stiffness,
            resting: self.resting,
            particle: self.particle,
            other_particle: self.other_particle
        })
    }
}
