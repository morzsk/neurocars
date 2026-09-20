use macroquad::prelude::*;

const FONT_SIZE: f32 = 20.0;

#[macroquad::main("MyGame")]
async fn main() {
    loop {
        clear_background(BLACK);

        draw_text(
            "hello world",
            screen_width() / 2.0,
            screen_height() / 2.0,
            FONT_SIZE,
            WHITE,
        );

        next_frame().await
    }
}
