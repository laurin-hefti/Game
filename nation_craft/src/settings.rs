use std::sync::Mutex;

use crate::Vec2;

pub struct Settings {
    gui: GuiSettings,
}

pub struct GuiSettings {
    pub screen_size: Vec2,
}

pub static GLOBAL_SETTINGS: Mutex<Settings> = Mutex::new(Settings {
    gui: GuiSettings {
        screen_size: Vec2::ZERO,
    },
});
