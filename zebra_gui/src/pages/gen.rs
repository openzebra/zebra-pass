//! -- Copyright (c) 2023 Rina Khasanshin
//! -- Email: hicarus@yandex.ru
//! -- Licensed under the GNU General Public License Version 3.0 (GPL-3.0)

use std::sync::{Arc, Mutex};

use iced::widget::{text_input, Space};
use iced::{Command, Length, Subscription};
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
    value: String,
}

#[derive(Debug, Clone, Copy)]
pub enum GeneratorMessage {
    RouteHome,
    RouteSettings,
    Copy,
    Refresh,
}

impl Page for Generator {
    type Message = GeneratorMessage;

    fn new(core: Arc<Mutex<Core>>) -> Result<Self, ZebraErrors> {
        Ok(Self {
            core,
            value: "DNSA(*3h2nger920fn)".to_owned(),
        })
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
            GeneratorMessage::Copy => Command::none(),
            GeneratorMessage::Refresh => Command::none(),
        }
    }

    fn view(&self) -> Element<Self::Message> {
        NavBar::<Self::Message>::new()
            .set_route(NavRoute::Gen)
            .on_home(GeneratorMessage::RouteHome)
            .on_settings(GeneratorMessage::RouteSettings)
            .view(self.view_entropy_gen())
            .into()
    }
}

impl Generator {
    pub fn view_entropy_gen(&self) -> Container<GeneratorMessage> {
        let row = Row::new()
            .push(self.view_generator())
            .align_items(iced::Alignment::Center)
            .height(Length::Fill);
        let col = Column::new()
            .push(row)
            .align_items(iced::Alignment::Center)
            .width(Length::Fill);

        Container::new(col).height(Length::Fill).width(Length::Fill)
    }

    pub fn view_generator(&self) -> Container<GeneratorMessage> {
        let entropy = text_input("", &self.value)
            .size(16)
            .padding(8)
            .width(250)
            // .id(self.input_id.clone())
            .style(zebra_ui::style::text_input::TextInput::Transparent);
        let reload_btn = Button::new(zebra_ui::image::reload_icon().height(30).width(30))
            .padding(0)
            .style(zebra_ui::style::button::Button::Transparent)
            .on_press(GeneratorMessage::Refresh);
        let copy_btn = Button::new(zebra_ui::image::copy_icon().height(25).width(25))
            .padding(0)
            .style(zebra_ui::style::button::Button::Transparent)
            .on_press(GeneratorMessage::Copy);

        let box_row: Row<'_, GeneratorMessage> = Row::new()
            .align_items(iced::Alignment::Center)
            .push(copy_btn)
            .push(entropy)
            .push(Space::new(20, 0))
            .push(reload_btn);
        let border_box = Container::new(box_row)
            .style(zebra_ui::style::container::Container::SecondaryRoundedBox)
            .padding(16);
        let col = Column::new().push(border_box);

        Container::new(col)
    }
}
