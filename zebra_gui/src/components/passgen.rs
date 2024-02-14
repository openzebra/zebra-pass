//! -- Copyright (c) 2024 Rina Khasanshin
//! -- Email: hicarus@yandex.ru
//! -- Licensed under the GNU General Public License Version 3.0 (GPL-3.0)

use crate::rust_i18n::t;
use iced::widget::{component, slider, text_input, Checkbox, Component};
use iced::Length;
use zebra_lib::core::passgen::PassGen;
use zebra_lib::errors::ZebraErrors;
use zebra_ui::style::Theme;
use zebra_ui::widget::*;

pub struct PassGenForm<Message>
where
    Message: Clone,
{
    value: String,
    length: u8,
    generator: PassGen,
    copy_msg: Option<Message>,
    change_msg: Option<Message>,
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
    pub fn new(length: u8) -> Result<Self, ZebraErrors> {
        let mut rng = rand::thread_rng(); // TODO: change to ChaCha
        let generator = zebra_lib::core::passgen::PassGen::default();
        let entropy_bytes = generator.gen(length as usize, &mut rng)?;
        let value = String::from_utf8_lossy(&entropy_bytes).to_string();

        Ok(Self {
            length,
            value,
            generator,
            copy_msg: None,
            change_msg: None,
        })
    }

    pub fn set_copy_message(mut self, msg: Message) -> Self {
        self.copy_msg = Some(msg);

        self
    }

    pub fn set_change_message(mut self, msg: Message) -> Self {
        self.change_msg = Some(msg);

        self
    }

    pub fn regenerate(&mut self) {
        let mut rng = rand::thread_rng(); // TODO: change to ChaCha

        match self.generator.gen(self.length as usize, &mut rng) {
            Ok(bytes) => {
                self.value = String::from_utf8_lossy(&bytes).to_string();
            }
            Err(_) => {}
        }
    }

    pub fn view_slider(&self) -> Container<Event> {
        let h_slider = slider(1..=255, self.length, Event::SliderChanged);
        let input_len = text_input("", &self.length.to_string())
            .size(12)
            .padding(4)
            .width(50)
            .on_input(Event::InputLength)
            .style(zebra_ui::style::text_input::TextInput::Primary);
        let slider_row = Row::new().push(h_slider).push(input_len).spacing(5);

        Container::new(slider_row).width(300)
    }

    pub fn view_gen_options(&self) -> Container<Event> {
        let lowercase_check_box = Checkbox::new(
            t!("lowercase_opt"),
            self.generator.lowercase,
            Event::InputLowercase,
        )
        .text_size(14);
        let upercase_check_box = Checkbox::new(
            t!("upercase_opt"),
            self.generator.upercase,
            Event::InputUpercase,
        )
        .text_size(14);
        let nums_check_box =
            Checkbox::new(t!("nums_opt"), self.generator.nums, Event::InputNums).text_size(14);
        let symbols_check_box = Checkbox::new(
            t!("symbols_opt"),
            self.generator.symbols,
            Event::InputSymbol,
        )
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
            .on_input(Event::InputEmpty)
            .style(zebra_ui::style::text_input::TextInput::Transparent);
        let reload_btn = Button::new(zebra_ui::image::reload_icon().height(30).width(30))
            .padding(0)
            .style(zebra_ui::style::button::Button::Transparent)
            .on_press(Event::Refresh);
        let copy_btn = Button::new(zebra_ui::image::copy_icon().height(25).width(25))
            .padding(0)
            .style(zebra_ui::style::button::Button::Transparent)
            .on_press(Event::Copy);

        let box_row: Row<'_, Event> = Row::new()
            .align_items(iced::Alignment::Center)
            .push(copy_btn)
            .push(entropy)
            .push(reload_btn);
        let border_box = Container::new(box_row)
            .style(zebra_ui::style::container::Container::SecondaryRoundedBox)
            .padding(16);
        let col = Column::new().push(border_box);

        Container::new(col)
    }

    fn short_text(&self) -> String {
        if self.value.len() > 22 {
            format!("{}...", &self.value[..22])
        } else {
            self.value.clone()
        }
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
                        self.length = v;
                        self.regenerate();
                        self.change_msg.clone()
                    } else {
                        None
                    }
                }
                Err(_) => None,
            },
            Event::SliderChanged(v) => {
                if v > 0 {
                    self.length = v;
                    self.regenerate();
                }

                self.change_msg.clone()
            }
            Event::InputNums(v) => {
                self.generator.nums = v;
                self.regenerate();

                self.change_msg.clone()
            }
            Event::InputEmpty(_) => None,
            Event::InputSymbol(v) => {
                self.generator.symbols = v;
                self.regenerate();

                self.change_msg.clone()
            }
            Event::InputUpercase(v) => {
                self.generator.upercase = v;
                self.regenerate();

                self.change_msg.clone()
            }
            Event::InputLowercase(v) => {
                self.generator.lowercase = v;
                self.regenerate();

                self.change_msg.clone()
            }
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

impl<'a, Message> From<PassGenForm<Message>> for Element<'a, Message>
where
    Message: 'a + Clone,
{
    fn from(form: PassGenForm<Message>) -> Self {
        component(form)
    }
}
