//! -- Copyright (c) 2023 Rina Khasanshin
//! -- Email: hicarus@yandex.ru
//! -- Licensed under the GNU General Public License Version 3.0 (GPL-3.0)
use crate::rust_i18n::t;
use iced::{alignment::Horizontal, widget::Space, Command, Length, Subscription};
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

#[derive(Debug, Clone)]
pub enum InterviewMessage {
    Next,
}

impl Interview {
    pub fn new() -> Self {
        Self {
            step: SlideStep::ZebraView,
        }
    }

    pub fn subscription(&self) -> Subscription<InterviewMessage> {
        Subscription::none()
    }

    pub fn update<M>(&mut self, _message: InterviewMessage) -> Command<M> {
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
            .size(18)
            .horizontal_alignment(Horizontal::Right)
            .vertical_alignment(iced::alignment::Vertical::Bottom);
        let zebra_img = zebra_ui::image::zebra_heat().height(250).width(250);
        let forward_btn = Button::new(zebra_ui::image::forward_icon().height(40).width(40))
            .padding(0)
            .style(zebra_ui::style::button::Button::Transparent)
            .on_press(InterviewMessage::Next);

        Column::new()
            .padding(20)
            .height(Length::Fill)
            .width(Length::Fill)
            .align_items(iced::Alignment::Center)
            .push(zebra_img)
            .push(description)
            .push(Space::with_height(50))
            .push(forward_btn)
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
