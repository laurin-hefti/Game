use log::warn;

use super::{
    layout::{LayoutType, Section}, Button, PopupFloat, Label, Layout
};
use crate::{
    traits::{Drawable, UiElement},
    Vec2,
};

#[derive(Debug)]
pub enum Widget {
    Layout(Layout),
    Button(Button),
    Label(Label),
    PopupFloat(PopupFloat),
    SpacerPercent(f32),
}

impl Drawable for Widget {
    fn draw(&self, ref_size: Vec2, pos: Vec2) {
        match self {
            Widget::Layout(widget) => widget.draw(ref_size, pos),
            Widget::Button(button) => button.draw(ref_size, pos),
            Widget::Label(label) => label.draw(ref_size, pos),
            Widget::PopupFloat(popup) => popup.draw(ref_size, pos),
            Widget::SpacerPercent(_) => {}
        }
    }
}

impl UiElement for Widget {
    fn update(&mut self) {
        match self {
            Widget::Layout(widget) => widget.update(),
            Widget::Button(button) => button.update(),
            Widget::Label(label) => label.update(),
            Widget::PopupFloat(popup) => popup.update(),
            Widget::SpacerPercent(_) => {}
        }
    }

    fn abs_size(&self, ref_size: Vec2) -> Vec2 {
        match self {
            Widget::Layout(widget) => widget.abs_size(ref_size),
            Widget::Button(button) => button.abs_size(ref_size),
            Widget::Label(label) => label.abs_size(ref_size),
            Widget::PopupFloat(popup) => popup.abs_size(ref_size),
            Widget::SpacerPercent(val_percent_parent) => *val_percent_parent * ref_size,
        }
    }
}

impl Into<Layout> for Widget {
    fn into(self) -> Layout {
        match self {
            Widget::Layout(layout) => layout,
            widget => {
                warn!("Widget to layout conversion: Implicitly assuming a vertical layout with left alignment");
                Layout::new(
                    LayoutType::Vertical,
                    Vec2::new(1.0, 1.0),
                    [Section(vec![widget]), Section(vec![]), Section(vec![])],
                    vec![],
                )
            }
        }
    }
}
