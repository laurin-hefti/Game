use super::widgets::{Widget, Button, Label};
use crate::{constants::BUTTON_SIZE, Vec2};

pub mod constraints {
    pub const PARENT: f32 = 1.0;
    pub const FIT_CONTENT: f32 = 0.0;
}

#[macro_export]
macro_rules! horizontal {
    (width: $width:expr,
     height: $height:expr,
     $(left  : $($left_widgets  :expr),* ;)?
     $(center: $($center_widgets:expr),* ;)?
     $(right : $($right_widgets :expr),* $(;)? )?) => {
        super::widgets::Widget::Layout(super::widgets::Layout::new(
            super::widgets::layout::LayoutType::Horizontal,
            crate::Vec2::new($width, $height), [
            super::widgets::layout::Section(vec![$($( $left_widgets   ),*)?]),
            super::widgets::layout::Section(vec![$($( $center_widgets ),*)?]),
            super::widgets::layout::Section(vec![$($( $right_widgets  ),*)?]),
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
        super::widgets::Widget::Layout(super::widgets::Layout::new(
            super::widgets::layout::LayoutType::Vertical,
            crate::Vec2::new($width, $height), [

            super::widgets::layout::Section(vec![$($( $top_widgets    ),*)?]),
            super::widgets::layout::Section(vec![$($( $center_widgets ),*)?]),
            super::widgets::layout::Section(vec![$($( $bottom_widgets ),*)?]),
        ]))
    };
}

#[macro_export]
macro_rules! vert_centered {
    ($($widgets:expr),*) => {
        vertical!(width: super::FIT_CONTENT, height: super::PARENT, center: $($widgets),*;)
    };
}

#[macro_export]
macro_rules! horiz_centered {
    ($($widgets:expr),*) => {
        horizontal!(width: PARENT, height: FIT_CONTENT, center: $($widgets),*;)
    };
}

#[allow(dead_code)]
pub fn button_with_rel_size<N: Into<f32>>(text: &str, on_click: fn(), x: N, y: N) -> Widget {
    Widget::Button(Button::new(Vec2::new(x.into(), y.into()), text, on_click))
}

#[allow(dead_code)]
pub fn button_with_abs_size<N: Into<f32>>(text: &str, on_click: fn(), x: N, y: N) -> Widget {
    Widget::Button(Button::new(Vec2::new(x.into(), y.into()), text, on_click).use_abs_size())
}

#[allow(dead_code)]
pub fn button(text: &str, on_click: fn()) -> Widget {
    Widget::Button(Button::new(BUTTON_SIZE, text, on_click).use_abs_size())
}

#[allow(dead_code)]
pub fn label(text: &str, x: f32, y: f32) -> Widget {
    Widget::Label(Label::new(Vec2::new(x, y), text))
}
