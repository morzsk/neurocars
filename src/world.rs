use rapier2d::prelude::Vector;

use crate::{
    controls::Controls,
    physics::Physics,
    racer::{Racer, RacerGeometry},
};

pub struct World {
    physics: Physics,
    racers: Vec<Racer>,
}

impl World {
    pub fn new() -> Self {
        Self {
            physics: Physics::new(),
            racers: Vec::new(),
        }
    }

    pub fn step(&mut self) {
        self.physics.step();
    }

    pub fn apply_actions(&mut self, racer_id: usize, controls: Controls) {
        let Some(racer) = self.racers.get(racer_id) else {
            return;
        };

        self.physics
            .apply_actions(racer.physics_handle, racer.geometry.wheelbase, controls);
    }

    pub fn spawn_racer_at(&mut self, geometry: RacerGeometry, translation: Vector) -> usize {
        let racer = self.physics.spawn_racer_at(geometry, translation);
        self.racers.push(racer);

        self.racers.len() - 1
    }

    pub fn racers(&self) -> &[Racer] {
        &self.racers
    }

    pub fn physics(&self) -> &Physics {
        &self.physics
    }
}

impl Default for World {
    fn default() -> Self {
        Self::new()
    }
}
