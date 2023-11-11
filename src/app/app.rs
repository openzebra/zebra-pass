//! -- Copyright (c) 2023 Rina Khasanshin
//! -- Email: hicarus@yandex.ru
//! -- Licensed under the GNU General Public License Version 3.0 (GPL-3.0)
use super::router::Routers;
use crate::core::core::Core;
use crate::settings::appearance::Themes;
use iced::theme::Theme;
use iced::widget::{button, column, row, text, text_input};
use iced::{executor, Application, Command, Element};

pub struct App {
    theme: Theme,
    router: Routers,
    core: Core,
}

#[derive(Debug, Clone)]
pub enum Messages {}

impl Application for App {
    type Message = Messages;
    type Theme = Theme;
    type Executor = executor::Default;
    type Flags = Core;

    fn new(flags: Self::Flags) -> (Self, iced::Command<Self::Message>) {
        let core = flags;
        let theme = match core.state.borrow().payload.settings.appearance.theme {
            Themes::Dark => Theme::Dark,
            Themes::Light => Theme::Light,
            Themes::Auto => Theme::default(), // TODO: make it depends of OS
        };
        let mut router = Routers::default();

        if !core.state.borrow().payload.inited {
            router = Routers::LangChoose;
        }

        (
            Self {
                core,
                theme,
                router,
            },
            Command::none(),
        )
    }

    fn update(&mut self, message: Self::Message) -> Command<Self::Message> {
        dbg!("updated");

        Command::none()
    }

    fn title(&self) -> String {
        "Zebra Password manager".into()
    }

    fn view(&self) -> Element<'_, Self::Message, iced::Renderer<Self::Theme>> {
        column![row![
            text("test").size(50),
            text_input("adsadsa", "").size(30),
            // button("click").width(100).height(50).padding(5)
        ],]
        .into()
    }

    fn theme(&self) -> Self::Theme {
        self.theme.clone()
    }
}
