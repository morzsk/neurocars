use macroquad::prelude::{DrawRectangleParams, Vec2, WHITE, draw_rectangle_ex, vec2};
use rapier2d::prelude::{
    ColliderBuilder, ColliderHandle, Pose, Ray, RigidBodyBuilder, RigidBodyHandle, Vector,
};

use crate::{
    neural::{Genome, decode_output_to_action, encode_sensors_to_input, evaluate, random_genome},
    physics::{ColliderKind, Physics},
    utils::AxisInput,
};

const RACER_WIDTH: f32 = 40.0;
const RACER_HEIGHT: f32 = 20.0;
const RACER_DAMPING: f32 = 0.1;
const RACER_FRONT_GRIP: f32 = 500.0;
const RACER_REAR_GRIP: f32 = 1_000.0;
const RACER_ROAD_FRICTION: f32 = 500.0;
const RACER_THROTTLE_FORCE: f32 = 100_000.0;
const RACER_STEER_FORCE: f32 = 15_000.0;
const RACER_SENSOR_ANGLE: f32 = std::f32::consts::FRAC_PI_4;
pub(crate) const RACER_SENSOR_MAX_DISTANCE: f32 = 400.0;
pub const RACER_SENSOR_INPUT_COUNT: usize = 3;
pub const RACER_ACTION_OUTPUT_COUNT: usize = 2;

pub struct Racer {
    body_handle: RigidBodyHandle,
    pub genome: Genome,
}

pub struct RacerSensor {
    pub left: Option<RacerSensorHit>,
    pub middle: Option<RacerSensorHit>,
    pub right: Option<RacerSensorHit>,
}

#[derive(Clone, Copy, Debug)]
pub struct RacerSensorHit {
    pub collider: ColliderHandle,
    pub kind: ColliderKind,
    pub distance: f32,
}

fn collider_kind(user_data: u128) -> ColliderKind {
    match user_data {
        value if value == ColliderKind::TrackWall as u128 => ColliderKind::TrackWall,
        value if value == ColliderKind::Racer as u128 => ColliderKind::Racer,
        _ => ColliderKind::Unknown,
    }
}

pub fn init_sensors() -> RacerSensor {
    RacerSensor {
        left: None,
        middle: None,
        right: None,
    }
}

fn fire_sensor(physics: &Physics, racer: &Racer, angle: f32) -> Option<RacerSensorHit> {
    let body = &physics.bodies[racer.body_handle];
    let translation = body.translation();
    let rotation = body.rotation();
    let local_origin = Vector::new(RACER_WIDTH / 2.0, 0.0);
    let local_direction = Vector::new(angle.cos(), angle.sin());
    let origin = translation + rotation.transform_vector(local_origin);
    let direction = rotation.transform_vector(local_direction);
    let ray = Ray::new(origin, direction);

    physics
        .cast_ray(&ray, RACER_SENSOR_MAX_DISTANCE, Some(racer.body_handle))
        .map(|(collider, distance)| RacerSensorHit {
            collider,
            kind: collider_kind(physics.colliders[collider].user_data),
            distance,
        })
}

pub fn fire_sensors(physics: &Physics, racer: &Racer, sensors: &mut RacerSensor) {
    sensors.left = fire_sensor(physics, racer, -RACER_SENSOR_ANGLE);
    sensors.middle = fire_sensor(physics, racer, 0.0);
    sensors.right = fire_sensor(physics, racer, RACER_SENSOR_ANGLE);
}

#[derive(Clone, Copy, Default)]
pub struct RacerAction {
    pub throttle: AxisInput,
    pub steer: AxisInput,
}

pub fn evaluate_racer_action(racer: &Racer, sensors: &RacerSensor) -> RacerAction {
    let input = encode_sensors_to_input(sensors);
    let output = evaluate(&racer.genome, input);
    decode_output_to_action(output)
}

pub fn init_racer(physics: &mut Physics, spawn_position: Vec2) -> Racer {
    let rigid_body = RigidBodyBuilder::dynamic()
        .translation(Vector::new(spawn_position.x, spawn_position.y))
        .linear_damping(RACER_DAMPING)
        .angular_damping(RACER_DAMPING)
        .build();
    let body_handle = physics.bodies.insert(rigid_body);
    let collider = ColliderBuilder::cuboid(RACER_WIDTH / 2.0, RACER_HEIGHT / 2.0)
        .user_data(ColliderKind::Racer as u128)
        .build();

    physics
        .colliders
        .insert_with_parent(collider, body_handle, &mut physics.bodies);

    Racer {
        body_handle,
        genome: random_genome(),
    }
}

pub fn restart_racer(physics: &mut Physics, racer: &mut Racer, spawn_position: Vec2) {
    let body = &mut physics.bodies[racer.body_handle];

    body.set_position(Pose::translation(spawn_position.x, spawn_position.y), true);
    body.set_linvel(Vector::ZERO, true);
    body.set_angvel(0.0, true);
    body.reset_forces(true);
    body.reset_torques(true);
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
