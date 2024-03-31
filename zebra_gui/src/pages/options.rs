//! -- Copyright (c) 2023 Rina Khasanshin
//! -- Email: hicarus@yandex.ru
//! -- Licensed under the GNU General Public License Version 3.0 (GPL-3.0)

use std::sync::{Arc, Mutex};
use zebra_lib::{core::core::Core, errors::ZebraErrors};
use zebra_ui::config::PRINT_WIDTH;

use crate::{
    gui::{GlobalMessage, Routers},
    rust_i18n::t,
};

use super::{
    error::ErrorPage,
    gen_phrase::GenPhrase,
    inverview::{Interview, SlideStep},
    restore::Restore,
    Page,
};
use iced::{alignment::Horizontal, Command, Length, Subscription};
use iced::{
    widget::{Button, Column, Container, Row, Space, Text},
    Element,
};

#[derive(Debug)]
pub struct Options {
    core: Arc<Mutex<Core>>,
}

#[derive(Debug, Clone)]
pub enum OptionsMessage {
    Back,
    Restore,
    Create,
}

impl Page for Options {
    type Message = OptionsMessage;

    fn new(core: Arc<Mutex<Core>>) -> Result<Self, ZebraErrors> {
        Ok(Self { core })
    }

    fn subscription(&self) -> Subscription<Self::Message> {
        Subscription::none()
    }

    fn update(&mut self, message: Self::Message) -> Command<GlobalMessage> {
        match message {
            OptionsMessage::Back => match Interview::new(Arc::clone(&self.core)) {
                Ok(mut inverview) => {
                    inverview.step = SlideStep::Quantom;

                    let route = Routers::Interview(inverview);
                    Command::perform(std::future::ready(1), |_| GlobalMessage::Route(route))
                }
                Err(e) => {
                    let route = Routers::ErrorPage(ErrorPage::from(e.to_string()));
                    Command::perform(std::future::ready(1), |_| GlobalMessage::Route(route))
                }
            },
            OptionsMessage::Create => match GenPhrase::new(Arc::clone(&self.core)) {
                Ok(gen_phrase) => {
                    let route = Routers::GenPhrase(gen_phrase);
                    Command::perform(std::future::ready(1), |_| GlobalMessage::Route(route))
                }
                Err(e) => {
                    let route = Routers::ErrorPage(ErrorPage::from(e.to_string()));
                    Command::perform(std::future::ready(1), |_| GlobalMessage::Route(route))
                }
            },
            OptionsMessage::Restore => match Restore::new(Arc::clone(&self.core)) {
                Ok(restore) => {
                    let route = Routers::Restore(restore);
                    Command::perform(std::future::ready(1), |_| GlobalMessage::Route(route))
                }
                Err(e) => {
                    let route = Routers::ErrorPage(ErrorPage::from(e.to_string()));
                    Command::perform(std::future::ready(1), |_| GlobalMessage::Route(route))
                }
            },
        }
    }

    fn view(&self) -> Element<'_, Self::Message> {
        let zebra_print = zebra_ui::image::zebra_print_view();
        let print_col = Column::new()
            .width(PRINT_WIDTH)
            .height(Length::Fill)
            .push(zebra_print);
        let title = Text::new(t!("zebra_name"))
            .size(34)
            .horizontal_alignment(Horizontal::Center);
        let back_btn = Button::new(zebra_ui::image::back_icon().height(50).width(50))
            .padding(0)
            .style(zebra_ui::styles::button::transparent)
            .on_press(OptionsMessage::Back);
        let col = Column::new()
            .height(Length::Fill)
            .width(Length::Fill)
            .padding(15)
            .align_items(iced::Alignment::Center)
            .push(title)
            .push(Space::new(0, 20))
            .push(self.options_view())
            .push(Space::new(0, 20))
            .push(back_btn);
        let row = Row::new().width(Length::Fill).push(print_col).push(col);

        Container::new(row)
            .height(Length::Fill)
            .width(Length::Fill)
            .into()
    }
}

impl Options {
    pub fn options_view<'a>(&self) -> Container<'a, OptionsMessage> {
        let options_title = Text::new(t!("options_title")).size(25);
        let create_btn = Button::new(
            Text::new(t!("create_btn"))
                .horizontal_alignment(Horizontal::Center)
                .width(Length::Fill)
                .size(16),
        )
        .padding(8)
        .width(200)
        .style(zebra_ui::styles::button::outline_primary)
        .on_press(OptionsMessage::Create);
        let restore_btn = Button::new(
            Text::new(t!("restore_btn"))
                .horizontal_alignment(Horizontal::Center)
                .width(Length::Fill)
                .size(16),
        )
        .padding(8)
        .width(200)
        .style(zebra_ui::styles::button::outline_primary)
        .on_press(OptionsMessage::Restore);
        let zebra_hooves = zebra_ui::image::zebra_hooves()
            .width(68)
            .height(Length::Fill);
        let options_col = Column::new()
            .align_items(iced::Alignment::Center)
            .padding(20)
            .height(Length::Fill)
            .width(Length::Fill)
            .push(options_title)
            .push(Space::new(0, 15))
            .push(zebra_hooves)
            .push(Space::new(0, 15))
            .push(create_btn)
            .push(Space::new(0, 5))
            .push(restore_btn);

        Container::new(options_col)
            .height(252)
            // .style(zebra_ui::style::container::Container::Bordered)
            .width(350)
    }
}
