//! -- Copyright (c) 2024 Rina Khasanshin
//! -- Email: hicarus@yandex.ru
//! -- Licensed under the GNU General Public License Version 3.0 (GPL-3.0)

use iced::alignment::Horizontal;
use iced::widget::{component, pick_list, Component, Space};
use iced::{clipboard, Alignment, Length};
use std::sync::{Arc, Mutex};
use zebra_lib::bip39::config::MAX_NB_WORDS;
use zebra_lib::bip39::mnemonic;
use zebra_lib::bip39::mnemonic::Mnemonic;
use zebra_lib::errors::ZebraErrors;
use zebra_ui::style::Theme;
use zebra_ui::widget::*;

#[derive(Debug)]
pub struct PhraseGenState {
    pub dict: mnemonic::Language,
    pub words: Vec<String>,
    pub count: usize,
}

impl<'a> Default for PhraseGenState {
    fn default() -> Self {
        Self {
            count: MAX_NB_WORDS,
            dict: mnemonic::Language::English,
            words: Vec::with_capacity(MAX_NB_WORDS),
        }
    }
}

#[derive(Debug)]
pub struct PhraseGenForm<Message>
where
    Message: Clone,
{
    state: Arc<Mutex<PhraseGenState>>,
    counts: [usize; 5],
    dicts: [mnemonic::Language; 1],
    on_change: Option<Message>,
}

#[derive(Clone)]
pub enum Event {
    ReGenerate,
    Copy,
    CountSelected(usize),
    LanguageSelected(mnemonic::Language),
}

impl<Message> PhraseGenForm<Message>
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
            on_change: None,
        })
    }

    pub fn set_on_change(mut self, on_change: Message) -> Self {
        self.on_change = Some(on_change);

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
        let count_pick_list = pick_list(
            self.counts.as_slice(),
            Some(self.state.lock().unwrap().count), // TODO: remove unwrap..
            Event::CountSelected,
        )
        .text_size(16)
        .padding(4)
        .width(80)
        .style(zebra_ui::style::pick_list::PickList::OutlineLight);
        let language_pick_list = pick_list(
            self.dicts.as_slice(),
            Some(self.state.lock().unwrap().dict), // TODO: remove unwrap..
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

impl<Message> Component<Message, Theme, Renderer> for PhraseGenForm<Message>
where
    Message: Clone,
{
    type State = ();
    type Event = Event;

    fn update(&mut self, _state: &mut Self::State, event: Self::Event) -> Option<Message> {
        match event {
            Event::Copy => {
                match self.state.lock() {
                    Ok(state) => {
                        clipboard::write::<Message>(state.words.join(" "));
                    }
                    Err(e) => {
                        dbg!(e);
                    }
                }
                return None;
            }
            Event::ReGenerate => {
                self.regenerate();

                None
            }
            Event::CountSelected(count) => {
                self.state.lock().unwrap().count = count; // remove unwrap...
                self.regenerate();

                None
            }
            Event::LanguageSelected(lang) => {
                self.state.lock().unwrap().dict = lang; // remove unwrap..
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

impl<'a, Message> From<PhraseGenForm<Message>> for Element<'a, Message>
where
    Message: 'a + Clone,
{
    fn from(form: PhraseGenForm<Message>) -> Self {
        component(form)
    }
}
