use std::sync::Mutex;

use crate::{gui::style::color_collection::ColorCollection, Vec2};

pub struct Settings {
    pub gui: GuiSettings,
}

pub struct GuiSettings {
    pub screen_size: Vec2,
    pub colors: ColorCollection,
}

pub static GLOBAL_SETTINGS: Mutex<Settings> = Mutex::new(Settings {
    gui: GuiSettings {
        screen_size: Vec2::ZERO,
        colors: ColorCollection::default(),
    },
});
