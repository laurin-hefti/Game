mod gui;
mod traits;
mod constants;
mod settings;

use constants::BG_COLOR_PRIMARY;
use gui::ui_presets;
use macroquad::{miniquad::window::screen_size, prelude::*, ui::root_ui};

use crate::gui::style::style_collection;

#[macroquad::main("Nation Craft")]
async fn main() {
    // Set up logging system
    {
        let env = env_logger::Env::default()
            .filter_or("RUST_LOG", "info")
            .write_style_or("RUST_LOG_STYLE", "none");

        env_logger::init_from_env(env);
        info!("Starting");
    }

    // Draw one frame to get the screen size
    next_frame().await;
    let window_size = Vec2::from(screen_size());

    // Which preset to use (a test or the default)
    let mut gui = ui_presets::default_ui();
    #[cfg(debug_assertions)]
    gui.check_for_size_overflow(window_size);

    // Style
    let skin = style_collection::default_skin(window_size);
    root_ui().push_skin(&skin);

    // Main loop
    loop {
        let screen_size = Vec2::from(screen_size());

        clear_background(BG_COLOR_PRIMARY);
        gui.draw(screen_size);

        gui.update();

        // Check for low FPS in debug builds
        #[cfg(debug_assertions)]
        if get_frame_time() > 1.0 / 30.0 {
            info!("FPS: {}", get_fps());
        }

        next_frame().await
    }
}
