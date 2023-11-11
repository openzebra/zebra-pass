use iced::{window, Sandbox, Settings};
// -- Copyright (c) 2023 Rina Khasanshin
// -- Email: hicarus@yandex.ru
// -- Licensed under the GNU General Public License Version 3.0 (GPL-3.0)
use zebra_pass::app::app::App;

pub fn main() -> iced::Result {
    App::run(Settings {
        window: window::Settings {
            size: (750, 450),
            resizable: false,
            ..Default::default()
        },
        ..Default::default()
    })
}

#[cfg(test)]
mod main_tests {}
