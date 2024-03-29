//! -- Copyright (c) 2023 Rina Khasanshin
//! -- Email: hicarus@yandex.ru
//! -- Licensed under the GNU General Public License Version 3.0 (GPL-3.0)
use crate::rust_i18n::t;
use iced::{executor, Application, Command, Element};
use iced::{Alignment, Length};
use zebra_ui::widget::*;
use zebra_ui::{color::ZebraPalette, style};

pub struct GUIError {
    error: String,
}

#[derive(Debug)]
pub enum ErrorMessage {}

impl Application for GUIError {
    type Executor = executor::Default;
    type Message = ErrorMessage;
    type Flags = String;
    type Theme = style::Theme;

    fn title(&self) -> String {
        "ZebraPass Error".into()
    }

    fn new(error: Self::Flags) -> (Self, Command<Self::Message>) {
        (Self { error }, Command::none())
    }

    fn update(&mut self, _message: Self::Message) -> Command<Self::Message> {
        Command::none()
    }

    fn view(&self) -> Element<'_, Self::Message, zebra_ui::style::Theme> {
        let palette = match self.theme() {
            style::Theme::Dark(p) => p,
            style::Theme::Light(p) => p,
        };
        let message = Text::new(t!("error.message", error = &self.error))
            .size(20)
            .style(style::text::Text::Color(palette.danger))
            .horizontal_alignment(iced::alignment::Horizontal::Center);

        let row = Row::new()
            .height(Length::Fill)
            .align_items(Alignment::Center)
            .push(message);

        Column::new()
            .width(Length::Fill)
            .align_items(Alignment::Center)
            .push(row)
            .into()
    }

    fn scale_factor(&self) -> f64 {
        1.0
    }

    fn theme(&self) -> Self::Theme {
        match dark_light::detect() {
            dark_light::Mode::Dark => style::Theme::Dark(ZebraPalette::DARK),
            dark_light::Mode::Light => style::Theme::Light(ZebraPalette::LIGHT),
            dark_light::Mode::Default => style::Theme::Dark(ZebraPalette::DARK),
        }
    }
}
