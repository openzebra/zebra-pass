//! -- Copyright (c) 2023 Rina Khasanshin
//! -- Email: hicarus@yandex.ru
//! -- Licensed under the GNU General Public License Version 3.0 (GPL-3.0)
use iced::widget::{pick_list, Space};
use iced::{alignment::Horizontal, Alignment, Command, Length, Subscription};
use std::sync::{Arc, Mutex};
use zebra_lib::{bip39::mnemonic::Mnemonic, core::core::Core, errors::ZebraErrors};

use crate::gui::GlobalMessage;
use crate::rust_i18n::t;
use rand;

use super::Page;
use zebra_ui::widget::*;

#[derive(Debug)]
pub struct GenPhrase {
    pub words: Vec<String>,
    pub count: usize,
    pub counts: [usize; 5],
    pub error_msg: Option<String>,
    pub dicts: [zebra_lib::bip39::mnemonic::Language; 1],
    pub dict: zebra_lib::bip39::mnemonic::Language,
    core: Arc<Mutex<Core>>,
}

#[derive(Debug, Clone, Copy)]
pub enum GenPhraseMessage {
    ReGenerate,
    CountSelected(usize),
    LanguageSelected(zebra_lib::bip39::mnemonic::Language),
    Back,
    Next,
}

impl Page for GenPhrase {
    type Message = GenPhraseMessage;

    fn new(core: Arc<Mutex<Core>>) -> Result<Self, ZebraErrors> {
        let mut rng = rand::thread_rng();
        let count = 12; // number of words
        let dict = zebra_lib::bip39::mnemonic::Language::English;
        let m = Mnemonic::gen(&mut rng, count, dict.clone())
            .or(Err(ZebraErrors::Bip39InvalidMnemonic))?;
        let words = m.get_vec().iter().map(|s| s.to_string()).collect();
        let error_msg = None;
        let counts = [12, 15, 18, 21, 24];
        let dicts = [dict.clone()];

        Ok(Self {
            core,
            dicts,
            counts,
            words,
            error_msg,
            count,
            dict,
        })
    }

    fn subscription(&self) -> iced::Subscription<Self::Message> {
        Subscription::none()
    }

    fn update(&mut self, message: Self::Message) -> Command<GlobalMessage> {
        self.error_msg = None;

        match message {
            GenPhraseMessage::ReGenerate => {
                self.re_generate();
                Command::none()
            }
            GenPhraseMessage::CountSelected(count) => {
                self.count = count;
                self.re_generate();
                Command::none()
            }
            GenPhraseMessage::LanguageSelected(lang) => {
                self.dict = lang;
                self.re_generate();
                Command::none()
            }
            GenPhraseMessage::Back => Command::none(),
            GenPhraseMessage::Next => Command::none(),
        }
    }

    fn view(&self) -> Element<Self::Message> {
        let zebra_print = zebra_ui::image::zebra_print_view();
        let print_col = Column::new()
            .width(220)
            .height(Length::Fill)
            .push(zebra_print);
        let forward_btn = Button::new(zebra_ui::image::forward_icon().height(50).width(50))
            .padding(0)
            .style(zebra_ui::style::button::Button::Transparent)
            .on_press(GenPhraseMessage::Next);
        let back_btn = Button::new(zebra_ui::image::back_icon().height(50).width(50))
            .padding(0)
            .style(zebra_ui::style::button::Button::Transparent)
            .on_press(GenPhraseMessage::Back);
        let btns_row = Row::new().push(back_btn).push(forward_btn);
        let header_col = self.view_header().push(btns_row);

        // let main_col = Column::new().push(header_col).push(btns_row);
        let row = Row::new()
            .width(Length::Fill)
            .push(print_col)
            .push(header_col);

        Container::new(row)
            .height(Length::Fill)
            .width(Length::Fill)
            .into()
    }
}

impl GenPhrase {
    pub fn re_generate(&mut self) {
        let mut rng = rand::thread_rng();
        let m = Mnemonic::gen(
            &mut rng,
            self.count,
            zebra_lib::bip39::mnemonic::Language::English,
        );

        match m {
            Ok(m) => {
                self.words = m.get_vec().iter().map(|s| s.to_string()).collect();
            }
            Err(e) => {
                self.error_msg = Some(e.to_string());
            }
        }
    }

    pub fn view_words_row(&self) -> Column<'_, GenPhraseMessage> {
        let words_row: Vec<Element<'_, GenPhraseMessage>> = self
            .words
            .chunks(4)
            .map(|chunk| {
                let words_chunk: Vec<Element<'_, GenPhraseMessage>> = chunk
                    .iter()
                    .map(|w| {
                        Button::new(
                            Text::new(w)
                                .size(14)
                                .horizontal_alignment(Horizontal::Center),
                        )
                        .style(zebra_ui::style::button::Button::Primary)
                        .width(90)
                        .height(30)
                        .into()
                    })
                    .collect();
                Row::with_children(words_chunk)
                    .spacing(5)
                    .align_items(Alignment::Start)
                    .into()
            })
            .collect();

        Column::with_children(words_row)
            .width(400)
            .height(220)
            .spacing(5)
            .align_items(Alignment::Center)
    }

    pub fn view_header(&self) -> Column<'_, GenPhraseMessage> {
        let title = match &self.error_msg {
            Some(e) => Text::new(e.clone()),
            None => Text::new(t!("gen_page_title")),
        }
        .size(20);
        let count_pick_list = pick_list(
            self.counts.as_slice(),
            Some(self.count),
            GenPhraseMessage::CountSelected,
        )
        .text_size(16)
        .padding(4)
        .width(80)
        .style(zebra_ui::style::pick_list::PickList::OutlineLight);
        let language_pick_list = pick_list(
            self.dicts.as_slice(),
            Some(self.dict),
            GenPhraseMessage::LanguageSelected,
        )
        .text_size(16)
        .padding(4)
        .width(150)
        .style(zebra_ui::style::pick_list::PickList::OutlineLight);
        let reload_btn = Button::new(zebra_ui::image::reload_icon().height(30).width(30))
            .padding(0)
            .style(zebra_ui::style::button::Button::Transparent)
            .on_press(GenPhraseMessage::ReGenerate);
        let header_row = Row::new()
            .push(reload_btn)
            .push(Space::new(10, 0))
            .push(count_pick_list)
            .push(Space::new(10, 0))
            .push(language_pick_list);

        Column::new()
            .width(Length::Fill)
            .height(Length::Fill)
            .align_items(iced::Alignment::Center)
            .padding(10)
            .push(title)
            .push(Space::new(0, 20))
            .push(header_row)
            .push(Space::new(0, 20))
            .push(self.view_words_row())
    }
}
