use macroquad::math::vec2;

use super::{
    button, button_with_abs_size, label,
    ui_building::{button_with_rel_size, constraints::PARENT, empty, popup_float},
    MainUi,
};
use crate::{constants, horizontal, settings, vert_centered, vertical};

fn quit_all() {
    log::info!("Quitting");
    std::process::exit(0);
}

#[allow(dead_code)]
pub fn default_ui() -> MainUi {
    MainUi::new(vertical![
        width: PARENT, height: PARENT,
        float:
            if settings!().gui.show_stats {
                popup_float(
                    button_with_rel_size("Popup", quit_all, 1.0, 1.0),
                    vec2(100.0, 100.0),
                    vec2(200.0, 200.0)
                )
            } else { empty() };
        top:
            // Title bar
            horizontal![
                width: PARENT,
                height: constants::TITLE_BAR_HEIGHT,
                left:
                    button("Profile", || println!("Profile")),
                    button("Settings", || println!("Settings"));
                center:
                    button("Stats", || {
                        let mut settings = settings!();
                        settings.gui.show_stats = !settings.gui.show_stats;
                        settings.gui.refresh_needed = true;
                    });
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
    ])
}

#[allow(dead_code)]
pub fn test() -> MainUi {
    MainUi::new(vertical![
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
