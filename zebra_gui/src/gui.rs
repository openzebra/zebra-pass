//! -- Copyright (c) 2023 Rina Khasanshin
//! -- Email: hicarus@yandex.ru
//! -- Licensed under the GNU General Public License Version 3.0 (GPL-3.0)
use std::sync::{Arc, Mutex};

use crate::pages::{locale::Locale, Page};

use super::pages;
use iced::{executor, Application, Command, Element};
use zebra_lib::{core::core, settings::appearance::Themes};

use zebra_ui::{color::ZebraPalette, style};

#[derive(Debug)]
pub enum Routers {
    Loading(pages::loader::Loader),
    Locale(pages::locale::Locale),
    Interview(pages::inverview::Interview),
}

pub struct GUI {
    core: Arc<Mutex<core::Core>>,
    route: Routers,
}

#[derive(Debug)]
pub enum GlobalMessage {
    LoadMessage(pages::loader::LoadMessage),
    LocaleMessage(pages::locale::LocaleMessage),
    InterviewMessage(pages::inverview::InterviewMessage),
    Event(iced::Event),
    Route(Routers),
}

async fn load() -> Result<(), ()> {
    std::thread::sleep(std::time::Duration::from_millis(100));
    // TODO: make it load when server sync added.
    Ok(())
}

impl Application for GUI {
    type Executor = executor::Default;
    type Message = GlobalMessage;
    type Flags = zebra_lib::core::core::Core;
    type Theme = style::Theme;

    fn title(&self) -> String {
        "Zebrapass".into()
    }

    fn new(core: Self::Flags) -> (GUI, Command<Self::Message>) {
        let route = Routers::Loading(pages::loader::Loader::new());

        (
            Self { core, route },
            Command::perform(load(), |_| {
                GlobalMessage::LoadMessage(pages::loader::LoadMessage::Synced)
            }),
        )
    }

    fn update(&mut self, message: Self::Message) -> Command<Self::Message> {
        match message {
            GlobalMessage::Event(e) => match e {
                _ => {
                    // TODO: native events...
                    Command::none()
                }
            },
            GlobalMessage::LoadMessage(_msg) => match &self.route {
                Routers::Loading(_view) => {
                    let route = Routers::Locale(Locale::new(Arc::clone(&self.core)));
                    Command::perform(std::future::ready(1), |_| GlobalMessage::Route(route))
                }
                _ => Command::none(),
            },
            GlobalMessage::LocaleMessage(msg) => match &mut self.route {
                Routers::Locale(view) => view.update(msg),
                _ => Command::none(),
            },
            GlobalMessage::InterviewMessage(msg) => match &mut self.route {
                Routers::Interview(view) => view.update(msg),
                _ => Command::none(),
            },
            GlobalMessage::Route(route) => {
                self.route = route;
                Command::none()
            }
        }
    }

    fn subscription(&self) -> iced::Subscription<Self::Message> {
        iced::Subscription::batch([
            match &self.route {
                Routers::Loading(v) => v.subscription().map(|msg| GlobalMessage::LoadMessage(msg)),
                Routers::Interview(v) => v
                    .subscription()
                    .map(|msg| GlobalMessage::InterviewMessage(msg)),
                Routers::Locale(v) => v
                    .subscription()
                    .map(|msg| GlobalMessage::LocaleMessage(msg)),
            },
            // iced::subscription::Subscription::map(Self::Message::Event),
        ])
    }

    fn view(&self) -> Element<'_, Self::Message, iced::Renderer<Self::Theme>> {
        match &self.route {
            Routers::Loading(l) => l.view().map(|msg| GlobalMessage::LoadMessage(msg)),
            Routers::Locale(l) => l.view().map(|msg| GlobalMessage::LocaleMessage(msg)),
            Routers::Interview(l) => l.view().map(|msg| GlobalMessage::InterviewMessage(msg)),
        }
    }

    fn scale_factor(&self) -> f64 {
        1.0
    }

    fn theme(&self) -> Self::Theme {
        // TODO: Remove unwrap.
        match self.core.lock().unwrap().state.settings.appearance.theme {
            Themes::Dark => style::Theme::Dark(ZebraPalette::DARK),
            Themes::Light => style::Theme::Light(ZebraPalette::LIGHT),
            Themes::Auto => match dark_light::detect() {
                dark_light::Mode::Dark => style::Theme::Dark(ZebraPalette::DARK),
                dark_light::Mode::Light => style::Theme::Light(ZebraPalette::LIGHT),
                dark_light::Mode::Default => style::Theme::Dark(ZebraPalette::DARK),
            },
        }
    }
}
