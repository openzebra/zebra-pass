//! -- Copyright (c) 2023 Rina Khasanshin
//! -- Email: hicarus@yandex.ru
//! -- Licensed under the GNU General Public License Version 3.0 (GPL-3.0)
use iced::widget::{pick_list, Checkbox, Space};
use iced::{alignment::Horizontal, clipboard, Alignment, Command, Length, Subscription};
use std::sync::{Arc, Mutex};
use zebra_lib::{bip39::mnemonic::Mnemonic, core::core::Core, errors::ZebraErrors};

use crate::components::phrasegen::PhraseGenForm;
use crate::gui::{GlobalMessage, Routers};
use crate::rust_i18n::t;

use super::options::Options;
use super::password_setup::{LastRoute, PasswordSetup};
use super::Page;
use zebra_ui::widget::*;

pub const COUNT: usize = 24;

#[derive(Debug)]
pub struct GenPhrase {
    pub words: Vec<String>,
    pub error_msg: Option<String>,
    is_checked: bool,
    dict: zebra_lib::bip39::mnemonic::Language,
    core: Arc<Mutex<Core>>,
}

#[derive(Debug, Clone)]
pub enum GenPhraseMessage {
    ApproveSeed(bool),
    CopyWords(Vec<String>),
    OnChange(Vec<String>),
    Back,
    Next,
}

impl Page for GenPhrase {
    type Message = GenPhraseMessage;

    fn new(core: Arc<Mutex<Core>>) -> Result<Self, ZebraErrors> {
        let dict = zebra_lib::bip39::mnemonic::Language::English;

        Ok(Self {
            core,
            dict,
            error_msg: None,
            words: Vec::new(),
            is_checked: false,
        })
    }

    fn subscription(&self) -> iced::Subscription<Self::Message> {
        Subscription::none()
    }

    fn update(&mut self, message: Self::Message) -> Command<GlobalMessage> {
        self.error_msg = None;

        match message {
            GenPhraseMessage::OnChange(v) => {
                self.words = v;
                Command::none()
            }
            GenPhraseMessage::ApproveSeed(v) => {
                self.is_checked = v;
                Command::none()
            }
            GenPhraseMessage::Back => {
                // TODO: remove unwrap!
                let options = Options::new(Arc::clone(&self.core)).unwrap();
                let route = Routers::Options(options);
                Command::perform(std::future::ready(1), |_| GlobalMessage::Route(route))
            }
            GenPhraseMessage::Next => {
                let words_str = self.words.join(" ");

                match Mnemonic::mnemonic_to_entropy(self.dict, &words_str) {
                    Ok(m) => {
                        let mut password_setup =
                            PasswordSetup::new(Arc::clone(&self.core)).unwrap();

                        password_setup.set_mnemonic(m);
                        password_setup.last_route = LastRoute::Gen;

                        let route = Routers::PasswordSetup(password_setup);
                        return Command::perform(std::future::ready(1), |_| {
                            GlobalMessage::Route(route)
                        });
                    }
                    Err(e) => {
                        self.error_msg = Some(t!("secret_phrase_invalid", code => e.to_string()));
                        return Command::none();
                    }
                }
            }
            GenPhraseMessage::CopyWords(v) => {
                let words = v.join(" ");
                clipboard::write::<GlobalMessage>(words)
            }
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
            .width(220)
            .height(Length::Fill)
            .push(zebra_print);
        let forward_icon =
            zebra_ui::image::forward_icon()
                .height(50)
                .width(50)
                .style(if self.is_checked {
                    zebra_ui::style::svg::Svg::Primary
                } else {
                    zebra_ui::style::svg::Svg::PrimaryDisabled
                });
        let forward_btn = Button::new(forward_icon)
            .padding(0)
            .style(zebra_ui::style::button::Button::Transparent)
            .on_press_maybe(match self.is_checked {
                true => Some(GenPhraseMessage::Next),
                false => None,
            });
        let back_btn = Button::new(zebra_ui::image::back_icon().height(50).width(50))
            .padding(0)
            .style(zebra_ui::style::button::Button::Transparent)
            .on_press(GenPhraseMessage::Back);
        let btns_row = Row::new().push(back_btn).push(forward_btn);
        let check_box = Checkbox::new(
            t!("approve_seed_remember"),
            self.is_checked,
            GenPhraseMessage::ApproveSeed,
        );
        let row_check_box = Row::new()
            .push(check_box)
            .align_items(Alignment::Start)
            .width(380);
        let phrase_gen_elem = PhraseGenForm::new(24)
            .unwrap()
            .set_on_change(GenPhraseMessage::OnChange);
        // .set_on_copy(GenPhraseMessage::CopyWords);
        let phrase_gen_warp = Container::new(phrase_gen_elem);
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
