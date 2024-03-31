//! -- Copyright (c) 2024 Rina Khasanshin
//! -- Email: hicarus@yandex.ru
//! -- Licensed under the GNU General Public License Version 3.0 (GPL-3.0)
pub const BORDER_RADIUS: f32 = 8.0;

pub mod dark {
    use iced::{color, Color};

    pub const BACKGROUND: Color = color!(0.0, 0.0, 0.0);
    pub const PRIMARY: Color = color!(0xa4bbd5);
    pub const SECONDARY: Color = color!(0x36498f);
    pub const SUCCESS: Color = color!(0xA0D6C6);
    pub const DANGER: Color = color!(0xff0000);
    pub const WARNING: Color = color!(0xFFBF9C);
    pub const MUTED: Color = color!(0x6e6e6e);
    pub const CARD: Color = color!(0x242424);
    pub const TEXT: Color = color!(0xf5f5f7);
}

pub mod light {
    use iced::{color, Color};

    pub const BACKGROUND: Color = color!(0xDBE1F9);
    pub const PRIMARY: Color = color!(0x29ccc4);
    pub const SECONDARY: Color = color!(0x13324f);
    pub const SUCCESS: Color = color!(0xa2d72a);
    pub const DANGER: Color = color!(0xfb6165);
    pub const WARNING: Color = color!(0xffc224);
    pub const MUTED: Color = color!(0x8E8EAE);
    pub const CARD: Color = color!(0xE7F1FF);
    pub const TEXT: Color = color!(0x394e63);
}
