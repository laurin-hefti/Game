mod gui;
mod constants;

use std::sync::Mutex;
use gui::ui_presets;
use macroquad::prelude::*;

pub struct GuiSettings {
    pub screen_size: Vec2,
}

pub struct Settings {
    gui: GuiSettings,
}

static GLOBAL_SETTINGS: Mutex<Settings> = Mutex::new(Settings {
    gui: GuiSettings {
        screen_size: Vec2::ZERO,
    },
});

#[macroquad::main("Nation Craft")]
async fn main() {
    // Set up logging system
    let env = env_logger::Env::default()
        .filter_or("RUST_LOG", "info")
        .write_style_or("RUST_LOG_STYLE", "none");

    env_logger::init_from_env(env);
    info!("Starting");

    // Draw one frame to get the screen size
    next_frame().await;
    let screen_size = Vec2::new(screen_width(), screen_height());
    GLOBAL_SETTINGS.lock().unwrap().gui.screen_size = screen_size;

    // Which preset to use (a test or the default)
    let mut gui = ui_presets::default_ui();
    #[cfg(debug_assertions)]
    gui.check_for_size_overflow(&screen_size);

    // Main loop
    loop {
        clear_background(GRAY);
        gui.update();
        gui.draw();
        gui.check_for_size_overflow(&screen_size);

        // Check for low FPS in debug builds
        #[cfg(debug_assertions)]
        if get_frame_time() > 1.0 / 30.0 {
            info!("FPS: {}", get_fps());
        }

        next_frame().await
    }
}
