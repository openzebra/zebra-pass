//! -- Copyright (c) 2023 Rina Khasanshin
//! -- Email: hicarus@yandex.ru
//! -- Licensed under the GNU General Public License Version 3.0 (GPL-3.0)

use std::sync::{Arc, Mutex};

use crate::components::passgen::PassGenForm;
use crate::rust_i18n::t;
use iced::{clipboard, Command, Length, Subscription};
use zebra_lib::{core::core::Core, errors::ZebraErrors};
use zebra_ui::widget::*;

use crate::components::home_nav_bar::{NavBar, NavRoute};
use crate::gui::{GlobalMessage, Routers};

use super::home::Home;
use super::settings::Settings;
use super::Page;

#[derive(Debug)]
pub struct Generator {
    core: Arc<Mutex<Core>>,
}

#[derive(Debug, Clone)]
pub enum GeneratorMessage {
    RouteHome,
    RouteSettings,
    CopyValue,
}

impl Page for Generator {
    type Message = GeneratorMessage;

    fn new(core: Arc<Mutex<Core>>) -> Result<Self, ZebraErrors> {
        Ok(Self { core })
    }

    fn subscription(&self) -> Subscription<Self::Message> {
        Subscription::none()
    }

    fn update(&mut self, message: Self::Message) -> iced::Command<GlobalMessage> {
        match message {
            GeneratorMessage::RouteHome => {
                // TODO: remove unwrap!
                let home = Home::new(Arc::clone(&self.core)).unwrap();
                let route = Routers::Home(home);

                return Command::perform(std::future::ready(1), |_| GlobalMessage::Route(route));
            }
            GeneratorMessage::RouteSettings => {
                // TODO: remove unwrap!
                let settings = Settings::new(Arc::clone(&self.core)).unwrap();
                let route = Routers::Settings(settings);

                return Command::perform(std::future::ready(1), |_| GlobalMessage::Route(route));
            }
            GeneratorMessage::CopyValue => clipboard::write::<GlobalMessage>("".to_owned()),
        }
    }

    fn view(&self) -> Element<Self::Message> {
        NavBar::<Self::Message>::new()
            .set_route(NavRoute::Gen)
            .on_home(GeneratorMessage::RouteHome)
            .on_settings(GeneratorMessage::RouteSettings)
            .view(self.view_password_gen())
            .into()
    }
}

impl Generator {
    pub fn view_password_gen(&self) -> Container<GeneratorMessage> {
        let pass_gen_form = PassGenForm::new(22, |v| {
            dbg!(v);
            GeneratorMessage::CopyValue
        })
        .unwrap(); // TODO: add Error message page.

        Container::new(pass_gen_form)
            .width(Length::Fill)
            .height(Length::Fill)
    }
}
