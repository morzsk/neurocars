use macroquad::prelude::*;

mod bezier;
mod camera;
pub mod neural;
pub mod physics;
pub mod racer;
pub mod simulation;
mod track;
pub mod utils;

use camera::{CameraMode, apply_camera, camera_mouse_position, init_camera, step_camera};
use physics::{init_physics, step_physics};
use racer::{
    RacerAction, draw_racer, evaluate_racer_action, fire_sensors, init_sensors, racer_position,
    step_racer,
};
use simulation::{Simulation, init_simulation, rerun_simulation};
use track::{
    Track, add_to_track, draw_track, draw_track_preview, init_track, remove_from_track,
    update_collider,
};

const TRACK_WIDTH: f32 = 400.0;
const CURVE_T_STEP: f32 = 1.0 / 40.0;
const DEFAULT_PIXELS_PER_METER: f32 = 1.0;

#[macroquad::main("Neurocar")]
async fn main() {
    let mut physics = init_physics();
    let mut spawners = Vec::<Vec2>::new();
    let mut simulation = None::<Simulation>;
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

        if simulation.is_none() && !spawners.is_empty() && is_key_pressed(KeyCode::Enter) {
            simulation = Some(init_simulation(&mut physics, &spawners));
        }

        if is_key_pressed(KeyCode::R)
            && let Some(current_simulation) = simulation.take()
        {
            simulation = Some(rerun_simulation(&mut physics, current_simulation));
        }

        if let Some(simulation) = &simulation {
            for simulation_racer in &simulation.racers {
                let racer = &simulation_racer.racer;
                let mut sensors = init_sensors();
                fire_sensors(&physics, racer, &mut sensors);

                let racer_action = if edit_mode {
                    RacerAction::default()
                } else {
                    evaluate_racer_action(racer, &sensors)
                };

                step_racer(&mut physics, racer, racer_action);
            }
        }

        step_physics(&mut physics);

        camera.mode = match (
            edit_mode,
            simulation
                .as_ref()
                .and_then(|simulation| simulation.racers.first()),
        ) {
            (false, Some(simulation_racer)) => {
                CameraMode::Follow(racer_position(&physics, &simulation_racer.racer))
            }
            _ => CameraMode::Free,
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

            if is_mouse_button_pressed(MouseButton::Right) {
                spawners.push(camera_mouse_position(&camera));
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

        for spawner in &spawners {
            draw_circle(spawner.x, spawner.y, 12.0, GREEN);
        }

        if let Some(simulation) = &simulation {
            for simulation_racer in &simulation.racers {
                draw_racer(&physics, &simulation_racer.racer);
            }
        }

        next_frame().await;
    }
}
