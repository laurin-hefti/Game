use crate::{traits::{Drawable, UiElement as _}, Vec2};

use super::widgets::Layout;


#[derive(Debug)]
pub struct MainUi {
    pub root_layout: Layout,
}

impl MainUi {
    pub fn new(root_layout: impl Into<Layout>) -> Self {
        Self {
            root_layout: root_layout.into(),
        }
    }

    pub fn check_for_size_overflow(&self, ref_size: Vec2) {
        self.root_layout.check_for_size_overflow(ref_size);
    }

    pub fn update(&mut self) {
        self.root_layout.update();
    }

    pub fn draw(&self, screen_size: Vec2) {
        self.root_layout.draw(screen_size, Vec2::new(0., 0.));
    }

}
