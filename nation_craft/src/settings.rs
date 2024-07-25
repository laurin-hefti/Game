use std::sync::Mutex;

use crate::gui::style::color_collection::ColorCollection;

pub struct Settings {
    pub gui: GuiSettings,
}

pub struct GuiSettings {
    pub refresh_needed: bool,
    pub colors: ColorCollection,
    pub show_stats: bool,
}

pub static GLOBAL_SETTINGS: Mutex<Settings> = Mutex::new(Settings {
    gui: GuiSettings {
        refresh_needed: true,
        colors: ColorCollection::default(),
        show_stats: false,
    },
});

#[macro_export]
macro_rules! settings {
    () => {
        $crate::settings::GLOBAL_SETTINGS.lock().unwrap()
    };
}
