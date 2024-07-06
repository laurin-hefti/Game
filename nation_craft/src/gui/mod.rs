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

use self::{layout::Section, ui_elements::label::Label};

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
    (width: $width:expr,
     height: $height:expr,
     $(left  : $($left_widgets  :expr),* ;)?
     $(center: $($center_widgets:expr),* ;)?
     $(right : $($right_widgets :expr),* $(;)? )?) => {
        crate::gui::Widget::Layout(crate::gui::Layout::new(
            crate::gui::LayoutType::Horizontal,
            crate::Vec2::new($width, $height), [
            crate::gui::Section(vec![$($( $left_widgets   ),*)?]),
            crate::gui::Section(vec![$($( $center_widgets ),*)?]),
            crate::gui::Section(vec![$($( $right_widgets  ),*)?]),
        ]))
    };
}

#[macro_export]
macro_rules! vertical {
    (width: $width:expr,
     height: $height:expr,

     $(top   : $($top_widgets    :expr),* ;)?
     $(center: $($center_widgets :expr),* ;)?
     $(bottom: $($bottom_widgets :expr),* $(;)? )?) => {
        crate::gui::Widget::Layout(crate::gui::Layout::new(
            crate::gui::LayoutType::Vertical,
            crate::Vec2::new($width, $height), [

            crate::gui::Section(vec![$($( $top_widgets    ),*)?]),
            crate::gui::Section(vec![$($( $center_widgets ),*)?]),
            crate::gui::Section(vec![$($( $bottom_widgets ),*)?]),
        ]))
    };
}

#[macro_export]
macro_rules! vert_centered {
    ($($widgets:expr),*) => {
        vertical!(width: FIT_CONTENT, height: PARENT, center: $($widgets),*;)
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

fn label(text: &str, x: f32, y: f32) -> Widget {
    Widget::Label(Label::new(Vec2::new(x, y), text))
}

const PARENT: f32 = 1.0;
const FIT_CONTENT: f32 = 0.0;

pub mod ui_presets {

    use super::{button, button_with_abs_size, label, quit_all, MainUi, PARENT, FIT_CONTENT};
    use crate::constants::*;

    #[allow(dead_code)]
    pub fn default_ui() -> MainUi {
        MainUi::new(vertical![
            width: PARENT, height: PARENT,
            top:
                // Title bar
                horizontal![
                    width: PARENT,
                    height: TITLE_BAR_HEIGHT,
                    left: button("Profile", || println!("Profile"));
                    center: button("Stats", || println!("Stats"));
                    right: button_with_abs_size("X", quit_all, 50.0, 50.0)
                ],
                // Main window content
                horizontal![
                    width: PARENT,
                    height: 1.0 - TITLE_BAR_HEIGHT - BOTTOM_BAR_HEIGHT,
                    center: vert_centered![button("Here", || println!("Here"))];
                ];
                // Footer
            bottom:
                horizontal![
                    width: PARENT,
                    height: BOTTOM_BAR_HEIGHT,
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
                        width: PARENT, height: 0.2,
                        left:
                            button("left1", || println!("left1"));
                        center:
                            button("center", || println!("center"));
                        right:
                            button("right1", || println!("right1")),
                            button("right2", || println!("right2"));
                    ];
                center:
                    button("center", || println!("center"));
                bottom:
                    button("bottom1", || println!("right1")),
                    button("bottom2", || println!("right2"));
        ])
    }
}
