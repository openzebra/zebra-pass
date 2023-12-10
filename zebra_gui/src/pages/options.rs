//! -- Copyright (c) 2023 Rina Khasanshin
//! -- Email: hicarus@yandex.ru
//! -- Licensed under the GNU General Public License Version 3.0 (GPL-3.0)

use std::sync::{Arc, Mutex};
use zebra_lib::{core::core::Core, errors::ZebraErrors};

use crate::{
    gui::{GlobalMessage, Routers},
    rust_i18n::t,
};

use super::{
    inverview::{Interview, SlideStep},
    Page,
};
use iced::{alignment::Horizontal, widget::Space, Command, Length, Subscription};
use zebra_ui::widget::*;

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
            OptionsMessage::Back => {
                let mut inverview = Interview::new(Arc::clone(&self.core)).unwrap();
                inverview.step = SlideStep::Quantom;
                let route = Routers::Interview(inverview);
                Command::perform(std::future::ready(1), |_| GlobalMessage::Route(route))
            }
            OptionsMessage::Create => Command::none(),
            OptionsMessage::Restore => Command::none(),
        }
    }

    fn view(&self) -> Element<Self::Message> {
        let zebra_print = zebra_ui::image::zebra_print_view();
        let print_col = Column::new()
            .width(220)
            .height(Length::Fill)
            .push(zebra_print);
        let title = Text::new(t!("zebra_name"))
            .size(34)
            .horizontal_alignment(Horizontal::Center);
        let forward_btn = Button::new(zebra_ui::image::back_icon().height(50).width(50))
            .padding(0)
            .style(zebra_ui::style::button::Button::Transparent)
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
            .push(forward_btn);
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
        .on_press(OptionsMessage::Create)
        .style(zebra_ui::style::button::Button::OutlinePrimary);
        let restore_btn = Button::new(
            Text::new(t!("restore_btn"))
                .horizontal_alignment(Horizontal::Center)
                .width(Length::Fill)
                .size(16),
        )
        .padding(8)
        .width(200)
        .on_press(OptionsMessage::Restore)
        .style(zebra_ui::style::button::Button::OutlinePrimary);
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
            .width(350)
            .style(zebra_ui::style::container::Container::Bordered)
    }
}
