use macroquad::{
    color::{Color, BLACK, GRAY, WHITE}, color_u8, math::RectOffset, texture::Image, ui::{self, root_ui, Skin}
};

use crate::{constants::{BG_COLOR, BUTTON_SIZE, FONT_SIZE, WINDOW_MARGIN}, GLOBAL_SETTINGS};

pub fn default_skin() -> ui::Skin {
    let window_size = GLOBAL_SETTINGS
        .lock()
        .expect("Couldn't aquire lock")
        .gui
        .screen_size;

    let window_style = root_ui()
        .style_builder()
        .background_margin(RectOffset::new(20.0, 20.0, 10.0, 10.0))
        .margin(WINDOW_MARGIN)
        .text_color(WHITE)
        .background(Image::gen_image_color(
            window_size.x as u16,
            window_size.y as u16,
            color_u8!(10, 10, 10, 100),
        ))
        .build();

    let label_style = root_ui()
        .style_builder()
        .font_size(FONT_SIZE)
        .text_color(WHITE)
        .background(Image::gen_image_color(
            BUTTON_SIZE.x as u16,
            BUTTON_SIZE.y as u16,
            BG_COLOR,
        ))
        .build();

    let button_style = root_ui()
        .style_builder()
        .background_margin(RectOffset::new(8.0, 8.0, 8.0, 8.0))
        .text_color(WHITE)
        .text_color_hovered(WHITE)
        .text_color_clicked(WHITE)
        .background(Image::gen_image_color(
            BUTTON_SIZE.x as u16,
            BUTTON_SIZE.y as u16,
            BLACK,
        ))
        .background_hovered(Image::gen_image_color(
            BUTTON_SIZE.x as u16,
            BUTTON_SIZE.y as u16,
            color_u8!(30, 30, 30, 100),
        ))
        .background_clicked(Image::gen_image_color(
            BUTTON_SIZE.x as u16,
            BUTTON_SIZE.y as u16,
            GRAY,
        ))
        .font_size(FONT_SIZE)
        .build();

    Skin {
        label_style,
        button_style,
        window_style,
        ..root_ui().default_skin()
    }
}
