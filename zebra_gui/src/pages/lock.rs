//! -- Copyright (c) 2023 Rina Khasanshin
//! -- Email: hicarus@yandex.ru
//! -- Licensed under the GNU General Public License Version 3.0 (GPL-3.0)

use std::sync::{Arc, Mutex};

use crate::{gui::Routers, rust_i18n::t};
use iced::{
    alignment::Horizontal,
    event,
    widget::{text_input, Space},
    Command, Event, Length, Subscription,
};
use zebra_lib::{core::core::Core, errors::ZebraErrors};
use zebra_ui::{components::circular, widget::*};

use crate::gui::GlobalMessage;

use super::{home::Home, options::Options, Page};

#[derive(Debug)]
pub struct Lock {
    core: Arc<Mutex<Core>>,
    password: String,
    show: bool,
    loading: bool,
    loaded: bool,
    err_message: String,
    input_id: text_input::Id,
}

#[derive(Debug, Clone)]
pub enum LockMessage {
    OnPasswordInput(String),
    TabPressed(bool),
    EventOccurred(Event),
    OnOptions,
    OnSubmit,
    OnFinishLoading(Result<(), ZebraErrors>),
}

pub async fn unlock(core: Arc<Mutex<Core>>, passowrd: String) -> Result<(), ZebraErrors> {
    let mut core = core.lock().or(Err(ZebraErrors::SyncStateLock))?;

    core.unlock(&passowrd)?;

    Ok(())
}

impl Page for Lock {
    type Message = LockMessage;

    fn new(core: Arc<Mutex<Core>>) -> Result<Self, ZebraErrors> {
        let password = String::new();
        let show = false;
        let loading = false;
        let loaded = false;
        let err_message = String::new();
        let input_id = text_input::Id::new("password_id");

        Ok(Self {
            core,
            err_message,
            input_id,
            loaded,
            loading,
            password,
            show,
        })
    }

    fn subscription(&self) -> Subscription<Self::Message> {
        Subscription::batch([
            event::listen().map(LockMessage::EventOccurred),
            iced::keyboard::on_key_press(|key_code, modifiers| match (key_code, modifiers) {
                (iced::keyboard::KeyCode::Tab, _) => {
                    Some(LockMessage::TabPressed(modifiers.shift()))
                }
                _ => None,
            }),
        ])
    }

    fn update(&mut self, message: Self::Message) -> iced::Command<GlobalMessage> {
        match message {
            LockMessage::OnSubmit => {
                if self.password.is_empty() {
                    return Command::none();
                }

                self.loading = true;

                Command::perform(unlock(Arc::clone(&self.core), self.password.clone()), |r| {
                    GlobalMessage::LockMessage(LockMessage::OnFinishLoading(r))
                })
            }
            LockMessage::OnOptions => {
                let options = Options::new(Arc::clone(&self.core)).unwrap();
                let route = Routers::Options(options);

                return Command::perform(std::future::ready(1), |_| GlobalMessage::Route(route));
            }
            LockMessage::OnPasswordInput(v) => {
                self.password = v;
                Command::none()
            }
            LockMessage::TabPressed(shift) => {
                if shift {
                    iced::widget::focus_previous()
                } else {
                    iced::widget::focus_next()
                }
            }
            LockMessage::EventOccurred(_) => {
                if !self.loaded {
                    self.loaded = true;
                    text_input::focus::<GlobalMessage>(self.input_id.clone())
                } else {
                    Command::none()
                }
            }
            LockMessage::OnFinishLoading(result) => match result {
                Ok(_) => {
                    let home = Home::new(Arc::clone(&self.core)).unwrap();
                    let route = Routers::Home(home);

                    return Command::perform(std::future::ready(1), |_| {
                        GlobalMessage::Route(route)
                    });
                }
                Err(e) => {
                    self.loading = false;
                    self.err_message = e.to_string();

                    text_input::focus::<GlobalMessage>(self.input_id.clone())
                }
            },
        }
    }

    fn view(&self) -> Element<Self::Message> {
        let zebra_print = zebra_ui::image::zebra_print_view();
        let error_message = Text::new(&self.err_message)
            .size(14)
            .style(zebra_ui::style::text::Text::Dabger)
            .horizontal_alignment(Horizontal::Center);
        let mut passowrd = text_input(&t!("placeholder_password"), &self.password)
            .size(16)
            .padding(8)
            .width(250)
            .id(self.input_id.clone())
            .style(zebra_ui::style::text_input::TextInput::Primary);

        if !self.loading {
            passowrd = passowrd
                .on_input(LockMessage::OnPasswordInput)
                .on_submit(LockMessage::OnSubmit);
        }
        if !self.show {
            passowrd = passowrd.password();
        }

        let submit_btn = Button::new(
            Text::new(t!("create_btn"))
                .horizontal_alignment(Horizontal::Center)
                .width(Length::Fill)
                .size(16),
        )
        .padding(8)
        .width(250)
        .height(38)
        .on_press(LockMessage::OnSubmit)
        .style(zebra_ui::style::button::Button::OutlinePrimary);
        let loading_btn = Button::new(
            Column::new()
                .push(circular::Circular::new().size(20.0))
                .width(Length::Fill)
                .align_items(iced::Alignment::Center),
        )
        .padding(8)
        .height(38)
        .width(250)
        .style(zebra_ui::style::button::Button::OutlinePrimary);
        let options_btn = Button::new(Text::new(t!("restore_or_create")).size(14))
            .on_press_maybe(if self.loading {
                None
            } else {
                Some(LockMessage::OnOptions)
            })
            .style(zebra_ui::style::button::Button::Ref);
        let options_col = Column::new()
            .width(250)
            .push(options_btn)
            .align_items(iced::Alignment::Start);
        let lock_icon = zebra_ui::image::lock_icon().width(100).height(100);
        let print_col = Column::new()
            .width(220)
            .height(Length::Fill)
            .push(zebra_print);
        let payload_col = Column::new()
            .width(Length::Fill)
            .height(Length::Fill)
            .align_items(iced::Alignment::Center)
            .padding(50)
            .push(lock_icon)
            .push(Space::new(0.0, 16.0))
            .push(error_message)
            .push(Space::new(0.0, 5.0))
            .push(passowrd)
            .push(Space::new(0.0, 5.0))
            .push(match self.loading {
                false => submit_btn,
                true => loading_btn,
            })
            .push(options_col);
        let row = Row::new()
            .width(Length::Fill)
            .push(print_col)
            .push(payload_col);

        Container::new(row)
            .height(Length::Fill)
            .width(Length::Fill)
            .into()
    }
}
