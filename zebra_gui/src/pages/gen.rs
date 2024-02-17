//! -- Copyright (c) 2023 Rina Khasanshin
//! -- Email: hicarus@yandex.ru
//! -- Licensed under the GNU General Public License Version 3.0 (GPL-3.0)

use std::sync::{Arc, Mutex};

use crate::components::passgen::{PassGenForm, PassGenState};
use crate::components::phrasegen::{PhraseGenForm, PhraseGenState};
use crate::rust_i18n::t;
use iced::{clipboard, Command, Length, Subscription};
use zebra_lib::{core::core::Core, errors::ZebraErrors};
use zebra_ui::widget::*;

use crate::components::home_nav_bar::{NavBar, NavRoute};
use crate::gui::{GlobalMessage, Routers};

use super::home::Home;
use super::settings::Settings;
use super::Page;

const MAX_CHARS_SHOWN: u8 = 22;

#[derive(Debug)]
pub enum Tabs {
    Password,
    bip39,
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

                    return Command::perform(std::future::ready(1), |_| {
                        GlobalMessage::Route(route)
                    });
                }
                Err(e) => {
                    // TODO: make error page....
                    dbg!(e);
                    Command::none()
                }
            },
            GeneratorMessage::RouteSettings => {
                match Settings::new(Arc::clone(&self.core)) {
                    Ok(settings) => {
                        let route = Routers::Settings(settings);

                        return Command::perform(std::future::ready(1), |_| {
                            GlobalMessage::Route(route)
                        });
                    }
                    Err(e) => {
                        // TODO: make error page....
                        dbg!(e);
                        Command::none()
                    }
                }
            }
            GeneratorMessage::CopyWords => match self.phrase_state.lock() {
                Ok(state) => clipboard::write::<GlobalMessage>(state.words.join(" ")),
                Err(e) => {
                    dbg!("CopyWords", e);
                    Command::none()
                }
            },
            GeneratorMessage::CopyPassword => match self.pass_gen_state.lock() {
                Ok(state) => clipboard::write::<GlobalMessage>(state.value.to_owned()),
                Err(e) => {
                    dbg!("CopyPassword", e);
                    Command::none()
                }
            },
        }
    }

    fn view(&self) -> Element<Self::Message> {
        let content = match self.tab {
            Tabs::Password => self.view_password_gen(),
            Tabs::bip39 => self.view_phrase_gen(),
        };

        NavBar::<Self::Message>::new()
            .set_route(NavRoute::Gen)
            .on_home(GeneratorMessage::RouteHome)
            .on_settings(GeneratorMessage::RouteSettings)
            .view(content)
            .into()
    }
}

impl Generator {
    pub fn view_phrase_gen(&self) -> Container<GeneratorMessage> {
        match PhraseGenForm::new(Arc::clone(&self.phrase_state)) {
            Ok(elem) => Container::new(elem.set_on_copy(GeneratorMessage::CopyWords)),
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
            .size(14)
            .style(zebra_ui::style::text::Text::Dabger);

        Container::new(err_msg)
    }
}
