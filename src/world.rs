use rapier2d::prelude::Vector;

use crate::{
    controls::Controls,
    physics::Physics,
    racer::{Racer, RacerGeometry},
    tracks::Track,
};

pub struct World {
    physics: Physics,
    racers: Vec<Racer>,
    track: Track,
}

impl World {
    pub fn new(track: Track) -> Self {
        let mut physics = Physics::new();
        physics.add_closed_polyline_collider(track.collision_boundary().inner_points());
        physics.add_closed_polyline_collider(track.collision_boundary().outer_points());

        Self {
            physics,
            racers: Vec::new(),
            track,
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

    pub fn track(&self) -> &Track {
        &self.track
    }

    pub fn physics(&self) -> &Physics {
        &self.physics
    }
}
