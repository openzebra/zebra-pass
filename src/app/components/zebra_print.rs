//! -- Copyright (c) 2023 Rina Khasanshin
//! -- Email: hicarus@yandex.ru
//! -- Licensed under the GNU General Public License Version 3.0 (GPL-3.0)

use iced::{
    widget::{image::Handle, Image},
    ContentFit, Length,
};

pub fn zebra_print_view() -> Image<Handle> {
    Image::new("src/app/images/zebra_print.png")
        .height(Length::Fill)
        .content_fit(ContentFit::Cover)
        .width(Length::Fill)
}
