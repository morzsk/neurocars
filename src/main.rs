use macroquad::prelude::*;

mod bezier;
mod physics;
mod track;

use physics::PhysicsWorld;
use track::{Track, add_to_track, draw_track, draw_track_preview, init_track};

const TRACK_WIDTH: f32 = 75.0;
const CURVE_SEGMENTS: usize = 40;

#[macroquad::main("Neurocar")]
async fn main() {
    let mut physics = PhysicsWorld::new();
    let mut track = None::<Track>;
    let mut pending_vertices = Vec::<Vec2>::new();
    let mut edit_mode = true;

    loop {
        physics.step();

        if is_key_pressed(KeyCode::Tab) {
            edit_mode = !edit_mode;
        }

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

        next_frame().await;
    }
}
