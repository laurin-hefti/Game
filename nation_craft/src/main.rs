mod gui;
mod constants;

use gui::ui_presets;
use macroquad::prelude::*;

const BUTTON_SIZE_PERCENT: Vec2 = Vec2::new(0.1, 0.1);


#[macroquad::main("Nation Craft")]
async fn main() {

    // Which preset to use (a test or the default)
    let mut gui = ui_presets::default_ui();

    // Main loop
    loop {
        clear_background(BLACK);
        gui.update();
        gui.draw();
        next_frame().await
    }
}
