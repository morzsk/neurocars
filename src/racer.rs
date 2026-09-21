use macroquad::prelude::{DrawRectangleParams, Vec2, WHITE, draw_rectangle_ex, vec2};
use rapier2d::prelude::{ColliderBuilder, RigidBodyBuilder, RigidBodyHandle, Vector};

use crate::physics::Physics;

const RACER_WIDTH: f32 = 40.0;
const RACER_HEIGHT: f32 = 20.0;
const RACER_DAMPING: f32 = 0.1;
const RACER_FRONT_GRIP: f32 = 500.0;
const RACER_REAR_GRIP: f32 = 1_000.0;
const RACER_ROAD_FRICTION: f32 = 500.0;
const RACER_THROTTLE_FORCE: f32 = 100_000.0;
const RACER_STEER_FORCE: f32 = 15_000.0;

pub struct Racer {
    body_handle: RigidBodyHandle,
}

#[derive(Clone, Copy, Default)]
pub struct AxisInput(f32);

impl AxisInput {
    pub const NEGATIVE: Self = Self(-1.0);
    pub const NEUTRAL: Self = Self(0.0);
    pub const POSITIVE: Self = Self(1.0);

    pub fn new(value: f32) -> Option<Self> {
        (-1.0..=1.0).contains(&value).then_some(Self(value))
    }

    fn value(self) -> f32 {
        self.0
    }
}

#[derive(Clone, Copy, Default)]
pub struct RacerAction {
    pub throttle: AxisInput,
    pub steer: AxisInput,
}

pub fn init_racer(physics: &mut Physics) -> Racer {
    let rigid_body = RigidBodyBuilder::dynamic()
        .linear_damping(RACER_DAMPING)
        .angular_damping(RACER_DAMPING)
        .build();
    let body_handle = physics.bodies.insert(rigid_body);
    let collider = ColliderBuilder::cuboid(RACER_WIDTH / 2.0, RACER_HEIGHT / 2.0).build();

    physics
        .colliders
        .insert_with_parent(collider, body_handle, &mut physics.bodies);

    Racer { body_handle }
}

pub fn step_racer(physics: &mut Physics, racer: &Racer, action: RacerAction) {
    let body = &mut physics.bodies[racer.body_handle];

    body.reset_forces(false);
    body.reset_torques(false);

    let forward = body.rotation().transform_vector(Vector::X);
    let sideways = body.rotation().transform_vector(Vector::Y);
    let front = body.translation() + forward * RACER_WIDTH / 2.0;
    let rear = body.translation() - forward * RACER_WIDTH / 2.0;
    let forward_speed = body.linvel().dot(forward);
    let front_lateral_speed = body.velocity_at_point(front).dot(sideways);
    let rear_lateral_speed = body.velocity_at_point(rear).dot(sideways);

    if forward_speed != 0.0 {
        body.add_force(-forward * forward_speed * RACER_ROAD_FRICTION, true);
    }

    if front_lateral_speed != 0.0 {
        body.add_force_at_point(
            -sideways * front_lateral_speed * RACER_FRONT_GRIP,
            front,
            true,
        );
    }

    if rear_lateral_speed != 0.0 {
        body.add_force_at_point(-sideways * rear_lateral_speed * RACER_REAR_GRIP, rear, true);
    }

    if action.throttle.value() != 0.0 {
        body.add_force(
            forward * action.throttle.value() * RACER_THROTTLE_FORCE,
            true,
        );
    }

    if action.steer.value() != 0.0 {
        body.add_force_at_point(
            sideways * action.steer.value() * RACER_STEER_FORCE,
            front,
            true,
        );
    }
}

pub fn draw_racer(physics: &Physics, racer: &Racer) {
    let position = physics.bodies[racer.body_handle].position();
    let translation = position.translation;

    draw_rectangle_ex(
        translation.x,
        translation.y,
        RACER_WIDTH,
        RACER_HEIGHT,
        DrawRectangleParams {
            offset: vec2(0.5, 0.5),
            rotation: position.rotation.angle(),
            color: WHITE,
        },
    );
}

pub fn racer_position(physics: &Physics, racer: &Racer) -> Vec2 {
    let translation = physics.bodies[racer.body_handle].translation();
    vec2(translation.x, translation.y)
}
