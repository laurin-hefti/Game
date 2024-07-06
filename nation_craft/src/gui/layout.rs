use core::panic;

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
pub struct Section(pub Vec<Widget>);

impl Section {
    pub fn abs_size(&self, parent_size: &Vec2) -> Vec2 {
        self.0.iter().map(|child| child.abs_size(parent_size)).sum()
    }
}

#[derive(Debug)]
pub struct Layout {
    pub layout: LayoutType,
    size_percent_parent: Vec2,
    sections: [Section; 3],
}

impl Layout {
    pub fn new(layout: LayoutType, size_percent_parent: Vec2, sections: [Section; 3]) -> Self {
        Self {
            layout,
            sections,
            size_percent_parent,
        }
    }

    pub fn children_size(&self, ref_size: &Vec2) -> Vec2 {
        self.sections[0].abs_size(ref_size)
            + self.sections[1].abs_size(ref_size)
            + self.sections[2].abs_size(ref_size)
    }

    #[cfg(debug_assertions)]
    pub fn check_for_size_overflow(&self, ref_size: &Vec2) {
        // Check that actual size of children isn't larger than available space
        {
            let available_size = self.size_percent_parent * *ref_size;
            let children_size = self.children_size(ref_size);
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
        let actual_size = self.abs_size(ref_size);

        let mask = match self.layout {
            LayoutType::Free => Vec2::ONE,
            LayoutType::Horizontal => Vec2::X,
            LayoutType::Vertical => Vec2::Y,
        };

        // First section
        let mut cursor_pos = pos.clone();
        for child in self.sections[0].0.iter() {
            child.draw(&actual_size, &cursor_pos);
            cursor_pos += child.abs_size(&actual_size) * mask;
        }

        // Center section
        //
        // Advance only in x or y
        let maybe_new_pos = (*ref_size - self.sections[1].abs_size(&actual_size)) * 0.5 * mask;
        match self.layout {
            LayoutType::Free | LayoutType::Horizontal => {
                cursor_pos.x = maybe_new_pos.x;
            }
            LayoutType::Vertical => {
                cursor_pos.y = maybe_new_pos.y;
            }
        }
        for child in self.sections[1].0.iter() {
            child.draw(&actual_size, &cursor_pos);
            cursor_pos += child.abs_size(&actual_size) * mask;
        }

        // Last section
        //
        // Advance only in x or y
        let maybe_new_pos = (*ref_size - self.sections[2].abs_size(&actual_size)) * mask;
        match self.layout {
            LayoutType::Free | LayoutType::Horizontal => {
                cursor_pos.x = maybe_new_pos.x;
            }
            LayoutType::Vertical => {
                cursor_pos.y = maybe_new_pos.y;
            }
        }
        for child in self.sections[2].0.iter() {
            child.draw(&actual_size, &cursor_pos);
            cursor_pos += child.abs_size(&actual_size) * mask;
        }

        #[cfg(debug_assertions)]
        {
            let size = self.abs_size(ref_size);
            macroquad::prelude::draw_rectangle_lines(
                pos.x,
                pos.y,
                size.x,
                size.y,
                1.0,
                macroquad::prelude::BLACK,
            );
        }
    }

    fn update(&mut self) {
        for section in self.sections.iter_mut() {
            section.0.iter_mut().for_each(|child| {
                child.update();
            });
        }
    }

    fn abs_size(&self, ref_size: &Vec2) -> Vec2 {
        let size = self.size_percent_parent * *ref_size;

        match size.as_ivec2().into() {
            (0, 0) => self.children_size(ref_size),
            (0, _) => Vec2::new(size.x, self.children_size(ref_size).y),
            (_, 0) => Vec2::new(self.children_size(ref_size).x, size.y),
            _ => size,
        }
    }
}
