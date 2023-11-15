//! -- Copyright (c) 2023 Rina Khasanshin
//! -- Email: hicarus@yandex.ru
//! -- Licensed under the GNU General Public License Version 3.0 (GPL-3.0)

use iced::{
    widget::{image::Handle, Image},
    ContentFit, Length,
};

const PRINT: &[u8] = include_bytes!("../../../public/imgs/zebra_print.png");

pub fn zebra_print_view() -> Image<Handle> {
    let h = Handle::from_memory(PRINT);

    Image::<Handle>::new(h)
        .height(Length::Fill)
        .content_fit(ContentFit::Cover)
        .width(Length::Fill)
}
