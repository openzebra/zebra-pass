//! -- Copyright (c) 2023 Rina Khasanshin
//! -- Email: hicarus@yandex.ru
//! -- Licensed under the GNU General Public License Version 3.0 (GPL-3.0)

use super::{error::ErrorPage, gen_phrase::GenPhrase, home::Home, restore::Restore, Page};
use crate::{
    gui::{GlobalMessage, Routers},
    rust_i18n::t,
};
use iced::widget::{self, Checkbox};
use iced::widget::{text_input, Button, Column, Container, Row, Space, Text};
use iced::{alignment::Horizontal, Command, Length, Subscription};
use iced::{
    keyboard::{self, key::Named},
    Element,
};
use std::sync::{Arc, Mutex};
use zebra_lib::{
    bip39::mnemonic::Mnemonic,
    core::{
        core::Core,
        email::is_valid_email,
        password_strength::{password_strength, MIN_PASSWORD_SIZE},
    },
    errors::ZebraErrors,
};

#[derive(Debug)]
pub enum LastRoute {
    Gen,
    Restore,
}

#[derive(Debug)]
pub struct PasswordSetup {
    pub last_route: LastRoute,
    error_msg: String,
    salt: String,
    password: String,
    confirm_password: String,
    email: String,
    approved: bool,
    server_sync: bool,
    email_restore: bool,
    enabled_salt: bool,
    loading: bool,
    core: Arc<Mutex<Core>>,
    mnemonic: Option<Arc<Mnemonic>>,
}

#[derive(Debug, Clone)]
pub enum PasswordSetupMessage {
    Next,
    Back,
    ApprovePolicy(bool),
    ApproveServerSync(bool),
    ApproveEmailRestore(bool),
    EnableSalt(bool),
    OnPasswordInputed(String),
    OnConfirmPasswordInputed(String),
    OnEmailInputed(String),
    OnSaltInput(String),
    TabPressed(bool),
    SetupFinish(Result<(), String>),
}

pub async fn setup_password(
    core: Arc<Mutex<Core>>,
    m: Arc<Mnemonic>,
    server_sync: bool,
    email: String,
    password: String,
    salt: String,
) -> Result<(), String> {
    let mut core = core.lock().or(Err(t!("thread_sync_error")))?;
    match core.init_data(server_sync, &email, &password, &salt, &m) {
        Ok(_) => {}
        Err(e) => return Err(e.to_string()),
    };
    Ok(())
}

impl Page for PasswordSetup {
    type Message = PasswordSetupMessage;

