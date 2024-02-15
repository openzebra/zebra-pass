//! -- Copyright (c) 2024 Rina Khasanshin
//! -- Email: hicarus@yandex.ru
//! -- Licensed under the GNU General Public License Version 3.0 (GPL-3.0)

use crate::rust_i18n::t;
use iced::widget::{component, slider, text_input, Checkbox, Component};
use iced::Length;
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
        let error_msg = None;
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
            Event::Copy => self.copy_msg.clone(),
            Event::Refresh => {
                self.regenerate();
                self.change_msg.clone()
            }
        }
    }

    fn view(
        &self,
        _state: &Self::State,
    ) -> iced::advanced::graphics::core::Element<'_, Self::Event, Theme, Renderer> {
        let col_pass_box = Column::new()
            .push(self.view_generator())
            .align_items(iced::Alignment::Center);
        let col_slider_box = Column::new()
            .push(self.view_slider())
            .align_items(iced::Alignment::Center);
        let col_opt_box = Column::new()
            .push(self.view_gen_options())
            .align_items(iced::Alignment::Center);
        let col = Column::new()
            .push(col_pass_box)
            .push(col_slider_box)
            .push(col_opt_box)
            .align_items(iced::Alignment::Center)
            .spacing(8)
            .width(Length::Fill);
        let row = Row::new()
            .push(col)
            .height(300)
            .align_items(iced::Alignment::Center);

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
