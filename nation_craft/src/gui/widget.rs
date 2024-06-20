use super::{ui_elements::label::Label, Button, Layout, UiElement};
use crate::Vec2;

#[derive(Debug)]
pub enum Widget {
    Layout(Layout),
    Button(Button),
    Label(Label),
    SpacerPercent(f32),
}

impl Widget {
    pub fn abs_size(&self, ref_size: &Vec2) -> Vec2 {
        match self {
            Widget::Layout(widget) => widget.abs_size(ref_size),
            Widget::Button(button) => button.abs_size(ref_size),
            Widget::Label(label) => label.abs_size(ref_size),
            Widget::SpacerPercent(val_percent_parent) => *val_percent_parent * *ref_size,
        }
    }
}

impl UiElement for Widget {
    fn draw(&self, ref_size: &Vec2, pos: &Vec2) {
        match self {
            Widget::Layout(widget) => widget.draw(ref_size, pos),
            Widget::Button(button) => button.draw(ref_size, pos),
            Widget::Label(label) => label.draw(ref_size, pos),
            Widget::SpacerPercent(_) => (),
        }
    }

    fn update(&mut self) {
        match self {
            Widget::Layout(widget) => widget.update(),
            Widget::Button(button) => button.update(),
            Widget::Label(label) => label.update(),
            Widget::SpacerPercent(_) => (),
        }
    }
}

