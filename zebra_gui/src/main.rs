//! -- Copyright (c) 2023 Rina Khasanshin
//! -- Email: hicarus@yandex.ru
//! -- Licensed under the GNU General Public License Version 3.0 (GPL-3.0)
extern crate rust_i18n;

use error::GUIError;
use gui::GUI;
use iced::{window, Application, Settings};
use rust_i18n::i18n;
use zebra_lib::core::core::Core;

mod app;
mod error;
mod gui;

i18n!("locales", fallback = "en");

fn main() -> iced::Result {
    let window = window::Settings {
        size: (750, 450),
        resizable: false,
        icon: Some(zebra_ui::image::liana_app_icon()),
        ..Default::default()
    };
    let core = match Core::new() {
        Ok(core) => core,
        Err(e) => {
            return GUIError::run(Settings {
                window,
                flags: e.to_string(),
                ..Default::default()
            });
        }
    };

    match core.sync() {
        Ok(_) => {}
        Err(e) => {
            return GUIError::run(Settings {
                window,
                flags: e.to_string(),
                ..Default::default()
            });
        }
    };

    rust_i18n::set_locale(&core.state.borrow().payload.settings.locale);

    GUI::run(Settings {
        window,
        flags: core,
        id: Some("ZebraPass".to_string()),
        default_font: Default::default(),
        default_text_size: 14.0,
        antialiasing: Default::default(),
        exit_on_close_request: true,
    })
}
