//! -- Copyright (c) 2023 Rina Khasanshin
//! -- Email: hicarus@yandex.ru
//! -- Licensed under the GNU General Public License Version 3.0 (GPL-3.0)

use super::{gen_phrase::GenPhrase, restore::Restore, Page};
use crate::{
    gui::{GlobalMessage, Routers},
    rust_i18n::t,
};
use iced::widget::Checkbox;
use iced::{
    alignment::Horizontal,
    widget::{text_input, Space},
    Command, Length, Subscription,
};
use std::sync::{Arc, Mutex};
use zebra_lib::{bip39::mnemonic::Mnemonic, core::core::Core, errors::ZebraErrors};
use zebra_ui::widget::*;

pub const MIN_PASSWORD_SIZE: usize = 6;

#[derive(Debug)]
pub enum LastRoute {
    Gen,
    Restore,
}

#[derive(Debug)]
pub struct PasswordSetup {
    pub last_route: LastRoute,
    password: String,
    confirm_password: String,
    email: String,
    approved: bool,
    server_sync: bool,
    email_restore: bool,
    core: Arc<Mutex<Core>>,
    mnemonic: Option<Mnemonic>,
}

#[derive(Debug, Clone)]
pub enum PasswordSetupMessage {
    Next,
    Back,
    ApprovePolicy(bool),
    ApproveServerSync(bool),
    ApproveEmailRestore(bool),
    OnPasswordInputed(String),
    OnConfirmPasswordInputed(String),
    OnEmailInputed(String),
}

impl Page for PasswordSetup {
    type Message = PasswordSetupMessage;

    fn new(core: Arc<Mutex<Core>>) -> Result<Self, ZebraErrors> {
        let mnemonic = None;
        let last_route = LastRoute::Gen;
        let password = String::new();
        let confirm_password = String::new();
        let approved = false;
        let server_sync = true;
        let email_restore = true;
        let email = String::new();

        Ok(Self {
            email,
            core,
            email_restore,
            server_sync,
            approved,
            mnemonic,
            last_route,
            password,
            confirm_password,
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
            PasswordSetupMessage::ApprovePolicy(v) => {
                self.approved = v;
                Command::none()
            }
            PasswordSetupMessage::OnPasswordInputed(v) => {
                self.password = v;
                Command::none()
            }
            PasswordSetupMessage::OnConfirmPasswordInputed(v) => {
                self.confirm_password = v;
                Command::none()
            }
            PasswordSetupMessage::OnEmailInputed(v) => {
                self.email = v;
                Command::none()
            }
            PasswordSetupMessage::ApproveServerSync(v) => {
                self.server_sync = v;
                Command::none()
            }
            PasswordSetupMessage::ApproveEmailRestore(v) => {
                self.email_restore = v;

                if !v {
                    self.email = String::new();
                }

                Command::none()
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
        let forward_icon = zebra_ui::image::forward_icon().height(50).width(50).style(
            if self.approved && self.password == self.confirm_password {
                zebra_ui::style::svg::Svg::Primary
            } else {
                zebra_ui::style::svg::Svg::PrimaryDisabled
            },
        );
        let back_btn = Button::new(zebra_ui::image::back_icon().height(50).width(50))
            .padding(0)
            .style(zebra_ui::style::button::Button::Transparent)
            .on_press(PasswordSetupMessage::Back);
        let forward_btn = Button::new(forward_icon)
            .padding(0)
            .style(zebra_ui::style::button::Button::Transparent)
            .on_press_maybe(if self.approved && self.password == self.confirm_password {
                Some(PasswordSetupMessage::Next)
            } else {
                None
            });
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
        let title = Text::new(t!("zebra_futures"))
            .size(14)
            .width(Length::Fill)
            .horizontal_alignment(Horizontal::Left);
        let server_sync_check_box = Checkbox::new(
            t!("server_sync_check_box"),
            self.server_sync,
            PasswordSetupMessage::ApproveServerSync,
        );
        let server_sync_row = Row::new()
            .push(server_sync_check_box)
            .width(Length::Fill)
            .align_items(iced::Alignment::Start);
        let email_restore_check_box = Checkbox::new(
            t!("server_sync_check_box"),
            self.email_restore,
            PasswordSetupMessage::ApproveEmailRestore,
        );
        let email_restore_row = Row::new()
            .push(email_restore_check_box)
            .width(Length::Fill)
            .align_items(iced::Alignment::Start);
        let mut email_input = text_input(&t!("placeholder_email"), &self.email)
            .size(14)
            .width(250)
            .style(zebra_ui::style::text_input::TextInput::Primary);

        if self.email_restore {
            email_input = email_input.on_input(PasswordSetupMessage::OnEmailInputed);
        }

        let options_col = Column::new()
            .align_items(iced::Alignment::Center)
            .padding(10)
            .spacing(5)
            .height(Length::Fill)
            .width(Length::Fill)
            .push(title)
            .push(server_sync_row)
            .push(email_restore_row)
            .push(email_input);
        Container::new(options_col)
            .height(152)
            .width(350)
            .style(zebra_ui::style::container::Container::Bordered)
    }

    pub fn view_content<'a>(&self, m: &Mnemonic) -> Container<'a, PasswordSetupMessage> {
        let info = self.view_info();
        let error_msg = Text::new("error")
            .style(zebra_ui::style::text::Text::Dabger)
            .size(14);
        let passowrd = text_input(&t!("placeholder_password"), &self.password)
            .size(20)
            .width(250)
            .password()
            .on_input(PasswordSetupMessage::OnPasswordInputed)
            .style(zebra_ui::style::text_input::TextInput::Primary);
        let confirm_passowrd =
            text_input(&t!("placeholder_confirm_password"), &self.confirm_password)
                .size(20)
                .width(250)
                .password()
                .on_input(PasswordSetupMessage::OnConfirmPasswordInputed)
                .style(zebra_ui::style::text_input::TextInput::Primary);
        let in_col = Column::new()
            .spacing(5)
            .push(passowrd)
            .push(confirm_passowrd);
        let check_box = Checkbox::new(
            t!("accept_privacy_policy"),
            self.approved,
            PasswordSetupMessage::ApprovePolicy,
        );
        let chec_row = Row::new()
            .push(check_box)
            .width(250)
            .align_items(iced::Alignment::Start);
        let main_col = Column::new()
            .align_items(iced::Alignment::Center)
            .push(Space::new(0, 20))
            .push(info)
            .push(Space::new(0, 5))
            .push(error_msg)
            .push(Space::new(0, 5))
            .push(in_col)
            .push(Space::new(0, 10))
            .push(chec_row);

        Container::new(main_col)
    }
}
