use macroquad::{color::Color, color_u8, math::RectOffset};

use crate::Vec2;

pub const BG_COLOR: Color = color_u8!(20, 20, 20, 255);

pub const BUTTON_SIZE: Vec2 = Vec2::new(100., 50.0);
pub const TITLE_BAR_HEIGHT: f32 = 0.03;
pub const BOTTOM_BAR_HEIGHT: f32 = 0.03;
pub const WINDOW_MARGIN: RectOffset = RectOffset {
    left: 10.0,
    right: 10.0,
    bottom: 10.0,
    top: 10.0,
};

pub const FONT_SIZE: u16 = 16;
