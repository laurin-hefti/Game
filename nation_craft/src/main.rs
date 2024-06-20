mod gui;
mod constants;

use gui::ui_presets;
use macroquad::prelude::*;


#[macroquad::main("Nation Craft")]
async fn main() {

    // Which preset to use (a test or the default)
    let mut gui = ui_presets::default_ui();

    // Main loop
    loop {
        clear_background(GRAY);
        gui.update();
        gui.draw();
        next_frame().await
    }
}
