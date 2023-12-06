//! -- Copyright (c) 2023 Rina Khasanshin
//! -- Email: hicarus@yandex.ru
//! -- Licensed under the GNU General Public License Version 3.0 (GPL-3.0)
use std::sync::Arc;

use crate::{
    gui::{GlobalMessage, Routers},
    rust_i18n::t,
};
use iced::{alignment::Horizontal, widget::Space, Command, Length, Subscription};
use zebra_lib::{core::core::Core, errors::ZebraErrors};
use zebra_ui::widget::*;

use super::{locale::Locale, Page};

#[derive(Debug, Default)]
enum SlideStep {
    #[default]
    ZebraView,
    Rust,
    Quantom,
}

#[derive(Debug)]
pub struct Interview {
    step: SlideStep,
}

#[derive(Debug, Clone)]
pub enum InterviewMessage {
    Next,
    Back,
}

impl Page for Interview {
    type Message = InterviewMessage;

    fn new(_core: Arc<Core>) -> Result<Self, ZebraErrors> {
        Ok(Self {
            step: SlideStep::ZebraView,
        })
    }

    fn subscription(&self) -> Subscription<Self::Message> {
        Subscription::none()
    }

    fn update(&mut self, message: InterviewMessage) -> Command<GlobalMessage> {
        match message {
            InterviewMessage::Next => Command::none(),
            InterviewMessage::Back => match &self.step {
                SlideStep::ZebraView => {
                    Command::none()
                    // let route = Routers::Locale(Locale::new(&self.core));
                    // Command::perform(std::future::ready(1), |_| GlobalMessage::Route(route))
                }
                SlideStep::Rust => Command::none(),
                SlideStep::Quantom => Command::none(),
            },
        }
    }

    fn view(&self) -> iced::Element<Self::Message> {
        let zebra_print = zebra_ui::image::zebra_print_view();
        let print_col = Column::new()
            .width(220)
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
        let description = Text::new(t!("start.description"))
            .size(18)
            .horizontal_alignment(Horizontal::Right)
            .vertical_alignment(iced::alignment::Vertical::Bottom);
        let zebra_img = zebra_ui::image::zebra_heat().height(250).width(250);
        let forward_btn = Button::new(zebra_ui::image::forward_icon().height(50).width(50))
            .padding(0)
            .style(zebra_ui::style::button::Button::Transparent)
            .on_press(InterviewMessage::Next);
        let back_btn = Button::new(zebra_ui::image::back_icon().height(50).width(50))
            .padding(0)
            .style(zebra_ui::style::button::Button::Transparent)
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
        let col = Column::new();
        let zebra_img = zebra_ui::image::zebra_heat().height(200).width(200);

        col.push(zebra_img)
    }

    fn quantom_slide<'a>(&self) -> Column<'a, InterviewMessage> {
        let col = Column::new();

        col
    }
}
