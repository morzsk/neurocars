use macroquad::prelude::*;

mod bezier;
mod controls;
mod inputs;
mod physics;
mod racer;
mod tracks;
mod visualizer;
mod world;

use racer::RacerGeometry;
use tracks::Track;
use world::World;

const TRACK_SIZE_METERS: f32 = 200.0;
const TRACK_WIDTH_METERS: f32 = 50.0;
const DEFAULT_PIXELS_PER_METER: f32 = 6.0;
const MIN_PIXELS_PER_METER: f32 = 1.0;
const MAX_PIXELS_PER_METER: f32 = 500.0;
const KEYBOARD_ZOOM_FACTOR_PER_SECOND: f32 = 2.0;
const MOUSE_WHEEL_ZOOM_STEP: f32 = 1.2;

#[macroquad::main("Neurocar")]
async fn main() {
    let track = Track::new(8, TRACK_SIZE_METERS, TRACK_WIDTH_METERS);
    let spawn_position = track.spawn_position(0.5);
    let mut world = World::new(track);
    let racer_id = world.spawn_racer_at(RacerGeometry::new(5.5, 2.0, 3.6, 2.3), spawn_position);
    let mut pixels_per_meter = DEFAULT_PIXELS_PER_METER;

    loop {
        let inputs = inputs::read();

        world.apply_actions(racer_id, inputs.controls);
        world.step();

        pixels_per_meter *=
            KEYBOARD_ZOOM_FACTOR_PER_SECOND.powf(inputs.zoom.keyboard_direction * get_frame_time());
        pixels_per_meter *= MOUSE_WHEEL_ZOOM_STEP.powf(inputs.zoom.mouse_wheel_delta);
        pixels_per_meter = pixels_per_meter.clamp(MIN_PIXELS_PER_METER, MAX_PIXELS_PER_METER);

        clear_background(BLACK);
        visualizer::draw(&world, pixels_per_meter);

        next_frame().await;
    }
}
