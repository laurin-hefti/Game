use crate::{gui::UiElement, Vec2};
use macroquad::ui;

#[derive(Debug)]
pub struct Button {
    size_percent_parent: Vec2,
    text: String,
    callback: fn(),
}

impl Button {
    pub fn new<S: Into<String>>(pos: Vec2, text: S, callback: fn()) -> Self {
        Self {
            size_percent_parent: pos,
            text: text.into(),
            callback,
        }
    }

    pub fn abs_size(&self, ref_size: &Vec2) -> Vec2 {
        self.size_percent_parent * *ref_size
    }
}

impl UiElement for Button {
    fn draw(&self, ref_size: &Vec2, pos: &Vec2) {
        if ui::widgets::Button::new(self.text.clone())
            .position(pos.clone())
            .size(self.abs_size(ref_size))
            .ui(&mut ui::root_ui())
        {
            (self.callback)();
        }
    }

    fn update(&mut self) {
        /* Handled in draw */
    }
}
