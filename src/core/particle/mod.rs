use core::vector::Vector3;
use types::Real;
use self::force_generators::ForceGenerator;
#[cfg(features = "single")]
use std::f32::INFINITY;
#[cfg(not(features = "single"))]
use std::f64::INFINITY;
use std::vec::Vec;
use std::collections::HashMap;

pub mod collision;
pub mod force_generators;
pub mod registry;

#[derive(Clone)]
pub struct Particle {
    position: Vector3,
    velocity: Vector3,
    acceleration: Vector3,
    damping: Real,
    inverse_mass: Real,
    forces: HashMap<String, Box<ForceGenerator + 'static>>
}

impl Particle {
    pub fn new(position: Vector3, velocity: Vector3, acceleration: Vector3, mass: Option<Real>) -> Self {
        let inverse_mass: Real = match mass {
            Some(m) => 1.0 / m,
            None => INFINITY
        };

        Self {
            position,
            velocity,
            acceleration,
            damping: 1.0,
            inverse_mass,
            forces: HashMap::new()
        }
    }

    /**
     * Getters
     */

    pub fn get_position(&self) -> Vector3 {
        self.position.clone()
    }

    pub fn get_velocity(&self) -> Vector3 {
        self.velocity.clone()
    }

    pub fn get_acceleration(&self) -> Vector3 {
        self.acceleration.clone()
    }

    pub fn get_mass(&self) -> Real {
        if self.inverse_mass <= 0.0 {
            return INFINITY;
        }
        1.0 / self.inverse_mass
    }

    pub fn get_force(&self, id: String) -> Option<Vector3> {
        match self.forces.get(&id) {
            None => None,
            Some(boxed_ref) => Some((**boxed_ref).sum_forces())
        }
    }

    pub fn get_forces(&self) -> Option<Vec<Vector3>> {
        let mut force_vec: Vec<Vector3> = Vec::new();

        for key in self.forces.keys() {
            match self.get_force(key.to_string()) {
                None => continue,
                Some(force) => force_vec.push(force)
            }
        }

        if force_vec.is_empty() {
            None
        } else {
            Some(force_vec)
        }
    }

    pub fn get_momentum(&self) -> Vector3 {
        &self.velocity * (1.0 / &self.inverse_mass)
    }

    /**
     * Setters
     */

    pub fn set_position(&mut self, position: Vector3) {
        self.position = position;
    }

    pub fn set_velocity(&mut self, velocity: Vector3) {
        self.velocity = velocity;
    }

    pub fn set_acceleration(&mut self, acceleration: Vector3) {
        self.acceleration = acceleration;
    }

    pub fn set_mass(&mut self, mass: Option<Real>) {
        self.inverse_mass = match mass {
            None => INFINITY,
            Some(some_mass) => some_mass
        }
    }

    pub fn set_force<T: ForceGenerator + 'static>(&mut self, id: String, force: T) {
        self.forces.insert(id, Box::new(force));
    }

    /**
     * Updaters
     */

    pub fn update(&mut self, time: Real) {
        if self.inverse_mass <= 0.0 {
            return;
        }

        let sum_of_forces = self.sum_forces();

        self.position = &self.position + &(&self.velocity * time) + &self.acceleration * time * time * 0.5;
        self.velocity = &self.velocity * self.damping.powf(time) + &self.acceleration * time;
        self.acceleration = sum_of_forces / (1.0 / self.inverse_mass);

        let mut forces_to_remove = Vec::new();
        for (id, force) in self.forces.iter_mut() {
            match force.time_elapsed(time) {
                true => continue,
                false => forces_to_remove.push(id.clone())
            };
        }

        for id in forces_to_remove.iter() {
            self.forces.remove(id);
        }
    }

    pub fn update_position(&mut self, new_position: Vector3) {
        self.position = new_position
    }

    pub fn update_velocity(&mut self, velocity_change: Vector3) {
        self.velocity = &self.velocity + &velocity_change
    }

    /**
     * Utility Functions
     */

    pub fn sum_forces(&self) -> Vector3 {
        let mut sum = Vector3::default();

        for boxed_force in self.forces.values() {
            sum = sum + (**boxed_force).sum_forces();
        }

        sum
    }
}
