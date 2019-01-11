use core::vector::Vector3;
use std::vec::Vec;
use std::clone::Clone;
use types::Real;
use core::particle::Particle;

pub trait ForceGenerator {
    fn update(&mut self, forces: Vec<Vector3>) -> Vector3;
    fn get_forces(&self) -> Vec<Vector3>;
    fn sum_forces(&self) -> Vector3;
    fn time_elapsed(&mut self, time: Real) -> bool;

    fn clone_box(&self) -> Box<ForceGenerator + 'static>;
}

impl Clone for Box<dyn ForceGenerator + 'static> {
    fn clone(&self) -> Self {
        self.clone_box()
    }
}

pub struct DefaultForceGenerator {
    force: Vector3
}

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

pub struct DefaultGravityForceGenerator;

impl DefaultGravityForceGenerator {
    pub fn new() -> DefaultForceGenerator {
        DefaultForceGenerator::new(vec![Vector3::new(0.0, -9.81, 0.0)])
    }
}

pub struct DefaultTemporaryForceGenerator {
    force: Vector3,
    ttl: Real
}

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

pub struct DefaultSpringForceGenerator {
    stiffness: Real,
    resting: Real,
    particle: &'static Particle,
    other_particle: &'static Particle

}

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
