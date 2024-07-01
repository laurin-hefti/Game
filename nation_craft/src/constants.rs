use macroquad::{color::{Color, BLACK, WHITE}, color_u8, math::RectOffset};

use crate::Vec2;

pub const BG_COLOR_PRIMARY: Color = color_u8!(30, 30, 30, 255);
pub const BG_COLOR_DARKER: Color = color_u8!(20, 20, 20, 255);
pub const BG_COLOR_DARKEST: Color = BLACK;
pub const FG_COLOR_PRIMARY: Color = WHITE;
pub const FG_COLOR_DARKER: Color = color_u8!(200, 200, 200, 255);

pub const BUTTON_SIZE: Vec2 = Vec2::new(100.0, 40.0);
pub const TITLE_BAR_HEIGHT: f32 = 0.03;
pub const BOTTOM_BAR_HEIGHT: f32 = 0.03;

pub const POPUP_MARGIN: RectOffset = RectOffset {
    left: 10.0,
    right: 10.0,
    bottom: 10.0,
    top: 10.0,
};

pub const FONT_SIZE: u16 = 16;
