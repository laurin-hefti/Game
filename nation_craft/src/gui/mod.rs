mod layout;
mod traits;
mod ui_elements;
mod widget;

// Exports
pub use layout::{Layout, LayoutType};
pub use traits::UiElement;
pub use ui_elements::button::Button;
pub use ui_elements::label::Label;
pub use widget::Widget;

// Main Ui
use crate::Vec2;
use macroquad::window::{screen_height, screen_width};

#[derive(Debug)]
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

    pub fn update(&mut self) {
        self.root_layout.update();
    }
}

pub mod ui_presets {
    use macroquad::math::Vec2;

    use super::{Button, Label, Layout, LayoutType, MainUi, Widget};
    use crate::constants::BUTTON_SIZE_PERCENT;

    #[allow(dead_code)]
    pub fn default_ui() -> MainUi {
        MainUi::new(Layout::new(
            LayoutType::Vertical,
            Vec::from([
                Widget::Layout(Layout::new(
                    LayoutType::Horizontal,
                    Vec::from([
                        Widget::Button(Button::new(BUTTON_SIZE_PERCENT, "1.0", || {
                            println!("1.0 click")
                        })),
                        Widget::SpacerPercent(0.35),
                        Widget::Label(Label::new(BUTTON_SIZE_PERCENT, "1.1")),
                        Widget::SpacerPercent(0.35),
                        Widget::Button(Button::new(BUTTON_SIZE_PERCENT, "1.2", || {
                            println!("1.2 click")
                        })),
                    ]),
                    Some(Vec2::new(1.0, 0.5)),
                )),
                Widget::Layout(Layout::new(
                    LayoutType::Horizontal,
                    Vec::from([
                        Widget::Button(Button::new(BUTTON_SIZE_PERCENT, "2.0", || {
                            println!("2.0 click")
                        })),
                        Widget::SpacerPercent(0.35),
                        Widget::Label(Label::new(BUTTON_SIZE_PERCENT, "2.1")),
                        Widget::SpacerPercent(0.35),
                        Widget::Button(Button::new(BUTTON_SIZE_PERCENT, "2.2", || {
                            println!("2.2 click")
                        })),
                    ]),
                    Some(Vec2::new(1.0, 0.5)),
                )),
            ]),
            None,
        ))
    }

    #[allow(dead_code)]
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
