//! -- Copyright (c) 2023 Rina Khasanshin
//! -- Email: hicarus@yandex.ru
//! -- Licensed under the GNU General Public License Version 3.0 (GPL-3.0)

use std::sync::{Arc, Mutex};
use zebra_lib::{core::core::Core, errors::ZebraErrors};

use crate::{
    gui::{GlobalMessage, Routers},
    rust_i18n::t,
};

use super::Page;
use iced::{alignment::Horizontal, widget::Space, Command, Length, Subscription};
use zebra_ui::widget::*;

#[derive(Debug)]
pub struct Restore {
    core: Arc<Mutex<Core>>,
}

#[derive(Debug, Clone)]
pub enum RestoreMessage {
    Back,
    Next,
}
