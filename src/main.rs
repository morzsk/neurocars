use macroquad::prelude::*;
use rapier2d::prelude::*;

const PIXELS_PER_METER: f32 = 40.0;
const GROUND_HALF_WIDTH: f32 = 100.0;
const GROUND_HALF_HEIGHT: f32 = 0.1;
const BALL_RADIUS: f32 = 0.5;

#[macroquad::main("Neurocar")]
async fn main() {
    let mut rigid_body_set = RigidBodySet::new();
    let mut collider_set = ColliderSet::new();

    let ground = ColliderBuilder::cuboid(GROUND_HALF_WIDTH, GROUND_HALF_HEIGHT)
        .restitution(0.95)
        .build();
    collider_set.insert(ground);

    let ball_body = RigidBodyBuilder::dynamic()
        .translation(Vector::new(0.0, 10.0))
        .build();
    let ball_collider = ColliderBuilder::ball(BALL_RADIUS).restitution(0.95).build();
    let ball_body_handle = rigid_body_set.insert(ball_body);
    collider_set.insert_with_parent(ball_collider, ball_body_handle, &mut rigid_body_set);

    let gravity = Vector::new(0.0, -9.81);
    let integration_parameters = IntegrationParameters::default();
    let mut physics_pipeline = PhysicsPipeline::new();
    let mut island_manager = IslandManager::new();
    let mut broad_phase = DefaultBroadPhase::new();
    let mut narrow_phase = NarrowPhase::new();
    let mut impulse_joint_set = ImpulseJointSet::new();
    let mut multibody_joint_set = MultibodyJointSet::new();
    let mut ccd_solver = CCDSolver::new();
    let physics_hooks = ();
    let event_handler = ();

    loop {
        physics_pipeline.step(
            gravity,
            &integration_parameters,
            &mut island_manager,
            &mut broad_phase,
            &mut narrow_phase,
            &mut rigid_body_set,
            &mut collider_set,
            &mut impulse_joint_set,
            &mut multibody_joint_set,
            &mut ccd_solver,
            &physics_hooks,
            &event_handler,
        );

        clear_background(BLACK);

        let visible_width = screen_width() / PIXELS_PER_METER;
        let visible_height = screen_height() / PIXELS_PER_METER;
        let camera = Camera2D::from_display_rect(Rect::new(
            -visible_width / 2.0,
            0.0,
            visible_width,
            visible_height,
        ));
        set_camera(&camera);

        draw_rectangle(
            -GROUND_HALF_WIDTH,
            -GROUND_HALF_HEIGHT,
            GROUND_HALF_WIDTH * 2.0,
            GROUND_HALF_HEIGHT * 2.0,
            WHITE,
        );

        let ball = &rigid_body_set[ball_body_handle];
        let ball_position = ball.translation();
        draw_circle(ball_position.x, ball_position.y, BALL_RADIUS, WHITE);

        next_frame().await;
    }
}
