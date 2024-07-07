use macroquad::math::Vec2;

use crate::{gui::gui_lib::widgets::Layout, traits::Drawable};

pub struct FloatingWindow {
    layout: Layout,
}

impl FloatingWindow {
    pub fn new(layout: Layout) -> Self {
        Self { layout }
    }
}

impl Drawable for FloatingWindow {
    fn draw(&self, ref_size: Vec2, pos: Vec2) {
        self.layout.draw(ref_size, pos);
    }
}
