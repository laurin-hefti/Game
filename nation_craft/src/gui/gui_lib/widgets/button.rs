use crate::{
    Vec2,
    traits::{Drawable, UiElement},
};
use macroquad::ui;

#[derive(Debug)]
pub struct Button {
    size: Vec2,
    use_abs_size: bool,
    text: String,
    callback: fn(),
}

impl Button {
    pub fn new<S: Into<String>>(size: Vec2, text: S, callback: fn()) -> Self {
        Self {
            size,
            use_abs_size: false,
            text: text.into(),
            callback,
        }
    }

    pub fn use_abs_size(mut self) -> Self {
        self.use_abs_size = true;
        self
    }
}

impl Drawable for Button {
    fn draw(&self, available_space: Vec2, pos: Vec2) {
        if ui::widgets::Button::new(self.text.clone())
            .position(pos.clone())
            .size(self.abs_size(available_space))
            .ui(&mut ui::root_ui())
        {
            (self.callback)();
        }
    }
}

impl UiElement for Button {
    fn update(&mut self) {
        /* Handled in draw */
    }

    fn abs_size(&self, available_space: Vec2) -> Vec2 {
        if self.use_abs_size {
            // Size is already absolute
            self.size
        } else {
            // Size is relative
            self.size * available_space
        }
    }
}
