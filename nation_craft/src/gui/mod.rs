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

use self::layout::Section;

#[derive(Debug)]
pub struct MainUi {
    root_layout: Layout,
}

impl MainUi {
    pub fn new(root_layout: impl Into<Layout>) -> Self {
        Self {
            root_layout: root_layout.into(),
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
macro_rules! horizontal {
    ($width:expr;
     $(left  : $($left_widgets  :expr),* ;)?
     $(center: $($center_widgets:expr),* ;)?
     $(right : $($right_widgets :expr),* $(;)? )?) => {
        crate::gui::Widget::Layout(crate::gui::Layout::new(
            crate::gui::LayoutType::Horizontal,
            crate::Vec2::new($width, 1.0), [
            crate::gui::Section(vec![$($($left_widgets  ),*)?]),
            crate::gui::Section(vec![$($($center_widgets),*)?]),
            crate::gui::Section(vec![$($($right_widgets ),*)?]),
        ]))
    };
}

#[macro_export]
macro_rules! vertical {
    ($height:expr;
     $(top   : $($top_widgets    :expr),* ;)?
     $(center: $($center_widgets :expr),* ;)?
     $(bottom: $($bottom_widgets :expr),* $(;)? )?) => {
        crate::gui::Widget::Layout(crate::gui::Layout::new(
            crate::gui::LayoutType::Vertical,
            crate::Vec2::new(1.0, $height), [
            crate::gui::Section(vec![$($($top_widgets    ),*)?]),
            crate::gui::Section(vec![$($($center_widgets ),*)?]),
            crate::gui::Section(vec![$($($bottom_widgets ),*)?]),
        ]))
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

    use macroquad::math::Vec2;

    use super::{
        button, button_with_abs_size, button_with_rel_size, quit_all, ui_elements::label::Label, Button, MainUi, Widget
    };
    use crate::constants::*;

    #[allow(dead_code)]
    pub fn default_ui() -> MainUi {
        MainUi::new(vertical![1.0;
            top:
                // Title bar
                horizontal![TITLE_BAR_HEIGHT;
                    left: button("Profile", || println!("Profile"));
                    center: button("Stats", || println!("Stats"));
                    right: button_with_abs_size("X", quit_all, 50.0, 50.0)
                ],
                // Main window content
                vertical![1.0 - TITLE_BAR_HEIGHT - BOTTOM_BAR_HEIGHT;
                    top: horizontal![0.0;
                        center: button("Here", || println!("Here"));
                    ];
                ],
                // Footer
                horizontal![BOTTOM_BAR_HEIGHT;
                    left:
                        Widget::Label(Label::new(Vec2::new(0.5, 1.0), "Nation Craft")),
                        Widget::Label(Label::new(Vec2::new(0.5, 1.0), "Made by LH&LH"));
                ];
        ])
    }

    #[allow(dead_code)]
    pub fn test() -> MainUi {
        MainUi::new(vertical![1.0;
                top:
                    button("left1", || println!("left1"));
                center:
                    // button_with_rel_size("vercenter2", || println!("center2"), 0.5, 0.25),
                    button("left2", || println!("left2")),
                    horizontal![0.0;
                        left:
                            button("l1", || println!("right1"));
                        center:
                            button("l2", || println!("right1"));
                        right:
                            button("l3", || println!("right1"));
                    ];
                bottom:
                    button("right1", || println!("right1")),
                    button("right2", || println!("right2"));
        ])
    }
}
