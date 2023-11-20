//! -- Copyright (c) 2023 Rina Khasanshin
//! -- Email: hicarus@yandex.ru
//! -- Licensed under the GNU General Public License Version 3.0 (GPL-3.0)
use super::app::view::loader::{LoadMessage, Loader};
use iced::{executor, Application, Command, Element};
use zebra_lib::{core::core, settings::appearance::Themes};

use zebra_ui::{color::ZebraPalette, theme};

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
            GlobalMessage::LoadMessage(msg) => match &mut self.route {
                Routers::Loading(view) => view.update::<GlobalMessage>(*msg),
            },
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
