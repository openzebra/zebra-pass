//! -- Copyright (c) 2023 Rina Khasanshin
//! -- Email: hicarus@yandex.ru
//! -- Licensed under the GNU General Public License Version 3.0 (GPL-3.0)
use iced::Color;

#[derive(Debug, Clone, Copy, PartialEq)]
pub struct ZebraPalette {
    pub light: Color,
    pub dark: Color,
    pub primary: Color,
    pub secondary: Color,
    pub success: Color,
    pub warning: Color,
    pub warn: Color,
    pub danger: Color,
    pub info: Color,
    pub window_background: Color,
    pub window_background_inverse: Color,
    pub radius: f32,
    pub font_size_small: u8,
    pub font_size_regular: u8,
    pub font_size_medium: u8,
    pub font_size_big: u8,
}

impl Default for ZebraPalette {
    fn default() -> Self {
        ZebraPalette::LIGHT
    }
}

impl ZebraPalette {
    pub const LIGHT: Self = Self {
        light: Color::WHITE,
        dark: Color::BLACK,
        primary: Color::from_rgb(
            0x5E as f32 / 255.0,
            0x7C as f32 / 255.0,
            0xE2 as f32 / 255.0,
        ),
        secondary: Color::from_rgb(
            0x5E as f32 / 255.0,
            0x7C as f32 / 255.0,
            0xE2 as f32 / 255.0,
        ),
        success: Color::from_rgb(
            0x5E as f32 / 255.0,
            0x7C as f32 / 255.0,
            0xE2 as f32 / 255.0,
        ),
        warning: Color::from_rgb(
            0x5E as f32 / 255.0,
            0x7C as f32 / 255.0,
            0xE2 as f32 / 255.0,
        ),
        warn: Color::from_rgb(
            0x5E as f32 / 255.0,
            0x7C as f32 / 255.0,
            0xE2 as f32 / 255.0,
        ),
        danger: Color::from_rgb(
            0x5E as f32 / 255.0,
            0x7C as f32 / 255.0,
            0xE2 as f32 / 255.0,
        ),
        info: Color::from_rgb(
            0x5E as f32 / 255.0,
            0x7C as f32 / 255.0,
            0xE2 as f32 / 255.0,
        ),
        window_background: Color::WHITE,
        window_background_inverse: Color::BLACK,
        radius: 4.0,
        font_size_small: 12,
        font_size_regular: 14,
        font_size_medium: 16,
        font_size_big: 18,
    };

    pub const DARK: Self = Self {
        light: Color::BLACK,
        dark: Color::WHITE,
        primary: Color {
            r: 164.0 / 255.0,
            g: 187.0 / 255.0,
            b: 213.0 / 255.0,
            a: 1.0,
        },
        secondary: Color {
            r: 54.0 / 255.0,
            g: 73.0 / 255.0,
            b: 143.0 / 255.0,
            a: 1.0,
        },
        success: Color {
            r: 160.0 / 255.0,
            g: 214.0 / 255.0,
            b: 198.0 / 255.0,
            a: 1.0,
        },
        warning: Color {
            r: 1.0,
            g: 191.0 / 255.0,
            b: 156.0 / 255.0,
            a: 1.0,
        },
        warn: Color {
            r: 247.0 / 255.0,
            g: 87.0 / 255.0,
            b: 0.0 / 255.0,
            a: 1.0,
        },
        danger: Color {
            r: 1.0,
            g: 0.0 / 255.0,
            b: 0.0 / 255.0,
            a: 1.0,
        },
        info: Color {
            r: 166.0 / 255.0,
            g: 166.0 / 255.0,
            b: 157.0 / 255.0,
            a: 1.0,
        },
        window_background: Color::BLACK,
        window_background_inverse: Color::WHITE,
        radius: 8.0,
        font_size_small: 12,
        font_size_regular: 14,
        font_size_medium: 16,
        font_size_big: 18,
    };
}
