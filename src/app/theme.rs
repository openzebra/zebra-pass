//! -- Copyright (c) 2023 Rina Khasanshin
//! -- Email: hicarus@yandex.ru
//! -- Licensed under the GNU General Public License Version 3.0 (GPL-3.0)

use iced::Color;

#[derive(Debug, Clone, PartialEq)]
pub struct ZebraColors {
    light: Color,
    dark: Color,
    primary: Color,
    secondary: Color,
    success: Color,
    warning: Color,
    warn: Color,
    danger: Color,
    info: Color,
    window_background: Color,
    window_background_inverse: Color,
    radius: u8,
    font_size_small: u8,
    font_size_regular: u8,
    font_size_medium: u8,
    font_size_big: u8,
}

impl ZebraColors {
    pub fn dark() -> Self {
        Self {
            light: Color::from_rgb8(255, 255, 255),
            dark: Color::from_rgb8(34, 34, 34),
            primary: Color::from_rgb8(0, 123, 255),
            secondary: Color::from_rgb8(108, 117, 125),
            success: Color::from_rgb8(40, 167, 69),
            warning: Color::from_rgb8(255, 193, 7),
            warn: Color::from_rgb8(255, 193, 7),
            danger: Color::from_rgb8(220, 53, 69),
            info: Color::from_rgb8(23, 162, 184),
            window_background: Color::from_rgb8(48, 48, 48),
            window_background_inverse: Color::from_rgb8(255, 255, 255),
            radius: 4,
            font_size_small: 12,
            font_size_regular: 14,
            font_size_medium: 16,
            font_size_big: 18,
        }
    }
    pub fn light() -> Self {
        Self {
            light: Color::from_rgb8(255, 255, 255),
            dark: Color::from_rgb8(34, 34, 34),
            primary: Color::from_rgb8(0, 123, 255),
            secondary: Color::from_rgb8(108, 117, 125),
            success: Color::from_rgb8(40, 167, 69),
            warning: Color::from_rgb8(255, 193, 7),
            warn: Color::from_rgb8(255, 193, 7),
            danger: Color::from_rgb8(220, 53, 69),
            info: Color::from_rgb8(23, 162, 184),
            window_background: Color::from_rgb8(255, 255, 255),
            window_background_inverse: Color::from_rgb8(34, 34, 34),
            radius: 4,
            font_size_small: 12,
            font_size_regular: 14,
            font_size_medium: 16,
            font_size_big: 18,
        }
    }
}

#[derive(Debug, Clone, PartialEq)]
pub enum Theme {
    Light(ZebraColors),
    Dark(ZebraColors),
}

impl Default for Theme {
    fn default() -> Self {
        match dark_light::detect() {
            dark_light::Mode::Dark => Theme::Dark(ZebraColors::dark()),
            dark_light::Mode::Light => Theme::Light(ZebraColors::light()),
            dark_light::Mode::Default => Theme::Dark(ZebraColors::dark()),
        }
    }
}
