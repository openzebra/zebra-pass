//! -- Copyright (c) 2023 Rina Khasanshin
//! -- Email: hicarus@yandex.ru
//! -- Licensed under the GNU General Public License Version 3.0 (GPL-3.0)
use std::sync::{Arc, Mutex};

use crate::pages::Page;

use super::pages;
use iced::{executor, Application, Command, Element};
use zebra_lib::{core::core, settings::appearance::Themes};

use zebra_ui::{color::ZebraPalette, style};

#[derive(Debug)]
pub enum Routers {
    Loading(pages::loader::Loader),
    Locale(pages::locale::Locale),
    Interview(pages::inverview::Interview),
    Options(pages::options::Options),
    GenPhrase(pages::gen_phrase::GenPhrase),
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
    OptionsMessage(pages::options::OptionsMessage),
    GenPhraseMessage(pages::gen_phrase::GenPhraseMessage),
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

    fn new(arg: Self::Flags) -> (GUI, Command<Self::Message>) {
        let core = Arc::new(Mutex::new(arg));
        // let tmp = pages::gen_phrase::GenPhrase::new(Arc::clone(&core)).unwrap(); // TODO: Remove unwrap
        // let route = Routers::GenPhrase(tmp);
        let loader = pages::loader::Loader::new(Arc::clone(&core)).unwrap(); // TODO: Remove unwrap
        let route = Routers::Loading(loader);

        (
            Self { core, route },
            Command::perform(load(), |_| {
                GlobalMessage::LoadMessage(pages::loader::LoadMessage::Synced)
            }),
        )
    }

    fn update(&mut self, message: Self::Message) -> Command<Self::Message> {
        match message {
            GlobalMessage::LoadMessage(msg) => match &mut self.route {
                Routers::Loading(view) => view.update(msg),
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
            GlobalMessage::OptionsMessage(msg) => match &mut self.route {
                Routers::Options(view) => view.update(msg),
                _ => Command::none(),
            },
            GlobalMessage::GenPhraseMessage(msg) => match &mut self.route {
                Routers::GenPhrase(view) => view.update(msg),
                _ => Command::none(),
            },
            GlobalMessage::Route(route) => {
                self.route = route;
                Command::none()
            }
        }
    }

    fn subscription(&self) -> iced::Subscription<Self::Message> {
        iced::Subscription::batch([match &self.route {
            Routers::Loading(v) => v.subscription().map(|msg| GlobalMessage::LoadMessage(msg)),
            Routers::Interview(v) => v
                .subscription()
                .map(|msg| GlobalMessage::InterviewMessage(msg)),
            Routers::Locale(v) => v
                .subscription()
                .map(|msg| GlobalMessage::LocaleMessage(msg)),
            Routers::Options(v) => v
                .subscription()
                .map(|msg| GlobalMessage::OptionsMessage(msg)),
            Routers::GenPhrase(v) => v
                .subscription()
                .map(|msg| GlobalMessage::GenPhraseMessage(msg)),
        }])
    }

    fn view(&self) -> Element<'_, Self::Message, iced::Renderer<Self::Theme>> {
        match &self.route {
            Routers::Loading(l) => l.view().map(|msg| GlobalMessage::LoadMessage(msg)),
            Routers::Locale(l) => l.view().map(|msg| GlobalMessage::LocaleMessage(msg)),
            Routers::Interview(l) => l.view().map(|msg| GlobalMessage::InterviewMessage(msg)),
            Routers::Options(l) => l.view().map(|msg| GlobalMessage::OptionsMessage(msg)),
            Routers::GenPhrase(l) => l.view().map(|msg| GlobalMessage::GenPhraseMessage(msg)),
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
