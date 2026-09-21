use macroquad::prelude::{KeyCode, is_key_down, mouse_wheel};

use crate::controls::Controls;

pub struct Inputs {
    pub controls: Controls,
    pub zoom: ZoomInput,
}

pub struct ZoomInput {
    pub keyboard_direction: f32,
    pub mouse_wheel_delta: f32,
}

pub fn read() -> Inputs {
    let forward = is_key_down(KeyCode::W) || is_key_down(KeyCode::Up);
    let backward = is_key_down(KeyCode::S) || is_key_down(KeyCode::Down);
    let left = is_key_down(KeyCode::A) || is_key_down(KeyCode::Left);
    let right = is_key_down(KeyCode::D) || is_key_down(KeyCode::Right);

    let zoom_in = is_key_down(KeyCode::Equal) || is_key_down(KeyCode::KpAdd);
    let zoom_out = is_key_down(KeyCode::Minus) || is_key_down(KeyCode::KpSubtract);
    let keyboard_direction = match (zoom_in, zoom_out) {
        (true, false) => 1.0,
        (false, true) => -1.0,
        _ => 0.0,
    };

    Inputs {
        controls: Controls::from_direction_inputs(forward, backward, left, right),
        zoom: ZoomInput {
            keyboard_direction,
            mouse_wheel_delta: mouse_wheel().1,
        },
    }
}
