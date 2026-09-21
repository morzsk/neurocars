use rapier2d::prelude::*;

use super::{
    controls::Controls,
    racer::{Racer, RacerGeometry},
};

const MAX_FORWARD_ACCELERATION: Real = 10.0;
const MAX_REVERSE_ACCELERATION: Real = 5.0;
const MAX_STEERING_ACCELERATION: Real = 3.0;
const FULL_STEERING_SPEED: Real = 5.0;
const LINEAR_DAMPING: Real = 0.2;
const ANGULAR_DAMPING: Real = 3.0;

pub struct Physics {
    gravity: Vector,
    integration_parameters: IntegrationParameters,
    pipeline: PhysicsPipeline,
    island_manager: IslandManager,
    broad_phase: DefaultBroadPhase,
    narrow_phase: NarrowPhase,
    bodies: RigidBodySet,
    colliders: ColliderSet,
    impulse_joints: ImpulseJointSet,
    multibody_joints: MultibodyJointSet,
    ccd_solver: CCDSolver,
}

impl Physics {
    pub fn new() -> Self {
        Self {
            gravity: Vector::ZERO,
            integration_parameters: IntegrationParameters::default(),
            pipeline: PhysicsPipeline::new(),
            island_manager: IslandManager::new(),
            broad_phase: DefaultBroadPhase::new(),
            narrow_phase: NarrowPhase::new(),
            bodies: RigidBodySet::new(),
            colliders: ColliderSet::new(),
            impulse_joints: ImpulseJointSet::new(),
            multibody_joints: MultibodyJointSet::new(),
            ccd_solver: CCDSolver::new(),
        }
    }

    pub fn step(&mut self) {
        let physics_hooks = ();
        let event_handler = ();

        self.pipeline.step(
            self.gravity,
            &self.integration_parameters,
            &mut self.island_manager,
            &mut self.broad_phase,
            &mut self.narrow_phase,
            &mut self.bodies,
            &mut self.colliders,
            &mut self.impulse_joints,
            &mut self.multibody_joints,
            &mut self.ccd_solver,
            &physics_hooks,
            &event_handler,
        );
    }

    pub fn add_closed_polyline_collider(&mut self, points: &[Vector]) {
        let indices = (0..points.len())
            .map(|index| [index as u32, ((index + 1) % points.len()) as u32])
            .collect();
        let collider = ColliderBuilder::polyline(points.to_vec(), Some(indices)).build();

        self.colliders.insert(collider);
    }

    pub fn apply_actions(&mut self, handle: RigidBodyHandle, wheelbase: Real, controls: Controls) {
        let Some(body) = self.bodies.get_mut(handle) else {
            return;
        };

        let throttle = controls.throttle.clamp(-1.0, 1.0);
        let steering = controls.steering.clamp(-1.0, 1.0);
        let forward = body.rotation().transform_vector(Vector::Y);
        let left = body.rotation().transform_vector(-Vector::X);
        let mass = body.mass();

        let acceleration = match throttle {
            throttle if throttle >= 0.0 => MAX_FORWARD_ACCELERATION * throttle,
            throttle => MAX_REVERSE_ACCELERATION * throttle,
        };
        let throttle_force = forward * mass * acceleration;

        let forward_speed = body.linvel().dot(forward);
        let steering_effectiveness = (forward_speed.abs() / FULL_STEERING_SPEED).clamp(0.0, 1.0);
        let steering_acceleration =
            MAX_STEERING_ACCELERATION * steering * steering_effectiveness * forward_speed.signum();
        let steering_force = left * mass * steering_acceleration;
        let front_axle = *body.position() * Vector::new(0.0, wheelbase / 2.0);

        body.reset_forces(false);
        body.reset_torques(false);
        body.add_force(throttle_force, true);
        body.add_force_at_point(steering_force, front_axle, true);
    }

    pub fn spawn_racer_at(&mut self, geometry: RacerGeometry, translation: Vector) -> Racer {
        let rigid_body = RigidBodyBuilder::dynamic()
            .translation(translation)
            .linear_damping(LINEAR_DAMPING)
            .angular_damping(ANGULAR_DAMPING)
            .build();
        let physics_handle = self.bodies.insert(rigid_body);
        let collider = ColliderBuilder::cuboid(geometry.width / 2.0, geometry.length / 2.0).build();
        self.colliders
            .insert_with_parent(collider, physics_handle, &mut self.bodies);

        Racer::new(geometry, physics_handle)
    }

    pub fn body(&self, handle: RigidBodyHandle) -> Option<&RigidBody> {
        self.bodies.get(handle)
    }
}

impl Default for Physics {
    fn default() -> Self {
        Self::new()
    }
}
