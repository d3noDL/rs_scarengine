use macroquad::prelude::*;

#[macroquad::main("Scarengine")]
async fn main() {
    //* pre loop vars and logic go here */
    loop {
        clear_background(Color {
            r: 100.0 / 255.0,
            g: 149.0 / 255.0,
            b: 237.0 / 255.0,
            a: 1.0,
        });
        //* main loop logic goes here */

        draw_text(
            "SCARENGINE 0.1",
            screen_width() / 2.0 - 140.0,
            screen_height() / 2.0,
            48.0,
            WHITE
        );

        //* main loop logic ends here */
        next_frame().await;
    }
}
