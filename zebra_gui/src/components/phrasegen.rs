//! -- Copyright (c) 2024 Rina Khasanshin
//! -- Email: hicarus@yandex.ru
//! -- Licensed under the GNU General Public License Version 3.0 (GPL-3.0)

use iced::alignment::Horizontal;
use iced::widget::{component, pick_list, Button, Column, Component, Container, Row, Space, Text};
use iced::{Alignment, Element, Length};
use iced::{Renderer, Theme};
use std::sync::{Arc, Mutex};
use zebra_lib::bip39::config::MAX_NB_WORDS;
use zebra_lib::bip39::mnemonic;
use zebra_lib::bip39::mnemonic::Mnemonic;
use zebra_lib::errors::ZebraErrors;

#[derive(Debug)]
pub struct PhraseGenState {
    pub dict: mnemonic::Language,
    pub words: Vec<String>,
    pub count: usize,
}

impl Default for PhraseGenState {
    fn default() -> Self {
        Self {
            count: MAX_NB_WORDS,
            dict: mnemonic::Language::English,
            words: Vec::with_capacity(MAX_NB_WORDS),
        }
    }
}

#[derive(Debug)]
pub struct PhraseGenForm<'a, Message>
where
    Message: Clone,
{
    state: Arc<Mutex<PhraseGenState>>,
    counts: [usize; 5],
    dicts: [mnemonic::Language; 1],
    on_copy: Option<&'a Message>,
}

#[derive(Clone)]
pub enum Event {
    ReGenerate,
    Copy,
    CountSelected(usize),
    LanguageSelected(mnemonic::Language),
}

impl<'a, Message> PhraseGenForm<'a, Message>
where
    Message: Clone,
{
    pub fn new(state: Arc<Mutex<PhraseGenState>>) -> Result<Self, ZebraErrors> {
        let counts = [12, 15, 18, 21, 24];
        let dicts = [zebra_lib::bip39::mnemonic::Language::English];
        let mut locked_state = state.lock().or(Err(ZebraErrors::SyncStateLock))?;
        let mut rng = rand::thread_rng(); // TODO: change to ChaCha
        let m = Mnemonic::gen(&mut rng, locked_state.count, locked_state.dict)
            .or(Err(ZebraErrors::Bip39InvalidMnemonic))?;

        if locked_state.words.is_empty() {
            locked_state.words = m.get_vec().iter().map(|s| s.to_string()).collect();
        }

        drop(locked_state);

        Ok(Self {
            state,
            dicts,
            counts,
            on_copy: None,
        })
    }

    pub fn set_on_copy(mut self, on_copy: &'a Message) -> Self {
        self.on_copy = Some(on_copy);

        self
    }

    pub fn view_words_row(&self) -> Column<'_, Event> {
        let words_vec = self.state.lock().unwrap().words.clone(); // TODO: remove unwrap..
        let words_row: Vec<Element<'_, Event>> = words_vec
            .chunks(4)
            .map(|chunk| {
                let words_chunk: Vec<Element<'_, Event>> = chunk
                    .iter()
                    .map(|w| {
                        Button::new(
                            Text::new(w.clone()) // TODO: remove it.
                                .size(14)
                                .horizontal_alignment(Horizontal::Center),
                        )
                        .style(zebra_ui::styles::button::outline_primary)
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

    pub fn view_content(&'a self) -> Column<'a, Event> {
        let count_pick_list = pick_list(
            self.counts.as_slice(),
            Some(self.state.lock().unwrap().count), // TODO: remove unwrap..
            Event::CountSelected,
        )
        .text_size(16)
        .padding(4)
        .style(zebra_ui::styles::pick_list::primary_field)
        .width(80);
        let language_pick_list = pick_list(
            self.dicts.as_slice(),
            Some(self.state.lock().unwrap().dict), // TODO: remove unwrap..
            Event::LanguageSelected,
        )
        .text_size(16)
        .padding(4)
        .style(zebra_ui::styles::pick_list::primary_field)
        .width(150);
        let reload_btn = Button::new(
            zebra_ui::image::reload_icon()
                .style(zebra_ui::styles::svg::primary_hover)
                .height(30)
                .width(30),
        )
        .padding(0)
        .style(zebra_ui::styles::button::transparent)
        .on_press(Event::ReGenerate);
        let icon = zebra_ui::image::copy_icon()
            .style(zebra_ui::styles::svg::primary_hover)
            .height(30)
            .width(30);
        let copy_btn = Button::new(icon)
            .padding(0)
            .style(zebra_ui::styles::button::transparent)
            .on_press(Event::Copy);

        let header_row = Row::new()
            .spacing(10)
            .push(reload_btn)
            .push(count_pick_list)
            .push(language_pick_list)
            .push(copy_btn);

        Column::new()
            .width(Length::Fill)
            .align_items(Alignment::Center)
            .padding(10)
            .push(Space::new(0, 20))
            .push(header_row)
            .push(Space::new(0, 20))
            .push(self.view_words_row())
    }

    pub fn regenerate(&self) {
        let mut rng = rand::thread_rng(); // TODO: change to ChaCha
        let m = Mnemonic::gen(
            &mut rng,
            self.state.lock().unwrap().count,
            zebra_lib::bip39::mnemonic::Language::English,
        );

        match m {
            Ok(m) => {
                self.state.lock().unwrap().words = // TODO: remove unwrap..
                m.get_vec().iter().map(|s| s.to_string()).collect();
            }
            Err(e) => {
                dbg!(e); // Remove debug.
            }
        }
    }
}

impl<'a, Message> Component<Message, Theme, Renderer> for PhraseGenForm<'a, Message>
where
    Message: Clone,
{
    type State = ();
    type Event = Event;

    fn update(&mut self, _state: &mut Self::State, event: Self::Event) -> Option<Message> {
        match event {
            Event::Copy => self.on_copy.cloned(),
            Event::ReGenerate => {
                self.regenerate();

                None
            }
            Event::CountSelected(count) => {
                match self.state.lock() {
                    Ok(mut state) => state.count = count,
                    Err(e) => {
                        dbg!(e);
                    }
                }
                self.regenerate();

                None
            }
            Event::LanguageSelected(lang) => {
                match self.state.lock() {
                    Ok(mut state) => state.dict = lang,
                    Err(e) => {
                        dbg!(e);
                    }
                }
                self.regenerate();

                None
            }
        }
    }

    fn view(
        &self,
        _state: &Self::State,
    ) -> iced::advanced::graphics::core::Element<'_, Self::Event, Theme, Renderer> {
        Container::new(self.view_content()).into()
    }
}

impl<'a, Message> From<PhraseGenForm<'a, Message>> for Element<'a, Message>
where
    Message: 'a + Clone,
{
    fn from(form: PhraseGenForm<'a, Message>) -> Self {
        component(form)
    }
}
