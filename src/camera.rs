use macroquad::prelude::{
    Camera2D, KeyCode, Vec2, get_frame_time, is_key_down, is_key_pressed, mouse_position,
    mouse_wheel, screen_height, screen_width, set_camera, vec2,
};

const MIN_PIXELS_PER_METER: f32 = 0.2;
const MAX_PIXELS_PER_METER: f32 = 5.0;
const SCROLL_SCALE_STEP: f32 = 1.1;
const PAN_SPEED_PIXELS_PER_SECOND: f32 = 500.0;
const FOLLOW_SPEED: f32 = 8.0;

pub struct Camera {
    pub mode: CameraMode,
    target: Vec2,
    pixels_per_meter: f32,
}

#[derive(Clone, Copy)]
pub enum CameraMode {
    Follow(Vec2),
    Free,
}

pub fn init_camera(pixels_per_meter: f32) -> Camera {
    Camera {
        mode: CameraMode::Free,
        target: Vec2::ZERO,
        pixels_per_meter,
    }
}

pub fn step_camera(camera: &mut Camera) {
    let (_, scroll) = mouse_wheel();
    let keyboard_zoom = if is_key_pressed(KeyCode::Equal) || is_key_pressed(KeyCode::KpAdd) {
        1.0
    } else if is_key_pressed(KeyCode::Minus) || is_key_pressed(KeyCode::KpSubtract) {
        -1.0
    } else {
        0.0
    };

    camera.pixels_per_meter = (camera.pixels_per_meter
        * SCROLL_SCALE_STEP.powf(scroll + keyboard_zoom))
    .clamp(MIN_PIXELS_PER_METER, MAX_PIXELS_PER_METER);

    match camera.mode {
        CameraMode::Follow(target) => {
            let amount = 1.0 - (-FOLLOW_SPEED * get_frame_time()).exp();
            camera.target = camera.target.lerp(target, amount);
        }
        CameraMode::Free => {
            let horizontal = axis(KeyCode::Right, KeyCode::Left);
            let vertical = axis(KeyCode::Down, KeyCode::Up);
            let direction = vec2(horizontal, vertical).normalize_or_zero();

            camera.target += direction * PAN_SPEED_PIXELS_PER_SECOND * get_frame_time()
                / camera.pixels_per_meter;
        }
    }
}

pub fn camera_mouse_position(camera: &Camera) -> Vec2 {
    let (x, y) = mouse_position();
    macroquad_camera(camera).screen_to_world(vec2(x, y))
}

pub fn apply_camera(camera: &Camera) {
    set_camera(&macroquad_camera(camera));
}

fn macroquad_camera(camera: &Camera) -> Camera2D {
    Camera2D {
        target: camera.target,
        zoom: vec2(
            2.0 * camera.pixels_per_meter / screen_width(),
            2.0 * camera.pixels_per_meter / screen_height(),
        ),
        ..Default::default()
    }
}

fn axis(positive: KeyCode, negative: KeyCode) -> f32 {
    match (is_key_down(positive), is_key_down(negative)) {
        (true, false) => 1.0,
        (false, true) => -1.0,
        _ => 0.0,
    }
}
