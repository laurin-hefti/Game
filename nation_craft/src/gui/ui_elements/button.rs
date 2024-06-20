use crate::{gui::UiElement, Vec2};
use macroquad::ui;

pub struct Button {
    size_percent_parent: Vec2,
    text: String,
    callback: fn(),
}

impl Button {
    pub fn new<S>(pos: Vec2, text: S, callback: fn()) -> Self
    where
        S: Into<String>,
    {
        Self {
            size_percent_parent: pos,
            text: text.into(),
            callback,
        }
    }

    pub fn abs_size(&self, screen_size: &Vec2) -> Vec2 {
        self.size_percent_parent * *screen_size
    }
}

impl UiElement for Button {
    fn draw(&self, screen_size: &Vec2, pos: &Vec2) {
        if ui::widgets::Button::new(self.text.clone())
            .position(pos.clone())
            .size(self.abs_size(screen_size))
            .ui(&mut ui::root_ui())
        {
            (self.callback)();
        }
    }

    fn update(&mut self) {
        /* Handled in draw */
    }
}
