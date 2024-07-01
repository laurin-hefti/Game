use macroquad::{
    math::RectOffset, texture::Image, ui::{self, root_ui, Skin}
};

use crate::GLOBAL_SETTINGS;
use crate::constants::*;

pub fn default_skin() -> ui::Skin {
    let window_size = GLOBAL_SETTINGS
        .lock()
        .expect("Couldn't aquire lock")
        .gui
        .screen_size;

    let window_style = root_ui()
        .style_builder()
        .background_margin(POPUP_MARGIN)
        .margin(POPUP_MARGIN)
        .text_color(FG_COLOR_PRIMARY)
        .background(Image::gen_image_color(
            window_size.x as u16,
            window_size.y as u16,
            BG_COLOR_PRIMARY
        ))
        .build();

    let label_style = root_ui()
        .style_builder()
        .font_size(FONT_SIZE)
        .text_color(FG_COLOR_PRIMARY)
        .background(Image::gen_image_color(
            BUTTON_SIZE.x as u16,
            BUTTON_SIZE.y as u16,
            BG_COLOR_PRIMARY,
        ))
        .build();

    let button_style = root_ui()
        .style_builder()
        .background_margin(RectOffset::new(8.0, 8.0, 8.0, 8.0))
        .text_color(FG_COLOR_PRIMARY)
        .text_color_hovered(FG_COLOR_DARKER)
        .text_color_clicked(FG_COLOR_DARKER)
        .background(Image::gen_image_color(
            BUTTON_SIZE.x as u16,
            BUTTON_SIZE.y as u16,
            BG_COLOR_DARKER,
        ))
        .background_hovered(Image::gen_image_color(
            BUTTON_SIZE.x as u16,
            BUTTON_SIZE.y as u16,
            BG_COLOR_DARKEST
        ))
        .background_clicked(Image::gen_image_color(
            BUTTON_SIZE.x as u16,
            BUTTON_SIZE.y as u16,
            BG_COLOR_DARKER
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
