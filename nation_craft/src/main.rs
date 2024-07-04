mod constants;
mod gui;

use constants::BG_COLOR_PRIMARY;
use gui::ui_presets;
use macroquad::{prelude::*, ui::root_ui};
use std::sync::Mutex;

use crate::gui::style_collection;

mod locic;
use crate::locic::*;


const LAURINWILLNURTESTEN: bool = true;

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
    if LAURINWILLNURTESTEN {
        
        let w0: locic::World<2,1,4,0,1> = newWorld(getResource(defaultWorldResources),Map1);
        //let w1 = initWorld![iron];
        println!("{}", w0.avaliableResources[1].name);

        std::process::exit(0);
    }

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
    let screen_size = Vec2::new(screen_width(), screen_height());
    GLOBAL_SETTINGS
        .lock()
        .expect("Couldn't acquire lock")
        .gui
        .screen_size = screen_size;

    // Which preset to use (a test or the default)
    let mut gui = ui_presets::default_ui();
    #[cfg(debug_assertions)]
    gui.check_for_size_overflow(&screen_size);

    // Style
    let skin = style_collection::default_skin();
    root_ui().push_skin(&skin);

    // Main loop
    loop {
        clear_background(BG_COLOR_PRIMARY);
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
