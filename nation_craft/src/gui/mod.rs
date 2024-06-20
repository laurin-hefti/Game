mod layout;
mod traits;
mod ui_elements;
mod widget;

// Exports
pub use layout::{Layout, LayoutType};
pub use traits::UiElement;
pub use ui_elements::button::Button;
pub use widget::Widget;

// Main Ui
use crate::Vec2;
use macroquad::window::{screen_height, screen_width};

pub struct MainUi {
    root_layout: Layout,
}

impl MainUi {
    pub fn new(root_layout: Layout) -> Self {
        Self { root_layout }
    }

    pub fn from<W>(widgets: W) -> Self
    where
        W: Into<Vec<Widget>>,
    {
        Self {
            root_layout: Layout::new(LayoutType::Horizontal, widgets.into(), None),
        }
    }

    pub fn draw(&self) {
        let screen_size = Vec2::new(screen_width(), screen_height());
        self.root_layout.draw(&screen_size, &Vec2::new(0., 0.));
    }

    pub fn update(&mut self) {}
}

pub mod ui_presets {
    use macroquad::math::Vec2;

    use super::{Button, Layout, LayoutType, MainUi, Widget};
    use crate::BUTTON_SIZE_PERCENT;

    pub fn default_ui() -> MainUi {
        MainUi::from([
            Widget::Button(Button::new(BUTTON_SIZE_PERCENT, "Click me", || {
                println!("Clicked")
            })),
            Widget::SpacerPercent(0.3),
            Widget::Button(Button::new(BUTTON_SIZE_PERCENT, "Second", || {
                println!("Second")
            })),
            Widget::SpacerPercent(0.4),
            Widget::Button(Button::new(BUTTON_SIZE_PERCENT, "Third", || {
                println!("third")
            })),
        ])
    }

    pub fn layout_test_ui() -> MainUi {
        MainUi::new(Layout::new(
            LayoutType::Vertical,
            Vec::from([
                Widget::SpacerPercent(0.2),
                Widget::Button(Button::new(BUTTON_SIZE_PERCENT, "Click me", || {
                    println!("Clicked")
                })),
                Widget::SpacerPercent(0.2),
                Widget::Button(Button::new(BUTTON_SIZE_PERCENT, "Second", || {
                    println!("Second")
                })),
                Widget::Button(Button::new(BUTTON_SIZE_PERCENT, "Third", || {
                    println!("third")
                })),
                Widget::SpacerPercent(0.2),
                Widget::Button(Button::new(BUTTON_SIZE_PERCENT, "uraa", || {
                    println!("uraa")
                })),
            ]),
            Some(Vec2::new(0.5, 0.5)), // Size == 100% of parent
        ))
    }
}