    fn new(core: Arc<Mutex<Core>>) -> Result<Self, ZebraErrors> {
        let mnemonic = None;
        let last_route = LastRoute::Gen;
        let password = String::new();
        let salt = String::new();
        let confirm_password = String::new();
        let approved = false;
        let server_sync = true;
        let email_restore = true;
        let email = String::new();
        let enabled_salt = false;
        let loading = false;
        let error_msg = String::new();

        Ok(Self {
            email,
            loading,
            error_msg,
            enabled_salt,
            salt,
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
        keyboard::on_key_press(|key_code, modifiers| match (key_code, modifiers) {
            (keyboard::Key::Named(Named::Tab), _) => {
                Some(PasswordSetupMessage::TabPressed(modifiers.shift()))
            }
            _ => None,
        })
    }

    fn update(&mut self, message: Self::Message) -> Command<GlobalMessage> {
        match message {
            PasswordSetupMessage::SetupFinish(result) => match result {
                Ok(_) => match Home::new(Arc::clone(&self.core)) {
                    Ok(home) => {
                        let route = Routers::Home(home);

                        Command::perform(std::future::ready(1), |_| GlobalMessage::Route(route))
                    }
                    Err(e) => {
                        let route = Routers::ErrorPage(ErrorPage::from(e.to_string()));

                        Command::perform(std::future::ready(1), |_| GlobalMessage::Route(route))
                    }
                },
                Err(e) => {
                    self.error_msg = e.to_string();
                    self.loading = false;

                    Command::none()
                }
            },
            PasswordSetupMessage::Next => {
                self.error_msg = String::new();
                if !self.approved
                    || self.password != self.confirm_password
                    || self.password.is_empty()
                    || self.confirm_password.is_empty()
                {
                    return Command::none();
                }

                if self.email_restore {
                    if self.email.is_empty() {
                        self.error_msg = t!("empty_email");

                        return Command::none();
                    }

                    match is_valid_email(&self.email) {
                        Ok(v) => {
                            if !v {
                                self.error_msg = t!("invalid_email");

                                return Command::none();
                            }
                        }
                        Err(e) => {
                            self.error_msg = e.to_string();

                            return Command::none();
                        }
                    }
                }

                let m_ref = match &self.mnemonic {
                    Some(m) => Arc::clone(m),
                    None => {
                        self.error_msg = t!("mnemonic_is_not_inited");

                        return Command::none();
                    }
                };
                let strength = match password_strength(&self.password) {
                    Ok(s) => s,
                    Err(e) => {
                        self.error_msg = t!(&e.to_string());

                        return Command::none();
                    }
                };

                if MIN_PASSWORD_SIZE > strength as usize {
                    self.error_msg = t!("week_password_len");

                    return Command::none();
                }

                self.loading = true;

                Command::perform(
                    setup_password(
                        Arc::clone(&self.core),
                        m_ref,
                        self.server_sync,
                        // TODO: maybe need optimze it
                        self.email.clone(),
                        self.password.clone(),
                        self.salt.clone(),
                    ),
                    |r| GlobalMessage::PasswordSetupMessage(PasswordSetupMessage::SetupFinish(r)),
                )
            }
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

                Command::perform(std::future::ready(1), |_| GlobalMessage::Route(route))
            }
            PasswordSetupMessage::ApprovePolicy(v) => {
                if !self.loading {
                    self.approved = v;
                    self.error_msg = String::new();
                }
                Command::none()
            }
            PasswordSetupMessage::OnPasswordInputed(v) => {
                self.error_msg = String::new();
                self.password = v;
                Command::none()
            }
            PasswordSetupMessage::OnConfirmPasswordInputed(v) => {
                self.error_msg = String::new();
                self.confirm_password = v;
                Command::none()
            }
            PasswordSetupMessage::OnEmailInputed(v) => {
                self.error_msg = String::new();
                self.email = v;
                Command::none()
            }
            PasswordSetupMessage::OnSaltInput(v) => {
                self.error_msg = String::new();
                self.salt = v;
                Command::none()
            }
            PasswordSetupMessage::ApproveServerSync(v) => {
                self.error_msg = String::new();
                if !self.loading {
                    self.server_sync = v;
                    self.email_restore = v;
                }
                Command::none()
            }
            PasswordSetupMessage::EnableSalt(v) => {
                self.error_msg = String::new();
                if !self.loading {
                    self.enabled_salt = v;
                    if !v {
                        self.salt = String::new();
                    }
                }
                Command::none()
            }
            PasswordSetupMessage::ApproveEmailRestore(v) => {
                self.error_msg = String::new();
                if !self.loading {
                    self.email_restore = v;
                }

                Command::none()
            }
            PasswordSetupMessage::TabPressed(shift) => {
                if shift {
                    widget::focus_previous()
                } else {
                    widget::focus_next()
                }
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
        let forward_icon = zebra_ui::image::forward_icon().height(50).width(50);

        //     .style(
        //     if self.approved
        //         && self.password == self.confirm_password
        //         && !self.password.is_empty()
        //         && !self.confirm_password.is_empty()
        //     {
        //         zebra_ui::style::svg::Svg::Primary
        //     } else {
        //         zebra_ui::style::svg::Svg::PrimaryDisabled
        //     },
        // );
        let back_btn = Button::new(zebra_ui::image::back_icon().height(50).width(50))
            .padding(0)
            // .style(zebra_ui::style::button::Button::Transparent)
            .on_press(PasswordSetupMessage::Back);
        let forward_btn = Button::new(forward_icon)
            .padding(0)
            // .style(zebra_ui::style::button::Button::Transparent)
            .on_press_maybe(
                if self.approved
                    && self.password == self.confirm_password
                    && !self.password.is_empty()
                    && !self.confirm_password.is_empty()
                {
                    Some(PasswordSetupMessage::Next)
                } else {
                    None
                },
            );
        let btns_row = Row::new().push(back_btn).push(forward_btn);
        let load_row = Row::new()
            .padding(5)
            .push(zebra_ui::components::circular::Circular::new().size(30.0));
        let content_col = Column::new()
            .height(Length::Fill)
            .width(Length::Fill)
            .align_items(iced::Alignment::Center)
            .push(title)
            .push(match &self.mnemonic {
                Some(_m) => self.view_content(),
                None => self.view_error(),
            })
            .push(match self.loading {
                false => btns_row,
                true => load_row,
            });
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
        self.mnemonic = Some(Arc::new(m));
    }

    pub fn view_error<'a>(&self) -> Container<'a, PasswordSetupMessage> {
        let error_message = Text::new(t!("mnemonic_is_not_inited"))
            .size(16)
            // .style(zebra_ui::style::text::Text::Dabger)
            .horizontal_alignment(Horizontal::Center);

        Container::new(Column::new().push(error_message))
    }

    pub fn view_info<'a>(&self) -> Container<'a, PasswordSetupMessage> {
        let server_sync_check_box = Checkbox::new(t!("server_sync_check_box"), self.server_sync)
            .on_toggle(PasswordSetupMessage::ApproveServerSync)
            .text_size(14);
        let server_sync_row = Row::new()
            .push(server_sync_check_box)
            .width(Length::Fill)
            .align_items(iced::Alignment::Start);
        let email_restore_check_box =
            Checkbox::new(t!("email_restore_checkbox"), self.email_restore)
                .on_toggle(PasswordSetupMessage::ApproveEmailRestore)
                .text_size(14);
        let phrase_salt_check_box = Checkbox::new(t!("secret_phrase_salt"), self.enabled_salt)
            .on_toggle(PasswordSetupMessage::EnableSalt)
            .text_size(14);
        let email_restore_row = Row::new()
            .push(email_restore_check_box)
            .width(Length::Fill)
            .align_items(iced::Alignment::Start);
        let mut email_input = text_input(&t!("placeholder_email"), &self.email)
            .size(14)
            // .style(zebra_ui::style::text_input::TextInput::Primary)
            .width(Length::Fill);
        let mut salt_input = text_input(&t!("placeholder_salt"), &self.salt)
            .size(14)
            // .style(zebra_ui::style::text_input::TextInput::Primary)
            .width(Length::Fill);

        if self.enabled_salt && !self.loading {
            salt_input = salt_input.on_input(PasswordSetupMessage::OnSaltInput);
        }

        if self.email_restore && !self.loading {
            email_input = email_input.on_input(PasswordSetupMessage::OnEmailInputed);
        }

        let salt_row = Row::new()
            .push(phrase_salt_check_box)
            .width(Length::Fill)
            .align_items(iced::Alignment::Start);

        let options_col = Column::new()
            .align_items(iced::Alignment::Center)
            .padding(10)
            .spacing(5)
            .height(Length::Fill)
            .width(Length::Fill)
            .push(server_sync_row)
            .push(email_restore_row)
            .push(email_input)
            .push(salt_row)
            .push(salt_input);
        Container::new(options_col).height(160).width(320)
        // .style(zebra_ui::style::container::Container::Bordered)
    }

    pub fn view_content(&self) -> Container<'_, PasswordSetupMessage> {
        let info = self.view_info();
        let error_msg = Text::new(&self.error_msg)
            // .style(zebra_ui::style::text::Text::Dabger)
            .size(14);
        let mut passowrd = text_input(&t!("placeholder_password"), &self.password)
            .size(16)
            .width(250)
            .padding(8)
            // .style(zebra_ui::style::text_input::TextInput::Primary)
            .secure(true);
        let mut confirm_passowrd =
            text_input(&t!("placeholder_confirm_password"), &self.confirm_password)
                .size(16)
                .padding(8)
                .width(250)
                // .style(zebra_ui::style::text_input::TextInput::Primary)
                .secure(true);

        if !self.loading {
            passowrd = passowrd
                .on_submit(PasswordSetupMessage::Next)
                .on_input(PasswordSetupMessage::OnPasswordInputed);

            confirm_passowrd = confirm_passowrd
                .on_input(PasswordSetupMessage::OnConfirmPasswordInputed)
                .on_submit(PasswordSetupMessage::Next);
        }

        let in_col = Column::new()
            .spacing(5)
            .push(passowrd)
            .push(confirm_passowrd);
        let check_box = Checkbox::new(t!("accept_privacy_policy"), self.approved)
            .on_toggle(PasswordSetupMessage::ApprovePolicy)
            .text_size(11);
        let chec_row = Row::new()
            .push(check_box)
            .width(250)
            .align_items(iced::Alignment::Start);
        let main_col = Column::new()
            .align_items(iced::Alignment::Center)
            .push(Space::new(0, 5))
            .push(info)
            .push(Space::new(0, 5))
            .push(error_msg)
            .push(Space::new(0, 5))
            .push(in_col)
            .push(Space::new(0, 5))
            .push(chec_row);

        Container::new(main_col)
    }
}
