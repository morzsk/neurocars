use rapier2d::prelude::*;

pub struct Physics {
    gravity: Vector,
    integration_parameters: IntegrationParameters,
    pipeline: PhysicsPipeline,
    pub(crate) island_manager: IslandManager,
    broad_phase: DefaultBroadPhase,
    narrow_phase: NarrowPhase,
    pub(crate) bodies: RigidBodySet,
    pub(crate) colliders: ColliderSet,
    impulse_joints: ImpulseJointSet,
    multibody_joints: MultibodyJointSet,
    ccd_solver: CCDSolver,
}

pub fn init_physics() -> Physics {
    Physics {
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

pub fn step_physics(physics: &mut Physics) {
    let physics_hooks = ();
    let event_handler = ();

    physics.pipeline.step(
        physics.gravity,
        &physics.integration_parameters,
        &mut physics.island_manager,
        &mut physics.broad_phase,
        &mut physics.narrow_phase,
        &mut physics.bodies,
        &mut physics.colliders,
        &mut physics.impulse_joints,
        &mut physics.multibody_joints,
        &mut physics.ccd_solver,
        &physics_hooks,
        &event_handler,
    );
}
