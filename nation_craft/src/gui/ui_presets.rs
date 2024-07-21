use macroquad::math::vec2;

use super::{
    button, button_with_abs_size, label, ui_building::{constraints::PARENT, popup_float}, MainUi
};
use crate::{
    horizontal, vert_centered, vertical,
    constants,
};

fn quit_all() {
    log::info!("Quitting");
    std::process::exit(0);
}

#[allow(dead_code)]
pub fn default_ui() -> MainUi {
    MainUi::new(
        vertical![
            width: PARENT, height: PARENT,
            top:
                // Float popup
                popup_float(
                    button_with_abs_size("Popup", quit_all, 100.0, 100.0),
                    vec2(100.0, 100.0),
                    vec2(200.0, 200.0)
                    ),
                // Title bar
                horizontal![
                    width: PARENT,
                    height: constants::TITLE_BAR_HEIGHT,
                    left:
                        button("Profile", || println!("Profile")),
                        button("Settings", || println!("Settings"));
                    center:
                        button("Stats", || println!("Stats"));
                    right:
                        button_with_abs_size("X", quit_all, 50.0, 50.0)
                ],
                // Main window content
                horizontal![
                    width: PARENT,
                    height: 1.0 - constants::TITLE_BAR_HEIGHT - constants::BOTTOM_BAR_HEIGHT,
                    center: vert_centered![button("Here", || println!("Here"))];
                ];
                // Footer
            bottom:
                horizontal![
                    width: PARENT,
                    height: constants::BOTTOM_BAR_HEIGHT,
                    left:
                        label("Nation Craft", 0.5, 1.0),
                        label("By LH && LH", 0.5, 1.0);
                ];
        ],
    )
}

#[allow(dead_code)]
pub fn test() -> MainUi {
    MainUi::new(
        vertical![
            width: PARENT, height: PARENT,
            top:
                horizontal![
                    width: PARENT,
                    height: constants::TITLE_BAR_HEIGHT,
                    left:
                        button("left1", || println!("left1")),
                        button("left1", || println!("left1"));
                    center:
                        button("center", || println!("center"));
                    right:
                        button_with_abs_size("X", quit_all, 50.0, 50.0);
                ],
                horizontal![
                    width: PARENT, height: 1.0 - 0.1,
                    center:
                        vert_centered![button("left2", || println!("left2"))];
                ];
            center:
                button("center", || println!("center"));
            bottom:
                horizontal![
                    width: PARENT,
                    height: constants::BOTTOM_BAR_HEIGHT,
                    left:
                        label("Nation Craft", 0.5, 1.0),
                        label("By LH && LH", 0.5, 1.0);
                ];
            ])
}
