use macroquad::math::Vec2;

use super::{UiElement, Widget};


pub enum LayoutType {
    Horizontal,
    Vertical,
    Free(f32),
}

pub struct Layout {
    pub layout: LayoutType,
    pub children: Vec<Widget>,
    size: Vec2,
}

impl Layout {
    pub fn new(layout: LayoutType, children: Vec<Widget>, size: Option<Vec2>) -> Self {
        Self {
            layout,
            children,
            size: size.unwrap_or(Vec2::ONE),
        }
    }

    pub fn abs_size(&self, ref_size: &Vec2) -> Vec2 {
        let ref_size = self.size * *ref_size;
        let mut cursor_pos = Vec2::new(0., 0.);
        for child in &self.children {
            match self.layout {
                LayoutType::Horizontal => {
                    cursor_pos.y += child.abs_size(&ref_size).x;
                }
                LayoutType::Vertical => {
                    cursor_pos.x += child.abs_size(&ref_size).y;
                }
                LayoutType::Free(_) => todo!(),
            }
        }
        #[cfg(debug_assertions)]
        if cursor_pos.x < 0. || cursor_pos.y < 0. {
            panic!("NestableWidget has negative size");
        }
        #[cfg(debug_assertions)]
        if cursor_pos.x > ref_size.x || cursor_pos.y > ref_size.y {
            panic!("Single elements in NestableWidget are bigger than 1.0 (100 per cent) of their parent");
        }
        Vec2::new(cursor_pos.x, cursor_pos.y)
    }
}

impl UiElement for Layout {
    fn draw(&self, ref_size: &Vec2, pos: &Vec2) {
        let ref_size = self.size * *ref_size;

        let mut cursor_pos = pos.clone();
        for child in &self.children {
            // Draw
            child.draw(&ref_size, &cursor_pos);

            // Advance the cursor by width of the drawn element so that the next element isn't
            // drawn on top of it
            match self.layout {
                LayoutType::Horizontal => {
                    cursor_pos.x += child.abs_size(&ref_size).x;
                }
                LayoutType::Vertical => {
                    cursor_pos.y += child.abs_size(&ref_size).y;
                }
                LayoutType::Free(_) => todo!(),
            }
        }
    }

    fn update(&mut self) {
        for child in &mut self.children {
            child.update();
        }
    }
}
