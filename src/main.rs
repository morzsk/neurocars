use macroquad::prelude::*;

mod bezier;
mod camera;
pub mod physics;
pub mod racer;
mod track;

use camera::{CameraMode, apply_camera, camera_mouse_position, init_camera, step_camera};
use physics::{init_physics, step_physics};
use racer::{AxisInput, RacerAction, draw_racer, init_racer, racer_position, step_racer};
use track::{
    Track, add_to_track, draw_track, draw_track_preview, init_track, remove_from_track,
    update_collider,
};

const TRACK_WIDTH: f32 = 200.0;
const CURVE_T_STEP: f32 = 1.0 / 40.0;
const DEFAULT_PIXELS_PER_METER: f32 = 1.0;

fn axis_input(positive: KeyCode, negative: KeyCode) -> AxisInput {
    match (is_key_down(positive), is_key_down(negative)) {
        (true, false) => AxisInput::POSITIVE,
        (false, true) => AxisInput::NEGATIVE,
        _ => AxisInput::NEUTRAL,
    }
}

#[macroquad::main("Neurocar")]
async fn main() {
    let mut physics = init_physics();
    let racer = init_racer(&mut physics);
    let mut track = None::<Track>;
    let mut edit_mode = true;
    let mut camera = init_camera(DEFAULT_PIXELS_PER_METER);

    loop {
        if is_key_pressed(KeyCode::Tab) {
            edit_mode = !edit_mode;

            if !edit_mode && let Some(track) = track.as_mut() {
                update_collider(track, &mut physics);
            }
        }

        let racer_action = if edit_mode {
            RacerAction::default()
        } else {
            RacerAction {
                throttle: axis_input(KeyCode::W, KeyCode::S),
                steer: axis_input(KeyCode::D, KeyCode::A),
            }
        };

        step_racer(&mut physics, &racer, racer_action);

        step_physics(&mut physics);

        camera.mode = match edit_mode {
            false => CameraMode::Follow(racer_position(&physics, &racer)),
            true => CameraMode::Free,
        };
        step_camera(&mut camera);

        if edit_mode {
            let control_pressed =
                is_key_down(KeyCode::LeftControl) || is_key_down(KeyCode::RightControl);

            if control_pressed && is_key_pressed(KeyCode::Z) {
                match track.as_mut() {
                    Some(track) if track.vertices.len() > 1 => {
                        remove_from_track(track);
                    }
                    Some(track_with_one_vertex) => {
                        update_collider(track_with_one_vertex, &mut physics);
                        track = None;
                    }
                    None => {}
                }
            }

            if is_mouse_button_pressed(MouseButton::Left) {
                let point = camera_mouse_position(&camera);

                match track.as_mut() {
                    Some(track) => add_to_track(track, point),
                    None => track = Some(init_track(TRACK_WIDTH, point, CURVE_T_STEP)),
                }
            }
        }

        clear_background(BLACK);
        apply_camera(&camera);

        if let Some(track) = &track {
            draw_track(track);

            if edit_mode {
                for vertex in &track.vertices {
                    draw_circle(vertex.x, vertex.y, 4.0, WHITE);
                }

                draw_track_preview(track, camera_mouse_position(&camera));
            }
        }

        draw_racer(&physics, &racer);

        next_frame().await;
    }
}
