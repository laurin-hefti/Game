use crate::Vec2;

pub trait UiElement {
    fn draw(&self, screen_size: &Vec2, pos: &Vec2);
    fn update(&mut self);
}
