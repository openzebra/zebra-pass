//! -- Copyright (c) 2023 Rina Khasanshin
//! -- Email: hicarus@yandex.ru
//! -- Licensed under the GNU General Public License Version 3.0 (GPL-3.0)
extern crate rust_i18n;

use error::GUIError;
use gui::Gui;
use iced::{advanced::Application, window, Settings, Size};
use rust_i18n::i18n;
use zebra_lib::core::Core;
use zebra_ui::config::{WIN_HEIGHT, WIN_WIDTH};

mod components;
mod error;
mod gui;
mod pages;

i18n!("zebra_gui/locales", fallback = "en");

fn main() -> iced::Result {
    let window = window::Settings {
        size: Size {
            width: WIN_WIDTH,
            height: WIN_HEIGHT,
        },
        resizable: false,
        icon: Some(zebra_ui::image::zebra_app_icon()),
        ..Default::default()
    };
    let mut core = match Core::new() {
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

    rust_i18n::set_locale(core.state.settings.locale.symbol());

    Gui::run(Settings {
        window,
        fonts: [].into(),
        flags: core,
        id: Some("ZebraPass".to_string()),
        default_font: Default::default(),
        default_text_size: 14.0.into(),
        antialiasing: Default::default(),
    })
}
