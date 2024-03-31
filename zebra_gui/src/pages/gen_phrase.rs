//! -- Copyright (c) 2023 Rina Khasanshin
//! -- Email: hicarus@yandex.ru
//! -- Licensed under the GNU General Public License Version 3.0 (GPL-3.0)
use iced::widget::{Button, Checkbox, Column, Container, Row, Text};
use iced::{Alignment, Command, Element, Length, Subscription};
use std::sync::{Arc, Mutex};
use zebra_lib::{bip39::mnemonic::Mnemonic, core::core::Core, errors::ZebraErrors};
use zebra_ui::config::PRINT_WIDTH;

use crate::components::phrasegen::{PhraseGenForm, PhraseGenState};
use crate::gui::{GlobalMessage, Routers};
use crate::rust_i18n::t;

use super::error::ErrorPage;
use super::options::Options;
use super::password_setup::{LastRoute, PasswordSetup};
use super::Page;

#[derive(Debug)]
pub struct GenPhrase {
    pub error_msg: Option<String>,
    is_checked: bool,
    core: Arc<Mutex<Core>>,
    phrase_state: Arc<Mutex<PhraseGenState>>,
}

#[derive(Debug, Clone)]
pub enum GenPhraseMessage {
    ApproveSeed(bool),
    CopyWords,
    Back,
    Next,
}

impl Page for GenPhrase {
    type Message = GenPhraseMessage;

    fn new(core: Arc<Mutex<Core>>) -> Result<Self, ZebraErrors> {
        let phrase_state = Arc::new(Mutex::new(PhraseGenState::default()));

        Ok(Self {
            core,
            phrase_state,
            error_msg: None,
            is_checked: false,
        })
    }

    fn subscription(&self) -> iced::Subscription<Self::Message> {
        Subscription::none()
    }

    fn update(&mut self, message: Self::Message) -> Command<GlobalMessage> {
        self.error_msg = None;

        match message {
            GenPhraseMessage::ApproveSeed(v) => {
                self.is_checked = v;
                Command::none()
            }
            GenPhraseMessage::Back => match Options::new(Arc::clone(&self.core)) {
                Ok(options) => {
                    let route = Routers::Options(options);
                    Command::perform(std::future::ready(1), |_| GlobalMessage::Route(route))
                }
                Err(e) => {
                    let route = Routers::ErrorPage(ErrorPage::from(e.to_string()));
                    Command::perform(std::future::ready(1), |_| GlobalMessage::Route(route))
                }
            },
            GenPhraseMessage::Next => {
                let locked_state = match self.phrase_state.lock() {
                    Ok(state) => state,
                    Err(e) => {
                        self.error_msg = Some(t!("secret_phrase_invalid", code => e.to_string()));
                        return Command::none();
                    }
                };
                let words_str = locked_state.words.join(" ");

                match Mnemonic::mnemonic_to_entropy(locked_state.dict, &words_str) {
                    Ok(m) => {
                        let mut password_setup =
                            PasswordSetup::new(Arc::clone(&self.core)).unwrap();

                        password_setup.set_mnemonic(m);
                        password_setup.last_route = LastRoute::Gen;

                        let route = Routers::PasswordSetup(password_setup);
                        Command::perform(std::future::ready(1), |_| GlobalMessage::Route(route))
                    }
                    Err(e) => {
                        self.error_msg = Some(t!("secret_phrase_invalid", code => e.to_string()));
                        Command::none()
                    }
                }
            }
            GenPhraseMessage::CopyWords => match self.phrase_state.lock() {
                Ok(state) => {
                    let words = state.words.join(" ");
                    iced::clipboard::write::<GlobalMessage>(words)
                }
                Err(e) => {
                    self.error_msg = Some(e.to_string());
                    Command::none()
                }
            },
        }
    }

    fn view(&self) -> Element<Self::Message> {
        let title = match &self.error_msg {
            Some(e) => Text::new(e.clone()),
            None => Text::new(t!("gen_page_title")),
        }
        .size(24);
        let zebra_print = zebra_ui::image::zebra_print_view();
        let print_col = Column::new()
            .width(PRINT_WIDTH)
            .height(Length::Fill)
            .push(zebra_print);
        let forward_icon = zebra_ui::image::forward_icon()
            .height(50)
            .style(if self.is_checked {
                zebra_ui::styles::svg::primary_hover
            } else {
                zebra_ui::styles::svg::primary_disabled
            })
            .width(50);
        let forward_btn = Button::new(forward_icon)
            .padding(0)
            .style(zebra_ui::styles::button::transparent)
            .on_press_maybe(match self.is_checked {
                true => Some(GenPhraseMessage::Next),
                false => None,
            });
        let back_btn = Button::new(zebra_ui::image::back_icon().height(50).width(50))
            .padding(0)
            .style(zebra_ui::styles::button::transparent)
            .on_press(GenPhraseMessage::Back);
        let btns_row = Row::new().push(back_btn).push(forward_btn);
        let check_box = Checkbox::new(t!("approve_seed_remember"), self.is_checked)
            .on_toggle(GenPhraseMessage::ApproveSeed);
        let row_check_box = Row::new()
            .push(check_box)
            .align_items(Alignment::Start)
            .width(380);
        let phrase_gen_warp = match PhraseGenForm::new(Arc::clone(&self.phrase_state)) {
            Ok(elem) => Container::new(elem.set_on_copy(GenPhraseMessage::CopyWords)),
            Err(e) => {
                let err_msg = Text::new(e.to_string())
                    .style(zebra_ui::styles::text::danger)
                    .size(14);

                Container::new(err_msg)
            }
        };
        let phrase_gen_col = Row::new()
            .width(Length::Fill)
            .align_items(Alignment::Center)
            .push(phrase_gen_warp);
        let content_col = Column::new()
            .align_items(Alignment::Center)
            .height(Length::Fill)
            .push(title)
            .push(phrase_gen_col)
            .push(row_check_box)
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
