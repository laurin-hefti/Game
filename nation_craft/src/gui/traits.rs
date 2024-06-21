use crate::Vec2;

pub trait UiElement {
    fn draw(&self, ref_size: &Vec2, pos: &Vec2);
    fn update(&mut self);
    fn abs_size(&self, ref_size: &Vec2) -> Vec2;
}


