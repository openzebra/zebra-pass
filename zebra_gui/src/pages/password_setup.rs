//! -- Copyright (c) 2023 Rina Khasanshin
//! -- Email: hicarus@yandex.ru
//! -- Licensed under the GNU General Public License Version 3.0 (GPL-3.0)

use super::Page;
use crate::{
    gui::{GlobalMessage, Routers},
    rust_i18n::t,
};
use iced::{alignment::Horizontal, Command, Length, Subscription};
use std::sync::{Arc, Mutex};
use zebra_lib::{bip39::mnemonic::Mnemonic, core::core::Core, errors::ZebraErrors};
use zebra_ui::widget::*;

#[derive(Debug)]
pub struct PasswordSetup {
    core: Arc<Mutex<Core>>,
    mnemonic: Option<Mnemonic>,
}

#[derive(Debug, Clone)]
pub enum PasswordSetupMessage {
    Next,
    Back,
}

impl Page for PasswordSetup {
    type Message = PasswordSetupMessage;

    fn new(core: Arc<Mutex<Core>>) -> Result<Self, ZebraErrors> {
        let mnemonic = None;
        Ok(Self { core, mnemonic })
    }

    fn subscription(&self) -> Subscription<Self::Message> {
        Subscription::none()
    }

    fn update(&mut self, message: Self::Message) -> Command<GlobalMessage> {
        match message {
            PasswordSetupMessage::Next => Command::none(),
            PasswordSetupMessage::Back => Command::none(),
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
        let forward_icon = zebra_ui::image::forward_icon()
            .height(50)
            .width(50)
            .style(zebra_ui::style::svg::Svg::Primary);
        let back_btn = Button::new(zebra_ui::image::back_icon().height(50).width(50))
            .padding(0)
            .style(zebra_ui::style::button::Button::Transparent)
            .on_press(PasswordSetupMessage::Back);
        let forward_btn = Button::new(forward_icon)
            .padding(0)
            .style(zebra_ui::style::button::Button::Transparent)
            .on_press(PasswordSetupMessage::Next);
        let btns_row = Row::new().push(back_btn).push(forward_btn);
        let content_col = Column::new()
            .height(Length::Fill)
            .width(Length::Fill)
            .align_items(iced::Alignment::Center)
            .push(title)
            .push(match &self.mnemonic {
                Some(m) => self.view_content(m),
                None => self.view_error(),
            })
            .push(btns_row);
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

impl PasswordSetup {
    pub fn set_mnemonic(&mut self, m: Mnemonic) {
        self.mnemonic = Some(m);
    }

    pub fn view_error(&self) -> Column<'_, PasswordSetupMessage> {
        let error_message = Text::new(t!("mnemonic_is_not_inited"))
            .size(16)
            .style(zebra_ui::style::text::Text::Dabger)
            .horizontal_alignment(Horizontal::Center);
        Column::new().push(error_message)
    }

    pub fn view_content(&self, m: &Mnemonic) -> Column<'_, PasswordSetupMessage> {
        Column::new()
    }
}
