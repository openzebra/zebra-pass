//! -- Copyright (c) 2023 Rina Khasanshin
//! -- Email: hicarus@yandex.ru
//! -- Licensed under the GNU General Public License Version 3.0 (GPL-3.0)
use super::pages::locale::LocalePage;
use super::router::Routers;
use crate::core::core::Core;
use crate::settings::appearance::Themes;
use iced::theme::Theme;
use iced::{executor, Application, Command, Element};

pub struct App {
    router: Routers,
    core: Core,
}

#[derive(Debug, Clone)]
pub enum RouteMessages {
    Next(Routers),
    Back,
}

impl Application for App {
    type Message = RouteMessages;
    type Theme = Theme;
    type Executor = executor::Default;
    type Flags = Core;

    fn new(flags: Self::Flags) -> (Self, iced::Command<Self::Message>) {
        let core = flags;
        let mut router = Routers::default();

        if !core.state.borrow().payload.inited {
            router = Routers::LangChoose;
        }

        (Self { core, router }, Command::none())
    }

    fn update(&mut self, message: Self::Message) -> Command<Self::Message> {
        dbg!("updated");

        Command::none()
    }

    fn title(&self) -> String {
        "Zebra Password manager".into()
    }

    fn view(&self) -> Element<'_, Self::Message, iced::Renderer<Self::Theme>> {
        match self.router {
            // Routers::Lock => Default::default(),
            // Routers::Home => Default::default(),
            // Routers::Start => Default::default(),
            // Routers::Login => Default::default(),
            // Routers::Privacy => Default::default(),
            // Routers::Mnemonic => Default::default(),
            // Routers::SetupAccount => Default::default(),
            Routers::LangChoose => LocalePage::from(&self.core).view().into(),
            _ => panic!("no implemented"),
        }
    }

    fn theme(&self) -> Self::Theme {
        match self.core.state.borrow().payload.settings.appearance.theme {
            Themes::Dark => Theme::Dark,
            Themes::Light => Theme::Light,
            Themes::Auto => Theme::default(), // TODO: make it depends of OS
        }
    }
}
