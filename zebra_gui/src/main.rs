//! -- Copyright (c) 2023 Rina Khasanshin
//! -- Email: hicarus@yandex.ru
//! -- Licensed under the GNU General Public License Version 3.0 (GPL-3.0)
extern crate rust_i18n;

use std::io::{Error, ErrorKind};

use app::view::loader::{LoadMessage, Loader};
use iced::{executor, window, Application, Command, Element, Settings};
use rust_i18n::i18n;
use zebra_lib::{core::core, settings::appearance::Themes};

use zebra_ui::{color::ZebraPalette, theme};

mod app;

i18n!("locales", fallback = "en");

pub enum Routers {
    Loading(Loader),
    // Locale,
}

pub struct GUI {
    core: core::Core,
    route: Routers,
}

#[derive(Debug)]
pub enum GlobalMessage {
    LoadMessage(LoadMessage),
    Event(iced::Event),
}

impl Application for GUI {
    type Executor = executor::Default;
    type Message = GlobalMessage;
    type Flags = zebra_lib::core::core::Core;
    type Theme = theme::Theme;

    fn title(&self) -> String {
        "Zebrapass".into()
    }

    fn new(core: Self::Flags) -> (GUI, Command<Self::Message>) {
        let route = Routers::Loading(Loader::new());
        (Self { core, route }, Command::none())
    }

    fn update(&mut self, message: Self::Message) -> Command<Self::Message> {
        match &message {
            GlobalMessage::Event(e) => match e {
                _ => {
                    // TODO: native events...
                    Command::none()
                }
            },
            GlobalMessage::LoadMessage(msg) => {
                match &mut self.route {
                    Routers::Loading(view) => {
                        let _ = view.update(*msg);
                    }
                };
                Command::none()
            }
        }
    }

    fn subscription(&self) -> iced::Subscription<Self::Message> {
        iced::Subscription::batch([
            match &self.route {
                Routers::Loading(v) => v.subscription().map(|msg| GlobalMessage::LoadMessage(msg)),
            },
            iced::subscription::events().map(Self::Message::Event),
        ])
    }

    fn view(&self) -> Element<'_, Self::Message, iced::Renderer<Self::Theme>> {
        match &self.route {
            // Routers::Locale => panic!("not impl yet"),
            Routers::Loading(l) => l.view().map(|msg| GlobalMessage::LoadMessage(msg)),
        }
    }

    fn scale_factor(&self) -> f64 {
        1.0
    }

    fn theme(&self) -> Self::Theme {
        match self.core.state.borrow().payload.settings.appearance.theme {
            Themes::Dark => theme::Theme::Dark(ZebraPalette::DARK),
            Themes::Light => theme::Theme::Light(ZebraPalette::LIGHT),
            Themes::Auto => match dark_light::detect() {
                dark_light::Mode::Dark => theme::Theme::Dark(ZebraPalette::DARK),
                dark_light::Mode::Light => theme::Theme::Light(ZebraPalette::LIGHT),
                dark_light::Mode::Default => theme::Theme::Dark(ZebraPalette::DARK),
            },
        }
    }
}

fn main() -> iced::Result {
    let core = match core::Core::new() {
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

    rust_i18n::set_locale(&core.state.borrow().payload.settings.locale);

    GUI::run(Settings {
        window: window::Settings {
            size: (750, 450),
            resizable: false,
            icon: Some(zebra_ui::image::liana_app_icon()),
            ..Default::default()
        },
        flags: core,
        id: Some("ZebraPass".to_string()),
        default_font: Default::default(),
        default_text_size: 14.0,
        antialiasing: Default::default(),
        exit_on_close_request: true,
    })
}
