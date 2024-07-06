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

    #[cfg(debug_assertions)]
    pub fn check_for_size_overflow(&self, ref_size: &Vec2) {
        // Check that actual size of children isn't larger than available space
        {
            let available_size = self.size_percent_parent * *ref_size;
            let children_size = self.sections[0].abs_size(ref_size)
                + self.sections[1].abs_size(ref_size)
                + self.sections[2].abs_size(ref_size);
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


        // In which direction to advance the cursor (free probably not working)
        let mask = match self.layout {
            LayoutType::Free => Vec2::ONE,
            LayoutType::Horizontal => Vec2::X,
            LayoutType::Vertical => Vec2::Y,
        };

        let mut cursor_pos = pos.clone();
        for child in self.sections[0].0.iter() {
            // Draw
            child.draw(&ref_size, &cursor_pos);
            cursor_pos += child.abs_size(&ref_size) * mask;
        }

        cursor_pos = (ref_size - self.sections[1].abs_size(&ref_size)) * 0.5 * mask;
        for child in self.sections[1].0.iter() {
            // Draw
            child.draw(&ref_size, &cursor_pos);
            cursor_pos += child.abs_size(&ref_size) * mask;
        }

        cursor_pos = (ref_size - self.sections[2].abs_size(&ref_size)) * mask;
        for child in self.sections[2].0.iter() {
            // Draw
            child.draw(&ref_size, &cursor_pos);
            cursor_pos += child.abs_size(&ref_size) * mask;
        }
    }

    fn update(&mut self) {
        for section in self.sections.iter_mut() {
            section.0.iter_mut().for_each(|child| {
                child.update();
            });
        }
    }

    fn abs_size(&self, parent_size: &Vec2) -> Vec2 {
        self.size_percent_parent * *parent_size
    }
}
