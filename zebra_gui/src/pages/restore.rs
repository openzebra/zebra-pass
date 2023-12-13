//! -- Copyright (c) 2023 Rina Khasanshin
//! -- Email: hicarus@yandex.ru
//! -- Licensed under the GNU General Public License Version 3.0 (GPL-3.0)

use std::sync::{Arc, Mutex};
use zebra_lib::{core::core::Core, errors::ZebraErrors};

use crate::{
    gui::{GlobalMessage, Routers},
    rust_i18n::t,
};

use super::{options::Options, Page};
use iced::{alignment::Horizontal, widget::Space, Command, Length, Subscription};
use zebra_ui::widget::*;

#[derive(Debug)]
pub struct Restore {
    words: Vec<String>,
    core: Arc<Mutex<Core>>,
}

#[derive(Debug, Clone)]
pub enum RestoreMessage {
    Back,
    Next,
}

impl Page for Restore {
    type Message = RestoreMessage;

    fn new(core: Arc<Mutex<Core>>) -> Result<Self, ZebraErrors> {
        let words = Vec::new();
        Ok(Self { core, words })
    }

    fn subscription(&self) -> Subscription<Self::Message> {
        Subscription::none()
    }

    fn update(&mut self, message: Self::Message) -> Command<GlobalMessage> {
        match message {
            RestoreMessage::Back => {
                // TODO: remove unwrap!
                let options = Options::new(Arc::clone(&self.core)).unwrap();
                let route = Routers::Options(options);
                Command::perform(std::future::ready(1), |_| GlobalMessage::Route(route))
            }
            RestoreMessage::Next => Command::none(),
        }
    }

    fn view(&self) -> Element<Self::Message> {
        let zebra_print = zebra_ui::image::zebra_print_view();
        let print_col = Column::new()
            .width(220)
            .height(Length::Fill)
            .push(zebra_print);
        let title = Text::new(t!("restore_page_title"))
            .size(34)
            .horizontal_alignment(Horizontal::Center);
        let back_btn = Button::new(zebra_ui::image::back_icon().height(50).width(50))
            .padding(0)
            .style(zebra_ui::style::button::Button::Transparent)
            .on_press(RestoreMessage::Back);
        let forward_btn = Button::new(zebra_ui::image::forward_icon().height(50).width(50))
            .padding(0)
            .style(zebra_ui::style::button::Button::Transparent)
            .on_press(RestoreMessage::Next);
        let btns_row = Row::new().push(back_btn).push(forward_btn);
        let content_col = Column::new()
            .push(title)
            .push(btns_row)
            .width(Length::Fill)
            .height(Length::Fill)
            .align_items(iced::Alignment::Center)
            .padding(10);
        let row = Row::new()
            .width(Length::Fill)
            .push(print_col)
            .push(content_col);

        Container::new(row)
            .height(Length::Fill)
            .width(Length::Fill)
            .into()
    }
}
