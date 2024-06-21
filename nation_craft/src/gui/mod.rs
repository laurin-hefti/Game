mod layout;
mod traits;
mod ui_elements;
mod widget;

// Exports
pub use layout::{Layout, LayoutType};
use log::info;
pub use traits::UiElement;
pub use ui_elements::button::Button;
pub use widget::Widget;

// Main Ui
use crate::{Vec2, GLOBAL_SETTINGS};
use macroquad::window::{screen_height, screen_width};

#[derive(Debug)]
pub struct MainUi {
    root_layout: Layout,
}

impl MainUi {
    pub fn new(root_layout: Layout) -> Self {
        Self { root_layout }
    }

    pub fn from<W: Into<Vec<Widget>>>(widgets: W) -> Self {
        Self {
            root_layout: Layout::new(LayoutType::Horizontal, widgets.into(), Vec2::ONE),
        }
    }

    pub fn check_for_size_overflow(&self, ref_size: &Vec2) {
        self.root_layout.check_for_size_overflow(ref_size);
    }

    pub fn draw(&self) {
        let screen_size = Vec2::new(screen_width(), screen_height());
        GLOBAL_SETTINGS.lock().unwrap().gui.screen_size = screen_size;
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

pub mod ui_presets {
    use macroquad::math::Vec2;

    use super::{quit_all, ui_elements::label::Label, Button, Layout, LayoutType, MainUi, Widget};
    use crate::constants::*;

    #[allow(dead_code)]
    pub fn default_ui() -> MainUi {
        MainUi::new(Layout::new(
            LayoutType::Vertical,
            Vec::from([
                // Title bar
                Widget::Layout(Layout::new(
                    LayoutType::Horizontal,
                    Vec::from([
                        // Left group (player profile button)
                        Widget::Button(Button::new(Vec2::new(0.1, 1.0), "Player Profile", || {
                            println!("Clicked")
                        })),
                        Widget::SpacerPercent((1.0 - 0.1 - 0.1 - 0.03) * 0.5),
                        // Middle Group (for stats and stuff)
                        Widget::Button(Button::new(Vec2::new(0.1, 1.0), "Show stats", || ())),
                        Widget::SpacerPercent((1.0 - 0.1 - 0.1 - 0.03) * 0.5),
                        // Right group (close window button)
                        Widget::Button(Button::new(
                            Vec2::new(0.03, 0.03 / TITLE_BAR_HEIGHT),
                            "X",
                            quit_all,
                        )),
                    ]),
                    Vec2::new(1.0, TITLE_BAR_HEIGHT),
                )),
                // Main window content
                Widget::Layout(Layout::new(
                    LayoutType::Horizontal,
                    Vec::from([
                        Widget::Button(
                            Button::new(BUTTON_SIZE, "Click me", || println!("Clicked"))
                                .use_abs_size(),
                        ),
                        Widget::Button(
                            Button::new(BUTTON_SIZE, "Click me", || println!("Clicked"))
                                .use_abs_size(),
                        ),
                    ]),
                    Vec2::new(1.0, 1.0 - TITLE_BAR_HEIGHT - BOTTOM_BAR_HEIGHT - 0.01),
                )),
                // Footer
                Widget::Layout(Layout::new(
                    LayoutType::Horizontal,
                    Vec::from([
                        Widget::Label(Label::new(Vec2::new(0.5, 1.0), "Nation Craft")),
                        Widget::Label(Label::new(Vec2::new(0.5, 1.0), "Made by LH&LH")),
                    ]),
                    Vec2::new(1.0, BOTTOM_BAR_HEIGHT),
                )),
            ]),
            Vec2::ONE,
        ))
    }

    #[allow(dead_code)]
    pub fn layout_test_ui() -> MainUi {
        MainUi::new(Layout::new(
            LayoutType::Vertical,
            Vec::from([
                Widget::SpacerPercent(0.2),
                Widget::Button(Button::new(BUTTON_SIZE, "Click me", || println!("Clicked"))),
                Widget::SpacerPercent(0.2),
                Widget::Button(Button::new(BUTTON_SIZE, "Second", || println!("Second"))),
                Widget::Button(Button::new(BUTTON_SIZE, "Third", || println!("third"))),
                Widget::SpacerPercent(0.2),
                Widget::Button(Button::new(BUTTON_SIZE, "uraa", || println!("uraa"))),
            ]),
            Vec2::new(0.5, 0.5), // Size == 100% of parent
        ))
    }
}
