//! -- Copyright (c) 2023 Rina Khasanshin
//! -- Email: hicarus@yandex.ru
//! -- Licensed under the GNU General Public License Version 3.0 (GPL-3.0)
use iced::{
    alignment::Horizontal,
    widget::{pick_list, text, Space},
    Command, Length, Subscription,
};
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
        text("tes").size(20).into()
    }
}
