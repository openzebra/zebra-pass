//! -- Copyright (c) 2023 Rina Khasanshin
//! -- Email: hicarus@yandex.ru
//! -- Licensed under the GNU General Public License Version 3.0 (GPL-3.0)

use std::sync::Arc;

use crate::gui::GlobalMessage;
use iced::{Command, Element, Subscription};
use zebra_lib::{core::core::Core, errors::ZebraErrors};

pub mod inverview;
pub mod loader;
pub mod locale;

pub trait Page {
    type Message: std::fmt::Debug + Send;

    fn new(core: Arc<Core>) -> Result<Self, ZebraErrors>;
    fn subscription(&self) -> Subscription<Self::Message>;
    fn update(&mut self, message: Self::Message) -> Command<GlobalMessage>;
    fn view(&self) -> Element<Self::Message>;
}
