//! -- Copyright (c) 2023 Rina Khasanshin
//! -- Email: hicarus@yandex.ru
//! -- Licensed under the GNU General Public License Version 3.0 (GPL-3.0)
use super::view::message::ViewMessage;
use iced::{Command, Subscription};
use message::StateMessage;
use zebra_ui::widget::Element;

mod message;

pub trait State {
    fn view<'a>(&'a self) -> Element<'a, ViewMessage>;
    fn update(&mut self) -> Command<StateMessage> {
        Command::none()
    }
    fn subscription(&self) -> Subscription<StateMessage> {
        Subscription::none()
    }
    fn load(&self) -> Command<StateMessage> {
        Command::none()
    }
}
