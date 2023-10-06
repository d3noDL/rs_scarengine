use macroquad::prelude::*;

#[macroquad::main("Scarengine")]
async fn main() {
    //* pre loop vars and logic go here */
    loop {
        clear_background(Color { r: 100.0 / 255., g: 149. / 255., b: 237. / 255., a: 255. / 255. });
        //* main loop logic goes here */

            draw_text("SCARENGINE 0.1", screen_width() / 2. - 140., screen_height() / 2., 48., WHITE);

        //* main loop logic ends here */
        next_frame().await;
    }
}
