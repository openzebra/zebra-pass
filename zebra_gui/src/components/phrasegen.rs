//! -- Copyright (c) 2024 Rina Khasanshin
//! -- Email: hicarus@yandex.ru
//! -- Licensed under the GNU General Public License Version 3.0 (GPL-3.0)

use crate::rust_i18n::t;
use iced::widget::{component, Component};
use zebra_lib::bip39::mnemonic::Mnemonic;
use zebra_lib::errors::ZebraErrors;
use zebra_ui::style::Theme;
use zebra_ui::widget::*;

pub struct PhraseGenForm<Message>
where
    Message: Clone,
{
    words: Vec<String>,
    count: usize,
    counts: [usize; 5],
    dicts: [zebra_lib::bip39::mnemonic::Language; 1],
    dict: zebra_lib::bip39::mnemonic::Language,
    msg: Option<Message>,
}

#[derive(Clone)]
pub enum Event {
    Refresh,
    Copy,
}

impl<Message> PhraseGenForm<Message>
where
    Message: Clone,
{
    pub fn new(count: usize) -> Result<Self, ZebraErrors> {
        let mut rng = rand::thread_rng(); // TODO: change to ChaCha
        let dict = zebra_lib::bip39::mnemonic::Language::English;
        let m = Mnemonic::gen(&mut rng, count, dict.clone())
            .or(Err(ZebraErrors::Bip39InvalidMnemonic))?;
        let words = m.get_vec().iter().map(|s| s.to_string()).collect();
        let counts = [12, 15, 18, 21, 24];
        let dicts = [dict.clone()];

        Ok(Self {
            dicts,
            counts,
            words,
            count,
            dict,
            msg: None,
        })
    }
}

impl<Message> Component<Message, Theme, Renderer> for PhraseGenForm<Message>
where
    Message: Clone,
{
    type State = ();
    type Event = Event;

    fn update(&mut self, _state: &mut Self::State, event: Self::Event) -> Option<Message> {
        match event {
            Event::Copy => None,
            Event::Refresh => None,
        }
    }

    fn view(
        &self,
        _state: &Self::State,
    ) -> iced::advanced::graphics::core::Element<'_, Self::Event, Theme, Renderer> {
        let row = Row::new();

        Container::new(row).into()
    }
}

impl<'a, Message> From<PhraseGenForm<Message>> for Element<'a, Message>
where
    Message: 'a + Clone,
{
    fn from(form: PhraseGenForm<Message>) -> Self {
        component(form)
    }
}
