//! -- Copyright (c) 2023 Rina Khasanshin
//! -- Email: hicarus@yandex.ru
//! -- Licensed under the GNU General Public License Version 3.0 (GPL-3.0)
use crate::rust_i18n::t;
use iced::{alignment::Horizontal, Command, Length, Subscription};
use zebra_ui::widget::*;

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

#[derive(Debug)]
pub enum InterviewMessage {}

impl Interview {
    pub fn new() -> Self {
        Self {
            step: SlideStep::ZebraView,
        }
    }

    pub fn subscription(&self) -> Subscription<InterviewMessage> {
        Subscription::none()
    }

    pub fn update<M>(&mut self, message: InterviewMessage) -> Command<M> {
        Command::none()
    }

    pub fn view(&self) -> Element<InterviewMessage> {
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

    fn start_slide<'a>(&self) -> Column<'a, InterviewMessage> {
        let description = Text::new(t!("start.description"))
            .size(20)
            .horizontal_alignment(Horizontal::Center);

        Column::new()
            .height(Length::Fill)
            .width(Length::Fill)
            .align_items(iced::Alignment::Center)
            .push(description)
    }

    fn rust_slide<'a>(&self) -> Column<'a, InterviewMessage> {
        let col = Column::new();

        col
    }

    fn quantom_slide<'a>(&self) -> Column<'a, InterviewMessage> {
        let col = Column::new();

        col
    }
}
