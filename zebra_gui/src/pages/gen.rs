//! -- Copyright (c) 2023 Rina Khasanshin
//! -- Email: hicarus@yandex.ru
//! -- Licensed under the GNU General Public License Version 3.0 (GPL-3.0)

use std::sync::{Arc, Mutex};

use crate::components::passgen::{PassGenForm, PassGenState};
use crate::components::phrasegen::{PhraseGenForm, PhraseGenState};
use crate::pages::error::ErrorPage;
use crate::rust_i18n::t;
use iced::alignment::Horizontal;
use iced::widget::{Button, Column, Container, Row, Space, Text};
use iced::{clipboard, Command, Element, Length, Subscription};
use zebra_lib::{core::Core, errors::ZebraErrors};

use crate::components::home_nav_bar::{NavBar, NavRoute};
use crate::gui::{GlobalMessage, Routers};

use super::add_record::AddRecordPage;
use super::home::Home;
use super::settings::Settings;
use super::Page;

const MAX_CHARS_SHOWN: u8 = 22;

#[derive(Debug)]
pub enum Tabs {
    Password,
    Bip39,
}

#[derive(Debug)]
pub struct Generator {
    core: Arc<Mutex<Core>>,
    pass_gen_state: Arc<Mutex<PassGenState>>,
    phrase_state: Arc<Mutex<PhraseGenState>>,
    tab: Tabs,
}

#[derive(Debug, Clone)]
pub enum GeneratorMessage {
    RouteHome,
    RouteSettings,
    AddRecord,
    PasswordTab,
    PhraseTab,
    CopyPassword,
    CopyWords,
}

impl Page for Generator {
    type Message = GeneratorMessage;

    fn new(core: Arc<Mutex<Core>>) -> Result<Self, ZebraErrors> {
        let tab = Tabs::Password;
        let pass_gen_state = Arc::new(Mutex::new(PassGenState {
            value: String::new(),
            length: MAX_CHARS_SHOWN,
        }));
        let phrase_state = Arc::new(Mutex::new(PhraseGenState::default()));

        Ok(Self {
            core,
            tab,
            pass_gen_state,
            phrase_state,
        })
    }

    fn subscription(&self) -> Subscription<Self::Message> {
        Subscription::none()
    }

    fn update(&mut self, message: Self::Message) -> iced::Command<GlobalMessage> {
        match message {
            GeneratorMessage::RouteHome => match Home::new(Arc::clone(&self.core)) {
                Ok(home) => {
                    let route = Routers::Home(home);

                    Command::perform(std::future::ready(1), |_| GlobalMessage::Route(route))
                }
                Err(e) => {
                    let route = Routers::ErrorPage(ErrorPage::from(e.to_string()));

                    Command::perform(std::future::ready(1), |_| GlobalMessage::Route(route))
                }
            },
            GeneratorMessage::RouteSettings => match Settings::new(Arc::clone(&self.core)) {
                Ok(settings) => {
                    let route = Routers::Settings(settings);

                    Command::perform(std::future::ready(1), |_| GlobalMessage::Route(route))
                }
                Err(e) => {
                    let route = Routers::ErrorPage(ErrorPage::from(e.to_string()));

                    Command::perform(std::future::ready(1), |_| GlobalMessage::Route(route))
                }
            },
            GeneratorMessage::AddRecord => match AddRecordPage::new(Arc::clone(&self.core)) {
                Ok(add_record) => {
                    let route = Routers::AddRecord(add_record);

                    Command::perform(std::future::ready(1), |_| GlobalMessage::Route(route))
                }
                Err(e) => {
                    let route = Routers::ErrorPage(ErrorPage::from(e.to_string()));

                    Command::perform(std::future::ready(1), |_| GlobalMessage::Route(route))
                }
            },
            GeneratorMessage::CopyWords => match self.phrase_state.lock() {
                Ok(state) => clipboard::write::<GlobalMessage>(state.words.join(" ")),
                Err(e) => {
                    let route = Routers::ErrorPage(ErrorPage::from(e.to_string()));

                    Command::perform(std::future::ready(1), |_| GlobalMessage::Route(route))
                }
            },
            GeneratorMessage::CopyPassword => match self.pass_gen_state.lock() {
                Ok(state) => clipboard::write::<GlobalMessage>(state.value.to_owned()),
                Err(e) => {
                    let route = Routers::ErrorPage(ErrorPage::from(e.to_string()));

                    Command::perform(std::future::ready(1), |_| GlobalMessage::Route(route))
                }
            },
            GeneratorMessage::PasswordTab => {
                self.tab = Tabs::Password;
                Command::none()
            }
            GeneratorMessage::PhraseTab => {
                self.tab = Tabs::Bip39;
                Command::none()
            }
        }
    }

    fn view(&self) -> Element<Self::Message> {
        let password_btn = Button::new(
            Text::new(t!("password_gen"))
                .horizontal_alignment(Horizontal::Center)
                .width(Length::Fill)
                .size(16),
        )
        .padding(8)
        .width(200)
        .on_press_maybe(match self.tab {
            Tabs::Password => None,
            Tabs::Bip39 => Some(GeneratorMessage::PasswordTab),
        })
        .style(match self.tab {
            Tabs::Password => zebra_ui::styles::button::primary,
            Tabs::Bip39 => zebra_ui::styles::button::outline_primary,
        });
        let bip39_btn = Button::new(
            Text::new(t!("bip39_gen"))
                .horizontal_alignment(Horizontal::Center)
                .width(Length::Fill)
                .size(16),
        )
        .padding(8)
        .width(200)
        .style(match self.tab {
            Tabs::Bip39 => zebra_ui::styles::button::primary,
            Tabs::Password => zebra_ui::styles::button::outline_primary,
        })
        .on_press_maybe(match self.tab {
            Tabs::Bip39 => None,
            Tabs::Password => Some(GeneratorMessage::PhraseTab),
        });
        let row_btns = Row::new().spacing(5).push(password_btn).push(bip39_btn);
        let content = match self.tab {
            Tabs::Password => self.view_password_gen(),
            Tabs::Bip39 => self.view_phrase_gen(),
        };
        let main_col = Column::new()
            .align_items(iced::Alignment::Center)
            .push(Space::new(0, 20))
            .push(row_btns)
            .push(content);
        let container = Container::new(main_col);

        NavBar::<Self::Message>::new()
            .set_route(NavRoute::Gen)
            .on_home(&GeneratorMessage::RouteHome)
            .on_settings(&GeneratorMessage::RouteSettings)
            .on_add(&GeneratorMessage::AddRecord)
            .view(container)
            .into()
    }
}

impl Generator {
    pub fn view_phrase_gen(&self) -> Container<GeneratorMessage> {
        match PhraseGenForm::new(Arc::clone(&self.phrase_state)) {
            Ok(elem) => Container::new(elem.set_on_copy(&GeneratorMessage::CopyWords))
                .width(Length::Fill)
                .height(Length::Fill),

            Err(e) => self.view_error(e.to_string()),
        }
    }

    pub fn view_password_gen(&self) -> Container<GeneratorMessage> {
        match PassGenForm::new(Arc::clone(&self.pass_gen_state)) {
            Ok(ctx) => Container::new(ctx.set_copy_message(GeneratorMessage::CopyPassword))
                .width(Length::Fill)
                .height(Length::Fill),
            Err(e) => self.view_error(e.to_string()),
        }
    }

    pub fn view_error(&self, error: String) -> Container<GeneratorMessage> {
        let err_msg = Text::new(error)
            .style(zebra_ui::styles::text::danger)
            .size(14);

        Container::new(err_msg)
    }
}
