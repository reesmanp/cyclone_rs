use super::Particle;
use std::collections::HashMap;
use types::Real;

pub struct ParticleRegistry {
    pub particles: HashMap<String, Particle>
}

impl ParticleRegistry {
    pub fn new() -> Self {
        Self {
            particles: HashMap::new()
        }
    }

    pub fn count(&self) -> usize {
        self.particles.len()
    }

    pub fn exists(&self, id: String) -> bool {
        match self.particles.get(&id) {
            None => false,
            Some(_) => true
        }
    }

    pub fn get_particle(&self, id: String) -> Option<Particle> {
        match self.particles.get(&id) {
            None => None,
            Some(particle) => Some(particle.clone())
        }
    }

    pub fn add_particle(&mut self, id: String, particle: Particle) {
        self.particles.insert(id, particle);
    }

    pub fn remove_particle(&mut self, id: String) {
        self.particles.remove(&id);
    }

    pub fn update(&mut self, time: Real) {
        for particle in self.particles.values_mut() {
            particle.update(time);
        }
    }
}
