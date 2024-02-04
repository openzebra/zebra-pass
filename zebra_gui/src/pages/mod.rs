//! -- Copyright (c) 2023 Rina Khasanshin
//! -- Email: hicarus@yandex.ru
//! -- Licensed under the GNU General Public License Version 3.0 (GPL-3.0)

use std::{
    fmt::Debug,
    sync::{Arc, Mutex},
};

use crate::gui::GlobalMessage;
use iced::{Command, Subscription};
use zebra_lib::{core::core::Core, errors::ZebraErrors};

pub mod gen;
pub mod gen_phrase;
pub mod home;
pub mod inverview;
pub mod loader;
pub mod locale;
pub mod lock;
pub mod options;
pub mod password_setup;
pub mod restore;
pub mod settings;

pub trait Page {
    type Message: Debug + Send;

    fn new(core: Arc<Mutex<Core>>) -> Result<Self, ZebraErrors>
    where
        Self: Sized + Send + Debug;

    fn subscription(&self) -> Subscription<Self::Message>;

    fn update(&mut self, message: Self::Message) -> Command<GlobalMessage>;

    fn view(&self) -> zebra_ui::widget::Element<Self::Message>;
}
