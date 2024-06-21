use crate::{gui::UiElement, Vec2};
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

    fn abs_size(&self, ref_size: &Vec2) -> Vec2 {
        if self.use_abs_size {
            // Size is already absolute
            self.size
        } else {
            // Size is relative
            self.size * *ref_size
        }
    }
}
