//! -- Copyright (c) 2024 Rina Khasanshin
//! -- Email: hicarus@yandex.ru
//! -- Licensed under the GNU General Public License Version 3.0 (GPL-3.0)

use iced::theme::Custom;

pub fn dark_custom_theme() -> Custom {
    let dark_palette = iced::theme::Palette {
        background: super::config::dark::BACKGROUND,
        text: super::config::dark::TEXT,
        primary: super::config::dark::PRIMARY,
        success: super::config::dark::SUCCESS,
        danger: super::config::dark::DANGER,
    };

    Custom::new("Dark".to_string(), dark_palette)
}

pub fn light_custom_theme() -> Custom {
    let dark_palette = iced::theme::Palette {
        background: super::config::light::BACKGROUND,
        text: super::config::light::TEXT,
        primary: super::config::light::PRIMARY,
        success: super::config::light::SUCCESS,
        danger: super::config::light::DANGER,
    };

    Custom::new("Light".to_string(), dark_palette)
}
