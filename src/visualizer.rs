use macroquad::prelude::{
    Camera2D, DARKGRAY, DrawRectangleParams, RED, Vec2, draw_rectangle_ex, screen_height,
    screen_width, set_camera, set_default_camera, vec2,
};

use crate::world::World;

const WHEEL_WIDTH_METERS: f32 = 0.1;
const WHEEL_LENGTH_METERS: f32 = 0.7;

pub fn draw(world: &World, pixels_per_meter: f32) {
    let camera = Camera2D {
        zoom: vec2(
            2.0 * pixels_per_meter / screen_width(),
            -2.0 * pixels_per_meter / screen_height(),
        ),
        ..Default::default()
    };
    set_camera(&camera);

    for racer in world.racers() {
        let Some(body) = world.physics().body(racer.physics_handle) else {
            continue;
        };

        let rotation = body.rotation().angle();

        for wheel_position in racer.geometry.wheel_positions() {
            let wheel_world_position = *body.position() * *wheel_position;

            draw_centered_rectangle(
                vec2(wheel_world_position.x, wheel_world_position.y),
                WHEEL_WIDTH_METERS,
                WHEEL_LENGTH_METERS,
                rotation,
                DARKGRAY,
            );
        }

        draw_centered_rectangle(
            vec2(body.translation().x, body.translation().y),
            racer.geometry.width,
            racer.geometry.length,
            rotation,
            RED,
        );
    }

    set_default_camera();
}

fn draw_centered_rectangle(
    position: Vec2,
    width: f32,
    height: f32,
    rotation: f32,
    color: macroquad::color::Color,
) {
    draw_rectangle_ex(
        position.x,
        position.y,
        width,
        height,
        DrawRectangleParams {
            offset: vec2(0.5, 0.5),
            rotation,
            color,
        },
    );
}
