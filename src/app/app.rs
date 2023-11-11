//! -- Copyright (c) 2023 Rina Khasanshin
//! -- Email: hicarus@yandex.ru
//! -- Licensed under the GNU General Public License Version 3.0 (GPL-3.0)
use super::router::Routers;
use crate::core::core::Core;
use crate::errors::ZebraErrors;
use iced::theme::Theme;
use iced::widget::{button, column, text, Column};
use iced::{executor, Alignment, Application, Color, Command, Element, Length, Sandbox, Settings};

pub struct App {
    theme: Theme,
    router: Routers,
    core: Core,
}

#[derive(Debug, Clone)]
pub enum Messages {}

impl Application for App {
    type Message = ZebraErrors;
    type Theme = Theme;
    type Executor = executor::Default;
    type Flags = Core;

    fn new(flags: Self::Flags) -> (Self, iced::Command<Self::Message>) {
        let core = flags;
        let theme = Theme::Dark;
        let router = Routers::default();

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

    fn view(&self) -> Element<'_, Self::Message> {
        column![text("test").size(50),].into()
    }

    fn theme(&self) -> Self::Theme {
        self.theme.clone()
    }
}
