use crate::Vec2;

pub trait Drawable {
    fn draw(&self, available_space: Vec2, pos: Vec2);
}

pub trait UiElement {
    fn update(&mut self);
    fn abs_size(&self, ref_size: Vec2) -> Vec2;
}


