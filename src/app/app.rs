//! -- Copyright (c) 2023 Rina Khasanshin
//! -- Email: hicarus@yandex.ru
//! -- Licensed under the GNU General Public License Version 3.0 (GPL-3.0)
use super::message::GlobalMessages;
use super::theme::ZebraPalette;
use super::{pages::locale::LocalePage, router::Routers};
use crate::core::core::Core;
use crate::settings::appearance::Themes;
use iced::theme::{Palette, Theme};
use iced::{executor, Application, Command, Element};

pub struct App {
    router: Routers,
    core: Core,
}

impl Application for App {
    type Message = GlobalMessages;
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
        let dark = Theme::custom(Palette {
            background: ZebraPalette::DARK.window_background,
            text: ZebraPalette::DARK.window_background_inverse,
            primary: ZebraPalette::DARK.primary,
            success: ZebraPalette::DARK.success,
            danger: ZebraPalette::DARK.danger,
        });
        let light = Theme::custom(Palette {
            background: ZebraPalette::LIGHT.window_background,
            text: ZebraPalette::LIGHT.window_background_inverse,
            primary: ZebraPalette::LIGHT.primary,
            success: ZebraPalette::LIGHT.success,
            danger: ZebraPalette::LIGHT.danger,
        });

        match self.core.state.borrow().payload.settings.appearance.theme {
            Themes::Dark => dark,
            Themes::Light => light,
            Themes::Auto => match dark_light::detect() {
                dark_light::Mode::Dark => dark,
                dark_light::Mode::Light => light,
                dark_light::Mode::Default => dark,
            },
        }
    }
}
