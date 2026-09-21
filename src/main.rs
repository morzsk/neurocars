use macroquad::prelude::*;

mod bezier;
pub mod physics;
pub mod racer;
mod track;

use physics::{Physics, init_physics, step_physics};
use racer::{AxisInput, Racer, RacerAction, draw_racer, init_racer, step_racer};
use track::{Track, add_to_track, draw_track, draw_track_preview, init_track};

const TRACK_WIDTH: f32 = 75.0;
const CURVE_SEGMENTS: usize = 40;

struct World {
    physics: Physics,
    racers: Vec<Racer>,
}

fn init_world() -> World {
    let mut physics = init_physics();
    let racer = init_racer(&mut physics);

    World {
        physics,
        racers: vec![racer],
    }
}

fn axis_input(positive: KeyCode, negative: KeyCode) -> AxisInput {
    match (is_key_down(positive), is_key_down(negative)) {
        (true, false) => AxisInput::POSITIVE,
        (false, true) => AxisInput::NEGATIVE,
        _ => AxisInput::NEUTRAL,
    }
}

#[macroquad::main("Neurocar")]
async fn main() {
    let mut world = init_world();
    let mut track = None::<Track>;
    let mut pending_vertices = Vec::<Vec2>::new();
    let mut edit_mode = true;

    loop {
        if is_key_pressed(KeyCode::Tab) {
            edit_mode = !edit_mode;
        }

        let racer_action = if edit_mode {
            RacerAction::default()
        } else {
            RacerAction {
                throttle: axis_input(KeyCode::W, KeyCode::S),
                steer: axis_input(KeyCode::D, KeyCode::A),
            }
        };

        for racer in &world.racers {
            step_racer(&mut world.physics, racer, racer_action);
        }

        step_physics(&mut world.physics);

        if edit_mode {
            let control_pressed =
                is_key_down(KeyCode::LeftControl) || is_key_down(KeyCode::RightControl);

            if control_pressed && is_key_pressed(KeyCode::Z) {
                if pending_vertices.pop().is_none() {
                    match track.as_mut() {
                        Some(track) if track.vertices.len() > 1 => {
                            track.vertices.truncate(track.vertices.len() - 2);
                        }
                        Some(_) => track = None,
                        None => {}
                    }
                }
            }

            if is_mouse_button_pressed(MouseButton::Left) {
                let (x, y) = mouse_position();
                let point = vec2(x, y);

                if let Some(track) = track.as_mut() {
                    pending_vertices.push(point);

                    if let [control, end] = pending_vertices.as_slice() {
                        add_to_track(track, *control, *end);
                        pending_vertices.clear();
                    }
                } else {
                    track = Some(init_track(TRACK_WIDTH, point));
                }
            }
        }

        clear_background(BLACK);

        if let Some(track) = &track {
            draw_track(track, CURVE_SEGMENTS);

            if edit_mode {
                for vertex in &track.vertices {
                    draw_circle(vertex.x, vertex.y, 4.0, WHITE);
                }

                if let [control] = pending_vertices.as_slice() {
                    let (x, y) = mouse_position();
                    draw_track_preview(track, *control, vec2(x, y), CURVE_SEGMENTS);
                }
            }
        }

        if edit_mode {
            for vertex in &pending_vertices {
                draw_circle(vertex.x, vertex.y, 4.0, WHITE);
            }
        }

        for racer in &world.racers {
            draw_racer(&world.physics, racer);
        }

        next_frame().await;
    }
}
