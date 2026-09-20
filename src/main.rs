use macroquad::prelude::*;

mod physics;

use physics::PhysicsWorld;

#[macroquad::main("Neurocar")]
async fn main() {
    let mut physics = PhysicsWorld::new();

    loop {
        physics.step();

        clear_background(BLACK);

        next_frame().await;
    }
}
