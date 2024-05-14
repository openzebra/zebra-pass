//! -- Copyright (c) 2023 Rina Khasanshin
//! -- Email: hicarus@yandex.ru
//! -- Licensed under the GNU General Public License Version 3.0 (GPL-3.0)
use std::sync::{Arc, Mutex};

use crate::{
    gui::{GlobalMessage, Routers},
    rust_i18n::t,
};
use iced::{alignment::Horizontal, Command, Length, Subscription};
use iced::{
    widget::{Button, Column, Container, Row, Space, Text},
    Element,
};
use zebra_lib::{core::Core, errors::ZebraErrors};
use zebra_ui::config::PRINT_WIDTH;

use super::{error::ErrorPage, locale::Locale, options::Options, Page};

#[derive(Debug, Default)]
pub enum SlideStep {
    #[default]
    ZebraView,
    Rust,
    Quantom,
}

#[derive(Debug)]
pub struct Interview {
    pub step: SlideStep,
    core: Arc<Mutex<Core>>,
}

#[derive(Debug, Clone)]
pub enum InterviewMessage {
    Next,
    Back,
}

impl Page for Interview {
    type Message = InterviewMessage;

    fn new(core: Arc<Mutex<Core>>) -> Result<Self, ZebraErrors> {
        Ok(Self {
            core,
            step: SlideStep::ZebraView,
        })
    }

    fn subscription(&self) -> Subscription<Self::Message> {
        Subscription::none()
    }

    fn update(&mut self, message: InterviewMessage) -> Command<GlobalMessage> {
        match message {
            InterviewMessage::Next => match self.step {
                SlideStep::ZebraView => {
                    self.step = SlideStep::Rust;
                    Command::none()
                }
                SlideStep::Rust => {
                    self.step = SlideStep::Quantom;
                    Command::none()
                }
                SlideStep::Quantom => match Options::new(Arc::clone(&self.core)) {
                    Ok(options) => {
                        let route = Routers::Options(options);
                        Command::perform(std::future::ready(1), |_| GlobalMessage::Route(route))
                    }
                    Err(e) => {
                        let route = Routers::ErrorPage(ErrorPage::from(e.to_string()));
                        Command::perform(std::future::ready(1), |_| GlobalMessage::Route(route))
                    }
                },
            },
            InterviewMessage::Back => match self.step {
                SlideStep::ZebraView => match Locale::new(Arc::clone(&self.core)) {
                    Ok(locale) => {
                        let route = Routers::Locale(locale);
                        Command::perform(std::future::ready(1), |_| GlobalMessage::Route(route))
                    }
                    Err(e) => {
                        let route = Routers::ErrorPage(ErrorPage::from(e.to_string()));
                        Command::perform(std::future::ready(1), |_| GlobalMessage::Route(route))
                    }
                },
                SlideStep::Rust => {
                    self.step = SlideStep::ZebraView;
                    Command::none()
                }
                SlideStep::Quantom => {
                    self.step = SlideStep::Rust;
                    Command::none()
                }
            },
        }
    }

    fn view(&self) -> Element<Self::Message> {
        let zebra_print = zebra_ui::image::zebra_print_view();
        let print_col = Column::new()
            .width(PRINT_WIDTH)
            .height(Length::Fill)
            .push(zebra_print);
        let row = Row::new()
            .width(Length::Fill)
            .push(print_col)
            .push(match &self.step {
                SlideStep::ZebraView => self.start_slide(),
                SlideStep::Rust => self.rust_slide(),
                SlideStep::Quantom => self.quantom_slide(),
            });

        Container::new(row)
            .height(Length::Fill)
            .width(Length::Fill)
            .into()
    }
}

impl Interview {
    fn start_slide<'a>(&self) -> Column<'a, InterviewMessage> {
        let description = Text::new(t!("zebra.description"))
            .size(18)
            .horizontal_alignment(Horizontal::Right)
            .vertical_alignment(iced::alignment::Vertical::Bottom);
        let zebra_img = zebra_ui::image::zebra_heat().height(250).width(250);
        let forward_btn = Button::new(zebra_ui::image::forward_icon().height(50).width(50))
            .padding(0)
            .style(zebra_ui::styles::button::transparent)
            .on_press(InterviewMessage::Next);
        let back_btn = Button::new(zebra_ui::image::back_icon().height(50).width(50))
            .padding(0)
            .style(zebra_ui::styles::button::transparent)
            .on_press(InterviewMessage::Back);
        let btns_row = Row::new().push(back_btn).push(forward_btn);

        Column::new()
            .padding(20)
            .height(Length::Fill)
            .width(Length::Fill)
            .align_items(iced::Alignment::Center)
            .push(zebra_img)
            .push(description)
            .push(Space::with_height(60))
            .push(btns_row)
    }

    fn rust_slide<'a>(&self) -> Column<'a, InterviewMessage> {
        let description = Text::new(t!("rust.description"))
            .size(18)
            .horizontal_alignment(Horizontal::Right)
            .vertical_alignment(iced::alignment::Vertical::Bottom);
        let zebra_img = zebra_ui::image::rust_logo().height(250).width(250);
        let forward_btn = Button::new(zebra_ui::image::forward_icon().height(50).width(50))
            .padding(0)
            .style(zebra_ui::styles::button::transparent)
            .on_press(InterviewMessage::Next);
        let back_btn = Button::new(zebra_ui::image::back_icon().height(50).width(50))
            .padding(0)
            .style(zebra_ui::styles::button::transparent)
            .on_press(InterviewMessage::Back);
        let btns_row = Row::new().push(back_btn).push(forward_btn);

        Column::new()
            .padding(20)
            .height(Length::Fill)
            .width(Length::Fill)
            .align_items(iced::Alignment::Center)
            .push(zebra_img)
            .push(description)
            .push(Space::with_height(60))
            .push(btns_row)
    }

    fn quantom_slide<'a>(&self) -> Column<'a, InterviewMessage> {
        let description = Text::new(t!("quantom.description"))
            .size(18)
            .horizontal_alignment(Horizontal::Right)
            .vertical_alignment(iced::alignment::Vertical::Bottom);
        let zebra_img = zebra_ui::image::atom().height(250).width(250);
        let forward_btn = Button::new(zebra_ui::image::forward_icon().height(50).width(50))
            .padding(0)
            .style(zebra_ui::styles::button::transparent)
            .on_press(InterviewMessage::Next);
        let back_btn = Button::new(zebra_ui::image::back_icon().height(50).width(50))
            .padding(0)
            .style(zebra_ui::styles::button::transparent)
            .on_press(InterviewMessage::Back);
        let btns_row = Row::new().push(back_btn).push(forward_btn);

        Column::new()
            .padding(20)
            .height(Length::Fill)
            .width(Length::Fill)
            .align_items(iced::Alignment::Center)
            .push(zebra_img)
            .push(description)
            .push(Space::with_height(60))
            .push(btns_row)
    }
}
