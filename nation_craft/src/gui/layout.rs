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
    pub fn new(layout: LayoutType, children: Vec<Widget>, size_percent_parent: Vec2) -> Self {
        Self {
            layout,
            children,
            size_percent_parent,
        }
    }

    #[cfg(debug_assertions)]
    pub fn check_for_size_overflow(&self, ref_size: &Vec2) {
        // Check that actual size of children isn't larger than available space
        {
            let available_size = self.size_percent_parent * *ref_size;
            let children_size = self
                .children
                .iter()
                .map(|child| child.abs_size(ref_size))
                .sum::<Vec2>();
            match self.layout {
                LayoutType::Free | LayoutType::Horizontal => {
                    if children_size.x > available_size.x {
                        warn!(
                            "Layout content width is too large (x: {} > {})",
                            children_size.x, available_size.x
                        );
                    }
                }
                LayoutType::Vertical => {
                    // TODO: also check for free
                    if children_size.y > available_size.y {
                        warn!(
                            "Layout content height is too large (y: {} > {})",
                            children_size.y, available_size.y
                        );
                    }
                }
            }
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

    fn abs_size(&self, parent_size: &Vec2) -> Vec2 {
        self.size_percent_parent * *parent_size
    }
}
