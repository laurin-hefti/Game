use macroquad::{math::Vec2, shapes::draw_rectangle_lines};

use crate::{gui::gui_lib::widgets::Layout, settings::GLOBAL_SETTINGS, traits::{Drawable, UiElement}};

#[derive(Debug)]
pub struct PopupFloat {
    layout: Layout,
    pos: Vec2,
    size: Vec2,
}

impl PopupFloat {
    pub fn new<L: Into<Layout>>(layout: L, pos: Vec2, size: Vec2) -> Self {
        Self { layout: layout.into(), pos, size }
    }
}

impl Drawable for PopupFloat {
    fn draw(&self, _ref_size: Vec2, _pos: Vec2) {
        self.layout.draw(self.size, self.pos);
        draw_rectangle_lines(
            self.pos.x,
            self.pos.y,
            self.size.x,
            self.size.y,
            2.0,
            GLOBAL_SETTINGS.lock().unwrap().gui.colors.window_border_color,
        );
    }
}

impl UiElement for PopupFloat {
    fn update(&mut self) {}
    fn abs_size(&self, _ref_size: Vec2) -> Vec2 {
        self.size
    }
}
