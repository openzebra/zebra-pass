//! -- Copyright (c) 2023 Rina Khasanshin
//! -- Email: hicarus@yandex.ru
//! -- Licensed under the GNU General Public License Version 3.0 (GPL-3.0)

use iced::Font;

pub const BOLD: Font = Font::External {
    name: "Bold",
    bytes: include_bytes!("../static/fonts/OpenSans-Bold.ttf"),
};

pub const MEDIUM: Font = Font::External {
    name: "SemiBold",
    bytes: include_bytes!("../static/fonts/OpenSans-SemiBold.ttf"),
};

pub const REGULAR_BYTES: &[u8] = include_bytes!("../static/fonts/OpenSans-Regular.ttf");

pub const REGULAR: Font = Font::External {
    name: "Regular",
    bytes: REGULAR_BYTES,
};
