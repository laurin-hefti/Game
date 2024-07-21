
pub struct ColorCollection {
    pub window_border_color: macroquad::color::Color,
}

impl ColorCollection {
    pub const fn default() -> Self {
        Self {
            window_border_color: macroquad::color::WHITE,
        }
    }
}
