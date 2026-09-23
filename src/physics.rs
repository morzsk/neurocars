use rapier2d::prelude::*;

#[repr(u8)]
#[derive(Clone, Copy, Debug, PartialEq, Eq)]
pub enum ColliderKind {
    Unknown = 0,
    TrackWall = 1,
    Racer = 2,
}

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

impl Physics {
    pub fn cast_ray(
        &self,
        ray: &Ray,
        max_distance: f32,
        exclude_rigid_body: Option<RigidBodyHandle>,
    ) -> Option<(ColliderHandle, f32)> {
        let filter = match exclude_rigid_body {
            Some(handle) => QueryFilter::default().exclude_rigid_body(handle),
            None => QueryFilter::default(),
        };
        let query_pipeline = self.broad_phase.as_query_pipeline(
            self.narrow_phase.query_dispatcher(),
            &self.bodies,
            &self.colliders,
            filter,
        );

        query_pipeline.cast_ray(ray, max_distance, true)
    }
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
