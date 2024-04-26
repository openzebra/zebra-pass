//! -- Copyright (c) 2024 Rina Khasanshin
//! -- Email: hicarus@yandex.ru
//! -- Licensed under the GNU General Public License Version 3.0 (GPL-3.0)

use crate::rust_i18n::t;
use iced::widget::{
    component, slider, text_input, Button, Checkbox, Column, Component, Container, Row,
};
use iced::Renderer;
use iced::Theme;
use iced::{Element, Length};
use std::sync::{Arc, Mutex};
use zebra_lib::core::passgen::PassGen;
use zebra_lib::errors::ZebraErrors;

#[derive(Debug)]
pub struct PassGenState {
    pub value: String,
    pub length: u8,
}

pub struct PassGenForm<Message>
where
    Message: Clone,
{
    state: Arc<Mutex<PassGenState>>,
    generator: PassGen,
    copy_msg: Option<Message>,
}

#[derive(Clone)]
pub enum Event {
    Refresh,
    Copy,
    SliderChanged(u8),
    InputLength(String),
    InputLowercase(bool),
    InputUpercase(bool),
    InputNums(bool),
    InputSymbol(bool),
    InputEmpty(String),
}

impl<Message> PassGenForm<Message>
where
    Message: Clone,
{
    pub fn new(state: Arc<Mutex<PassGenState>>) -> Result<Self, ZebraErrors> {
        let mut locked_state = state.lock().unwrap(); // TODO: remove unwrap..
        let mut rng = rand::thread_rng(); // TODO: change to ChaCha
        let generator = zebra_lib::core::passgen::PassGen::default();
        let entropy_bytes = generator.gen(locked_state.length as usize, &mut rng)?;

        if locked_state.value.is_empty() {
            locked_state.value = String::from_utf8_lossy(&entropy_bytes).to_string();
        }

        drop(locked_state);

        Ok(Self {
            state,
            generator,
            copy_msg: None,
        })
    }

    pub fn set_copy_message(mut self, msg: Message) -> Self {
        self.copy_msg = Some(msg);

        self
    }

    pub fn regenerate(&self) {
        let mut rng = rand::thread_rng(); // TODO: change to ChaCha
        let mut locked_state = self.state.lock().unwrap(); // TODO: remove unwrap..

        match self.generator.gen(locked_state.length as usize, &mut rng) {
            Ok(bytes) => {
                locked_state.value = String::from_utf8_lossy(&bytes).to_string();
            }
            Err(_) => {}
        }
    }

    pub fn view_slider(&self) -> Container<Event> {
        let state = self.state.lock().unwrap(); // TODO: remove unwrap..
        let h_slider = slider(1..=u8::MAX, state.length, Event::SliderChanged);
        let input_len = text_input("", &state.length.to_string())
            .size(12)
            .padding(4)
            .width(50)
            .style(zebra_ui::styles::input::primary)
            .on_input(Event::InputLength);
        let slider_row = Row::new().push(h_slider).push(input_len).spacing(5);

        Container::new(slider_row).width(300)
    }

    pub fn view_gen_options(&self) -> Container<Event> {
        let lowercase_check_box = Checkbox::new(t!("lowercase_opt"), self.generator.lowercase)
            .on_toggle(Event::InputLowercase)
            .text_size(14);
        let upercase_check_box = Checkbox::new(t!("upercase_opt"), self.generator.upercase)
            .on_toggle(Event::InputUpercase)
            .text_size(14);
        let nums_check_box = Checkbox::new(t!("nums_opt"), self.generator.nums)
            .on_toggle(Event::InputNums)
            .text_size(14);
        let symbols_check_box = Checkbox::new(t!("symbols_opt"), self.generator.symbols)
            .on_toggle(Event::InputSymbol)
            .text_size(14);
        let col0 = Column::new()
            .spacing(5)
            .push(lowercase_check_box)
            .push(upercase_check_box);
        let col1 = Column::new()
            .spacing(5)
            .push(nums_check_box)
            .push(symbols_check_box);
        let row = Row::new().spacing(16).push(col0).push(col1);

        Container::new(row)
    }

    pub fn view_generator(&self) -> Container<Event> {
        let entropy = text_input("", &self.short_text())
            .size(16)
            .padding(8)
            .width(250)
            .style(zebra_ui::styles::input::transparent_primary)
            .on_input(Event::InputEmpty);
        let reload_btn = Button::new(
            zebra_ui::image::reload_icon()
                .style(zebra_ui::styles::svg::primary_hover)
                .height(30)
                .width(30),
        )
        .padding(0)
        .style(zebra_ui::styles::button::transparent)
        .on_press(Event::Refresh);
        let copy_btn = Button::new(
            zebra_ui::image::copy_icon()
                .style(zebra_ui::styles::svg::primary_hover)
                .height(25)
                .width(25),
        )
        .padding(0)
        .style(zebra_ui::styles::button::transparent)
        .on_press(Event::Copy);

        let box_row: Row<'_, Event> = Row::new()
            .align_items(iced::Alignment::Center)
            .push(copy_btn)
            .push(entropy)
            .push(reload_btn);
        let border_box = Container::new(box_row)
            .style(zebra_ui::styles::container::primary_bordered)
            .padding(16);
        let col = Column::new().push(border_box);

        Container::new(col)
    }

    fn short_text(&self) -> String {
        let state = self.state.lock().unwrap(); // TODO: remove unwrap..

        state.value.clone()
    }
}

impl<Message> Component<Message, Theme, Renderer> for PassGenForm<Message>
where
    Message: Clone,
{
    type State = ();
    type Event = Event;
    fn update(&mut self, _state: &mut Self::State, event: Self::Event) -> Option<Message> {
        match event {
            Event::InputLength(v) => match v.parse::<u8>() {
                Ok(v) => {
                    if v > 0 {
                        let mut state = self.state.lock().unwrap(); // TODO: remove unwrap..
                        state.length = v;
                        drop(state);
                        self.regenerate();

                        None
                    } else {
                        None
                    }
                }
                Err(_) => None,
            },
            Event::SliderChanged(v) => {
                if v > 0 {
                    let mut state = self.state.lock().unwrap(); // TODO: remove unwrap..
                    state.length = v;
                    drop(state);
                    self.regenerate();
                }

                None
            }
            Event::InputNums(v) => {
                self.generator.nums = v;
                self.regenerate();

                None
            }
            Event::InputEmpty(_) => None,
            Event::InputSymbol(v) => {
                self.generator.symbols = v;
                self.regenerate();

                None
            }
            Event::InputUpercase(v) => {
                self.generator.upercase = v;
                self.regenerate();

                None
            }
            Event::InputLowercase(v) => {
                self.generator.lowercase = v;
                self.regenerate();

                None
            }
            Event::Copy => self.copy_msg.clone(),
            Event::Refresh => {
                self.regenerate();
                None
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

impl<'a, Message> From<PassGenForm<Message>> for Element<'a, Message>
where
    Message: 'a + Clone,
{
    fn from(form: PassGenForm<Message>) -> Self {
        component(form)
    }
}
