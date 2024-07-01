mod layout;
mod traits;
mod ui_elements;
mod widget;

// Exports
pub use layout::{Layout, LayoutType};
use log::info;
pub mod style_collection;
pub use traits::UiElement;
pub use ui_elements::button::Button;
pub use widget::Widget;

// Main Ui
use crate::{constants::BUTTON_SIZE, Vec2, GLOBAL_SETTINGS};
use macroquad::window::{screen_height, screen_width};

#[derive(Debug)]
pub struct MainUi {
    root_layout: Layout,
}

impl MainUi {
    pub fn new(root_layout: Layout) -> Self {
        Self { root_layout }
    }

    pub fn from_widgets_vertical<T: Into<Vec<Widget>>>(widgets: T) -> Self {
        Self {
            root_layout: Layout::new(LayoutType::Vertical, widgets.into(), Vec2::new(1.0, 1.0)),
        }
    }

    pub fn check_for_size_overflow(&self, ref_size: &Vec2) {
        self.root_layout.check_for_size_overflow(ref_size);
    }

    pub fn draw(&self) {
        let screen_size = Vec2::new(screen_width(), screen_height());
        GLOBAL_SETTINGS
            .lock()
            .expect("Couldn't acquire lock")
            .gui
            .screen_size = screen_size;
        self.root_layout.draw(&screen_size, &Vec2::new(0., 0.));
    }

    pub fn update(&mut self) {
        self.root_layout.update();
    }
}

pub fn quit_all() {
    info!("Quitting");
    std::process::exit(0);
}

#[macro_export]
macro_rules! layout {
    (horizontal, $width:expr, $( $widgets:expr ),* $(,)? ) => {
        Widget::Layout(Layout::new(
            LayoutType::Horizontal,
            Vec::from([
                $($widgets,)*
            ]),
            Vec2::new($width, 1.0),
        ))
    };
    (vertical, $height:expr, $( $widgets:expr ),* $(,)? ) => {
        Widget::Layout(Layout::new(
            LayoutType::Vertical,
            Vec::from([
                $($widgets,)*
            ]),
            Vec2::new(1.0, $height),
        ))
    };
}

fn button_with_rel_size<N: Into<f32>>(text: &str, on_click: fn(), x: N, y: N) -> Widget {
    Widget::Button(Button::new(Vec2::new(x.into(), y.into()), text, on_click))
}

fn button_with_abs_size<N: Into<f32>>(text: &str, on_click: fn(), x: N, y: N) -> Widget {
    Widget::Button(Button::new(Vec2::new(x.into(), y.into()), text, on_click).use_abs_size())
}

fn button(text: &str, on_click: fn()) -> Widget {
    Widget::Button(Button::new(BUTTON_SIZE, text, on_click).use_abs_size())
}

pub mod ui_presets {
    use crate::gui::button_with_abs_size;
    use macroquad::math::Vec2;

    use super::{
        button, quit_all, ui_elements::label::Label, Button, Layout, LayoutType, MainUi, Widget,
    };
    use crate::constants::*;

    #[allow(dead_code)]
    pub fn default_ui() -> MainUi {
        MainUi::from_widgets_vertical([
            // Title bar
            layout![
                horizontal,
                TITLE_BAR_HEIGHT,
                // Left group (player profile button)
                button("Profile", || println!("Profile")),
                Widget::SpacerPercent((1.0 - 0.1 - 0.1 - 0.03) * 0.5),
                // Middle Group (for stats and stuff)
                button("Stats", || println!("Stats")),
                Widget::SpacerPercent((1.0 - 0.1 - 0.1 - 0.03) * 0.5),
                // Right group (close window button)
                button_with_abs_size("X", quit_all, 50.0, 50.0),
            ],
            // Main window content
            layout![
                vertical,
                1.0 - TITLE_BAR_HEIGHT - BOTTOM_BAR_HEIGHT,
                Widget::SpacerPercent(0.1),
                layout![
                    horizontal,
                    1.0,
                    Widget::Button(Button::new(BUTTON_SIZE, "Here", || {})),
                    Widget::Button(Button::new(BUTTON_SIZE, "Here", || {}))
                ]
            ],
            // Footer
            layout![
                horizontal,
                BOTTOM_BAR_HEIGHT,
                Widget::Label(Label::new(Vec2::new(0.5, 1.0), "Nation Craft")),
                Widget::Label(Label::new(Vec2::new(0.5, 1.0), "Made by LH&LH")),
            ],
        ])
    }

    #[allow(dead_code)]
    pub fn layout_test_ui() -> MainUi {
        MainUi::from_widgets_vertical([
            Widget::SpacerPercent(0.2),
            Widget::Button(
                Button::new(BUTTON_SIZE, "Click me", || println!("Clicked")).use_abs_size(),
            ),
            Widget::SpacerPercent(0.2),
            Widget::Button(
                Button::new(BUTTON_SIZE, "Second", || println!("Second")).use_abs_size(),
            ),
            Widget::Button(Button::new(BUTTON_SIZE, "Third", || println!("third")).use_abs_size()),
            Widget::SpacerPercent(0.2),
            Widget::Button(Button::new(BUTTON_SIZE, "uraa", || println!("uraa")).use_abs_size()),
        ])
    }
}
