//! -- Copyright (c) 2024 Rina Khasanshin
//! -- Email: hicarus@yandex.ru
//! -- Licensed under the GNU General Public License Version 3.0 (GPL-3.0)

use crate::rust_i18n::t;
use iced::alignment::Horizontal;
use iced::widget::{component, pick_list, Component, Space};
use iced::{Alignment, Length};
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
    error_msg: Option<String>,
    msg: Option<Message>,
}

#[derive(Clone)]
pub enum Event {
    ReGenerate,
    Copy,
    CountSelected(usize),
    LanguageSelected(zebra_lib::bip39::mnemonic::Language),
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
            error_msg: None,
        })
    }

    pub fn view_words_row(&self) -> Column<'_, Event> {
        let words_row: Vec<Element<'_, Event>> = self
            .words
            .chunks(4)
            .map(|chunk| {
                let words_chunk: Vec<Element<'_, Event>> = chunk
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
            .height(220)
            .spacing(5)
            .align_items(Alignment::Center)
    }

    pub fn view_content(&self) -> Column<'_, Event> {
        let title = match &self.error_msg {
            Some(e) => Text::new(e.clone()),
            None => Text::new(t!("gen_page_title")),
        }
        .size(24);
        let count_pick_list = pick_list(
            self.counts.as_slice(),
            Some(self.count),
            Event::CountSelected,
        )
        .text_size(16)
        .padding(4)
        .width(80)
        .style(zebra_ui::style::pick_list::PickList::OutlineLight);
        let language_pick_list = pick_list(
            self.dicts.as_slice(),
            Some(self.dict),
            Event::LanguageSelected,
        )
        .text_size(16)
        .padding(4)
        .width(150)
        .style(zebra_ui::style::pick_list::PickList::OutlineLight);
        let reload_btn = Button::new(zebra_ui::image::reload_icon().height(30).width(30))
            .padding(0)
            .style(zebra_ui::style::button::Button::Transparent)
            .on_press(Event::ReGenerate);
        let copy_btn = Button::new(zebra_ui::image::copy_icon().height(30).width(30))
            .padding(0)
            .style(zebra_ui::style::button::Button::Transparent)
            .on_press(Event::Copy);
        let header_row = Row::new()
            .spacing(10)
            .push(reload_btn)
            .push(count_pick_list)
            .push(language_pick_list)
            .push(copy_btn);

        Column::new()
            .width(Length::Fill)
            .height(Length::Fill)
            .align_items(Alignment::Center)
            .padding(10)
            .push(title)
            .push(Space::new(0, 20))
            .push(header_row)
            .push(Space::new(0, 20))
            .push(self.view_words_row())
    }

    pub fn regenerate(&mut self) {
        let mut rng = rand::thread_rng(); // TODO: change to ChaCha
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
            Event::ReGenerate => None,
            Event::CountSelected(_count) => None,
            Event::LanguageSelected(_lang) => None,
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
