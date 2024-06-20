use crate::{gui::UiElement, Vec2};
use macroquad::ui::{self, root_ui};

#[derive(Debug)]
pub struct Label {
    size_percent_parent: Vec2,
    text: String,
}

impl Label {
    pub fn new<S: Into<String>>(pos: Vec2, text: S) -> Self {
        Self {
            size_percent_parent: pos,
            text: text.into(),
        }
    }

    pub fn abs_size(&self, ref_size: &Vec2) -> Vec2 {
        self.size_percent_parent * *ref_size
    }
}

impl UiElement for Label {
    fn draw(&self, ref_size: &Vec2, pos: &Vec2) {
        ui::widgets::Label::new(self.text.clone())
            .position(pos.clone())
            .size(self.abs_size(ref_size))
            .ui(&mut root_ui());
    }

    fn update(&mut self) {}
}
