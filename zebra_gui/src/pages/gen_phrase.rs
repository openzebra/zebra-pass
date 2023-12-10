//! -- Copyright (c) 2023 Rina Khasanshin
//! -- Email: hicarus@yandex.ru
//! -- Licensed under the GNU General Public License Version 3.0 (GPL-3.0)
use iced::{Command, Length, Subscription};
use std::sync::{Arc, Mutex};
use zebra_lib::{bip39::mnemonic::Mnemonic, core::core::Core, errors::ZebraErrors};

use crate::gui::GlobalMessage;
use rand;

use super::Page;
use zebra_ui::widget::*;

#[derive(Debug)]
pub struct GenPhrase {
    pub words: Vec<String>,
    pub count: usize,
    pub error_msg: Option<String>,
    core: Arc<Mutex<Core>>,
}

#[derive(Debug)]
pub enum GenPhraseMessage {
    ReGenerate,
}

impl Page for GenPhrase {
    type Message = GenPhraseMessage;

    fn new(core: Arc<Mutex<Core>>) -> Result<Self, ZebraErrors> {
        let mut rng = rand::thread_rng();
        let count = 12; // number of words
        let m = Mnemonic::gen(
            &mut rng,
            count,
            zebra_lib::bip39::mnemonic::Language::English,
        )
        .or(Err(ZebraErrors::Bip39InvalidMnemonic))?;
        let words = m.get_vec().iter().map(|s| s.to_string()).collect();
        let error_msg = None;

        Ok(Self {
            core,
            words,
            error_msg,
            count,
        })
    }

    fn subscription(&self) -> iced::Subscription<Self::Message> {
        Subscription::none()
    }

    fn update(&mut self, message: Self::Message) -> Command<GlobalMessage> {
        match message {
            GenPhraseMessage::ReGenerate => {
                let mut rng = rand::thread_rng();
                let m = Mnemonic::gen(
                    &mut rng,
                    self.count,
                    zebra_lib::bip39::mnemonic::Language::English,
                );

                match m {
                    Ok(m) => {
                        self.words = m.get_vec().iter().map(|s| s.to_string()).collect();
                        Command::none()
                    }
                    Err(e) => {
                        self.error_msg = Some(e.to_string());
                        Command::none()
                    }
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
        let row = Row::new().width(Length::Fill).push(print_col);

        Container::new(row)
            .height(Length::Fill)
            .width(Length::Fill)
            .into()
    }
}
