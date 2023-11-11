use std::io::{Error, ErrorKind};

use iced::{window, Application, Settings};
// -- Copyright (c) 2023 Rina Khasanshin
// -- Email: hicarus@yandex.ru
// -- Licensed under the GNU General Public License Version 3.0 (GPL-3.0)
use zebra_pass::{app::app::App, core::core::Core};

pub fn main() -> iced::Result {
    let core = match Core::new() {
        Ok(core) => core,
        Err(e) => {
            let error = Error::new(ErrorKind::Other, e.to_string());
            return iced::Result::Err(iced::Error::ExecutorCreationFailed(error));
        }
    };

    match core.sync() {
        Ok(_) => {}
        Err(e) => {
            let error = Error::new(ErrorKind::Other, e.to_string());
            return iced::Result::Err(iced::Error::ExecutorCreationFailed(error));
        }
    };

    App::run(Settings {
        window: window::Settings {
            size: (750, 450),
            resizable: false,
            ..Default::default()
        },
        flags: core,
        id: Default::default(),
        default_font: Default::default(),
        default_text_size: Default::default(),
        antialiasing: Default::default(),
        exit_on_close_request: Default::default(),
    })
}

#[cfg(test)]
mod main_tests {}
