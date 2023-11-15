//! -- Copyright (c) 2023 Rina Khasanshin
//! -- Email: hicarus@yandex.ru
//! -- Licensed under the GNU General Public License Version 3.0 (GPL-3.0)
use iced::Font;

pub const BOLD: Font = Font::External {
    name: "Bold",
    bytes: include_bytes!("../../public/fonts/OpenSans-Bold.ttf"),
};

pub const REGULAR: Font = Font::External {
    name: "Regular",
    bytes: include_bytes!("../../public/fonts/OpenSans-Regular.ttf"),
};

pub const SEMI_BOLD: Font = Font::External {
    name: "Semi",
    bytes: include_bytes!("../../public/fonts/OpenSans-SemiBoldItalic.ttf"),
};
