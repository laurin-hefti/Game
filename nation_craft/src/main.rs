mod gui;
mod constants;

use gui::ui_presets;
use macroquad::prelude::*;


#[macroquad::main("Nation Craft")]
async fn main() {
    // Set up logging system
    let env = env_logger::Env::default()
        .filter_or("RUST_LOG", "info")
        .write_style_or("RUST_LOG_STYLE", "none");

    env_logger::init_from_env(env);
    info!("Starting");

    // Which preset to use (a test or the default)
    let mut gui = ui_presets::default_ui();

    // Main loop
    loop {
        clear_background(GRAY);
        gui.update();
        gui.draw();

        // Check for low FPS in debug builds
        #[cfg(debug_assertions)]
        if get_frame_time() > 1.0 / 30.0 {
            info!("FPS: {}", get_fps());
        }

        next_frame().await
    }
}
