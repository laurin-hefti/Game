use crate::traits::{Drawable, UiElement};
use std::fmt::Display;

use log::warn;
use macroquad::math::Vec2;

use super::Widget;


#[derive(Debug)]
pub enum LayoutType {
    Horizontal,
    Vertical,
}

impl Display for LayoutType {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        match self {
            LayoutType::Horizontal => write!(f, "Horizontal"),
            LayoutType::Vertical => write!(f, "Vertical"),
        }
    }
}

#[derive(Debug)]
pub struct Section(pub Vec<Widget>);

impl Section {
    pub fn abs_size(&self, parent_size: Vec2) -> Vec2 {
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
            size_percent_parent,
            sections,
        }
    }

    pub fn children_size(&self, ref_size: Vec2) -> Vec2 {
        match self.layout {
            LayoutType::Horizontal => Vec2 {
                x: if !self.sections[2].0.is_empty() {
                    // If there is a right section => it spans all the available space in width
                    ref_size.x
                } else if !self.sections[1].0.is_empty() {
                    // If there is no right section, but a center section
                    // => it spans until the last element of the center section
                    (ref_size.x + self.sections[1].abs_size(ref_size).x) * 0.5
                } else {
                    // If there is no right section and no center section
                    // => it spans only as far as the elements in the left section
                    self.sections[0].abs_size(ref_size).x
                },

                // y is max of all heights
                y: self.sections[0]
                    .abs_size(ref_size)
                    .y
                    .max(self.sections[1].abs_size(ref_size).y)
                    .max(self.sections[2].abs_size(ref_size).y),
            },

            LayoutType::Vertical => Vec2 {
                // x is max of all widths
                x: self.sections[0]
                    .abs_size(ref_size)
                    .x
                    .max(self.sections[1].abs_size(ref_size).x)
                    .max(self.sections[2].abs_size(ref_size).x),

                y: if !self.sections[2].0.is_empty() {
                    // Exists bottom_section? => height = height of available space
                    ref_size.y
                } else if !self.sections[1].0.is_empty() {
                    // Exists center_section? => height = max y of lowest element in center section
                    (ref_size.y + self.sections[1].abs_size(ref_size).y) * 0.5
                } else {
                    // No center_section? => height = height of left_section
                    self.sections[0].abs_size(ref_size).y
                },
            },
        }
    }

    #[cfg(debug_assertions)]
    pub fn check_for_size_overflow(&self, ref_size: Vec2) {
        // Check that actual size of children isn't larger than available space
        {
            let available_size = self.size_percent_parent * ref_size;
            let children_size = self.children_size(ref_size);
            match self.layout {
                LayoutType::Horizontal => {
                    if children_size.x > available_size.x {
                        warn!(
                            "Layout content width is too large (x: {} > {})",
                            children_size.x, available_size.x
                        );
                    }
                }
                LayoutType::Vertical => {
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

impl Drawable for Layout {
    fn draw(&self, available_space: Vec2, pos: Vec2) {
        let usable_space = self.abs_size(available_space);

        let mask = match self.layout {
            LayoutType::Horizontal => Vec2::X,
            LayoutType::Vertical => Vec2::Y,
        };

        // First section
        let mut cursor_pos = pos.clone();
        for child in self.sections[0].0.iter() {
            child.draw(usable_space, cursor_pos);
            cursor_pos += child.abs_size(usable_space) * mask;
        }

        // Center section
        //
        // Advance only in x or y
        let maybe_new_pos = pos + (usable_space - self.sections[1].abs_size(usable_space)) * 0.5;
        match self.layout {
            LayoutType::Horizontal => {
                cursor_pos.x = maybe_new_pos.x;
            }
            LayoutType::Vertical => {
                cursor_pos.y = maybe_new_pos.y;
            }
        }
        for child in self.sections[1].0.iter() {
            child.draw(usable_space, cursor_pos);
            cursor_pos += child.abs_size(usable_space) * mask;
        }

        // Last section
        //
        // Advance only in x or y
        let maybe_new_pos = pos + usable_space - self.sections[2].abs_size(usable_space);
        match self.layout {
            LayoutType::Horizontal => {
                cursor_pos.x = maybe_new_pos.x;
            }
            LayoutType::Vertical => {
                cursor_pos.y = maybe_new_pos.y;
            }
        }
        for child in self.sections[2].0.iter() {
            child.draw(usable_space, cursor_pos);
            cursor_pos += child.abs_size(usable_space) * mask;
        }

        #[cfg(debug_assertions)]
        {
            assert_ne!(usable_space.x, 0.0);
            assert_ne!(usable_space.y, 0.0);
            if log::log_enabled!(log::Level::Error) {
                macroquad::prelude::draw_rectangle_lines(
                    pos.x,
                    pos.y,
                    usable_space.x,
                    usable_space.y,
                    2.0,
                    macroquad::prelude::RED,
                );
            }
            if log::log_enabled!(log::Level::Debug) {
                macroquad::prelude::draw_line(
                    pos.x,
                    pos.y,
                    pos.x + usable_space.x,
                    pos.y + usable_space.y,
                    1.0,
                    macroquad::prelude::ORANGE,
                );
                macroquad::prelude::draw_text(
                    self.layout.to_string().as_str(),
                    pos.x + usable_space.x * 0.5,
                    pos.y + usable_space.y * 0.5,
                    20.0,
                    macroquad::prelude::RED,
                );
            }
        }
    }
}

impl UiElement for Layout {
    fn update(&mut self) {
        for section in self.sections.iter_mut() {
            section.0.iter_mut().for_each(|child| {
                child.update();
            });
        }
    }

    fn abs_size(&self, available_space: Vec2) -> Vec2 {
        let size = self.size_percent_parent * available_space;

        match size.as_ivec2().into() {
            (0, 0) => self.children_size(available_space),
            (0, _) => Vec2::new(self.children_size(available_space).x, size.y),
            (_, 0) => Vec2::new(size.x, self.children_size(available_space).y),
            (_, _) => size,
        }
    }
}
