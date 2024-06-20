use log::warn;
use macroquad::math::Vec2;

use super::{UiElement, Widget};

#[derive(Debug)]
pub enum LayoutType {
    Horizontal,
    Vertical,
    Free,
}

#[derive(Debug)]
pub struct Layout {
    pub layout: LayoutType,
    pub children: Vec<Widget>,
    size_percent_parent: Vec2,
}

impl Layout {
    pub fn new(layout: LayoutType, children: Vec<Widget>, size: Option<Vec2>) -> Self {
        let layout = Self {
            layout,
            children,
            size_percent_parent: size.unwrap_or(Vec2::ONE),
        };

        #[cfg(debug_assertions)]
        layout.abs_size(&layout.size_percent_parent);
        layout
    }

    pub fn abs_size(&self, parent_size: &Vec2) -> Vec2 {
        let ref_size = self.size_percent_parent * *parent_size;
        match self.layout {
            LayoutType::Horizontal => {
                let children_width = self.children.iter().map(|w| w.abs_size(&ref_size).x).sum();
                #[cfg(debug_assertions)]
                if children_width > ref_size.x {
                    warn!("NestableWidget: Children are bigger than parent (in width)")
                }
                Vec2::new(children_width, ref_size.y)
            }
            LayoutType::Vertical => {
                let children_height = self.children.iter().map(|w| w.abs_size(&ref_size).y).sum();
                #[cfg(debug_assertions)]
                if children_height > ref_size.y {
                    warn!("NestableWidget: Children are bigger than parent (in height)")
                }
                Vec2::new(ref_size.x, children_height)
            }
            LayoutType::Free => todo!(),
        }
    }
}

impl UiElement for Layout {
    fn draw(&self, ref_size: &Vec2, pos: &Vec2) {
        let ref_size = self.size_percent_parent * *ref_size;

        let mut cursor_pos = pos.clone();
        for child in &self.children {
            // Draw
            child.draw(&ref_size, &cursor_pos);

            // Advance the cursor by width of the drawn element so that the next element isn't
            // drawn on top of it
            let child_size = child.abs_size(&ref_size);
            match self.layout {
                LayoutType::Horizontal => {
                    cursor_pos.x += child_size.x;
                }
                LayoutType::Vertical => {
                    cursor_pos.y += child_size.y;
                }
                LayoutType::Free => todo!(),
            }
        }
    }

    fn update(&mut self) {
        for child in &mut self.children {
            child.update();
        }
    }
}
