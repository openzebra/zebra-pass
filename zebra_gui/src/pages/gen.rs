//! -- Copyright (c) 2023 Rina Khasanshin
//! -- Email: hicarus@yandex.ru
//! -- Licensed under the GNU General Public License Version 3.0 (GPL-3.0)

use std::sync::{Arc, Mutex};

use crate::rust_i18n::t;
use iced::widget::{slider, text_input, Checkbox};
use iced::{Command, Length, Subscription};
use zebra_lib::core::passgen::PassGen;
use zebra_lib::{core::core::Core, errors::ZebraErrors};
use zebra_ui::widget::*;

use crate::components::home_nav_bar::{NavBar, NavRoute};
use crate::gui::{GlobalMessage, Routers};

use super::home::Home;
use super::settings::Settings;
use super::Page;

#[derive(Debug)]
pub struct Generator {
    core: Arc<Mutex<Core>>,
    value: String,
    generator: PassGen,
    length: u8,
}

#[derive(Debug, Clone)]
pub enum GeneratorMessage {
    RouteHome,
    RouteSettings,
    Copy,
    Refresh,
    SliderChanged(u8),
    InputLength(String),
    InputLowercase(bool),
    InputUpercase(bool),
    InputNums(bool),
    InputSymbol(bool),
    InputEmpty(String),
}

impl Page for Generator {
    type Message = GeneratorMessage;

    fn new(core: Arc<Mutex<Core>>) -> Result<Self, ZebraErrors> {
        let mut rng = rand::thread_rng(); // TODO: change to ChaCha
        let length = 20u8;
        let generator = zebra_lib::core::passgen::PassGen::default();
        let entropy_bytes = generator.gen(length as usize, &mut rng)?;
        let value = String::from_utf8_lossy(&entropy_bytes).to_string();

        Ok(Self {
            core,
            generator,
            length,
            value,
        })
    }

    fn subscription(&self) -> Subscription<Self::Message> {
        Subscription::none()
    }

    fn update(&mut self, message: Self::Message) -> iced::Command<GlobalMessage> {
        match message {
            GeneratorMessage::RouteHome => {
                // TODO: remove unwrap!
                let home = Home::new(Arc::clone(&self.core)).unwrap();
                let route = Routers::Home(home);

                return Command::perform(std::future::ready(1), |_| GlobalMessage::Route(route));
            }
            GeneratorMessage::RouteSettings => {
                // TODO: remove unwrap!
                let settings = Settings::new(Arc::clone(&self.core)).unwrap();
                let route = Routers::Settings(settings);

                return Command::perform(std::future::ready(1), |_| GlobalMessage::Route(route));
            }
            GeneratorMessage::Copy => Command::none(),
            GeneratorMessage::Refresh => {
                self.regenerate();
                Command::none()
            }
            GeneratorMessage::SliderChanged(value) => {
                self.length = value;
                self.regenerate();
                Command::none()
            }
            GeneratorMessage::InputLength(value) => {
                match value.parse::<u8>() {
                    Ok(v) => {
                        self.length = v;
                        self.regenerate();
                    }
                    Err(_) => {}
                }
                Command::none()
            }
            GeneratorMessage::InputLowercase(value) => {
                self.generator.lowercase = value;
                self.regenerate();

                Command::none()
            }
            GeneratorMessage::InputUpercase(value) => {
                self.generator.upercase = value;
                self.regenerate();

                Command::none()
            }
            GeneratorMessage::InputNums(value) => {
                self.generator.nums = value;
                self.regenerate();

                Command::none()
            }
            GeneratorMessage::InputSymbol(value) => {
                self.generator.symbols = value;
                self.regenerate();

                Command::none()
            }
            GeneratorMessage::InputEmpty(_) => Command::none(),
        }
    }

    fn view(&self) -> Element<Self::Message> {
        NavBar::<Self::Message>::new()
            .set_route(NavRoute::Gen)
            .on_home(GeneratorMessage::RouteHome)
            .on_settings(GeneratorMessage::RouteSettings)
            .view(self.view_entropy_gen())
            .into()
    }
}

impl Generator {
    pub fn regenerate(&mut self) {
        let mut rng = rand::thread_rng(); // TODO: change to ChaCha
        let entropy_bytes = self.generator.gen(self.length as usize, &mut rng).unwrap(); // TODO: remove unwrap.

        self.value = String::from_utf8_lossy(&entropy_bytes).to_string();
    }

    pub fn view_entropy_gen(&self) -> Container<GeneratorMessage> {
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

        Container::new(row)
            .width(Length::Fill)
            .height(Length::Fill)
            .style(zebra_ui::style::container::Container::WeekBorder)
    }

    pub fn view_slider(&self) -> Container<GeneratorMessage> {
        let h_slider = slider(1..=255, self.length, GeneratorMessage::SliderChanged);
        let input_len = text_input("", &self.length.to_string())
            .size(12)
            .padding(4)
            .width(50)
            .on_input(GeneratorMessage::InputLength)
            .style(zebra_ui::style::text_input::TextInput::Primary);
        let slider_row = Row::new().push(h_slider).push(input_len).spacing(5);

        Container::new(slider_row).width(300)
    }

    pub fn view_generator(&self) -> Container<GeneratorMessage> {
        let entropy = text_input("", &self.short_text())
            .size(16)
            .padding(8)
            .width(250)
            .on_input(GeneratorMessage::InputEmpty)
            .style(zebra_ui::style::text_input::TextInput::Transparent);
        let reload_btn = Button::new(zebra_ui::image::reload_icon().height(30).width(30))
            .padding(0)
            .style(zebra_ui::style::button::Button::Transparent)
            .on_press(GeneratorMessage::Refresh);
        let copy_btn = Button::new(zebra_ui::image::copy_icon().height(25).width(25))
            .padding(0)
            .style(zebra_ui::style::button::Button::Transparent)
            .on_press(GeneratorMessage::Copy);

        let box_row: Row<'_, GeneratorMessage> = Row::new()
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

    pub fn view_gen_options(&self) -> Container<GeneratorMessage> {
        // TODO: drop when all flags down.
        let lowercase_check_box = Checkbox::new(
            t!("lowercase_opt"),
            self.generator.lowercase,
            GeneratorMessage::InputLowercase,
        )
        .text_size(14);
        let upercase_check_box = Checkbox::new(
            t!("upercase_opt"),
            self.generator.upercase,
            GeneratorMessage::InputUpercase,
        )
        .text_size(14);
        let nums_check_box = Checkbox::new(
            t!("nums_opt"),
            self.generator.nums,
            GeneratorMessage::InputNums,
        )
        .text_size(14);
        let symbols_check_box = Checkbox::new(
            t!("symbols_opt"),
            self.generator.symbols,
            GeneratorMessage::InputSymbol,
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

    fn short_text(&self) -> String {
        if self.value.len() > 22 {
            format!("{}...", &self.value[..22])
        } else {
            self.value.clone()
        }
    }
}
