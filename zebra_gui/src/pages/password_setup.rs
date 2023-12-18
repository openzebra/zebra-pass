//! -- Copyright (c) 2023 Rina Khasanshin
//! -- Email: hicarus@yandex.ru
//! -- Licensed under the GNU General Public License Version 3.0 (GPL-3.0)

use super::{gen_phrase::GenPhrase, restore::Restore, Page};
use crate::{
    gui::{GlobalMessage, Routers},
    rust_i18n::t,
};
use iced::{alignment::Horizontal, widget::text_input, Command, Length, Subscription};
use std::sync::{Arc, Mutex};
use zebra_lib::{bip39::mnemonic::Mnemonic, core::core::Core, errors::ZebraErrors};
use zebra_ui::widget::*;

#[derive(Debug)]
pub enum LastRoute {
    Gen,
    Restore,
}

#[derive(Debug)]
pub struct PasswordSetup {
    pub last_route: LastRoute,
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
        let last_route = LastRoute::Gen;

        Ok(Self {
            core,
            mnemonic,
            last_route,
        })
    }

    fn subscription(&self) -> Subscription<Self::Message> {
        Subscription::none()
    }

    fn update(&mut self, message: Self::Message) -> Command<GlobalMessage> {
        match message {
            PasswordSetupMessage::Next => Command::none(),
            PasswordSetupMessage::Back => {
                let route = match self.last_route {
                    LastRoute::Gen => {
                        let gen_phrase = GenPhrase::new(Arc::clone(&self.core)).unwrap();

                        Routers::GenPhrase(gen_phrase)
                    }
                    LastRoute::Restore => {
                        let restore = Restore::new(Arc::clone(&self.core)).unwrap();

                        Routers::Restore(restore)
                    }
                };

                return Command::perform(std::future::ready(1), |_| GlobalMessage::Route(route));
            }
        }
    }

    fn view(&self) -> Element<Self::Message> {
        let zebra_print = zebra_ui::image::zebra_print_view();
        let print_col = Column::new()
            .width(220)
            .height(Length::Fill)
            .push(zebra_print);
        let title = Text::new(t!("setup_account_and_password"))
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

    pub fn view_error<'a>(&self) -> Container<'a, PasswordSetupMessage> {
        let error_message = Text::new(t!("mnemonic_is_not_inited"))
            .size(16)
            .style(zebra_ui::style::text::Text::Dabger)
            .horizontal_alignment(Horizontal::Center);

        Container::new(Column::new().push(error_message))
    }

    pub fn view_info<'a>(&self) -> Container<'a, PasswordSetupMessage> {
        let options_col = Column::new()
            .align_items(iced::Alignment::Center)
            .padding(20)
            .height(Length::Fill)
            .width(Length::Fill);
        Container::new(options_col)
            .height(152)
            .width(350)
            .style(zebra_ui::style::container::Container::Bordered)
    }

    pub fn view_content<'a>(&self, m: &Mnemonic) -> Container<'a, PasswordSetupMessage> {
        let info = self.view_info();
        let passowrd = text_input("", "")
            .size(14)
            .width(90)
            .password()
            .style(zebra_ui::style::text_input::TextInput::Primary);
        let in_row = Row::new().push(passowrd);
        let main_col = Column::new().push(info).push(in_row);

        Container::new(main_col)
    }
}
