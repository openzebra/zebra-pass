//! -- Copyright (c) 2023 Rina Khasanshin
//! -- Email: hicarus@yandex.ru
//! -- Licensed under the GNU General Public License Version 3.0 (GPL-3.0)
use serde::{Deserialize, Serialize};

#[derive(Debug, Serialize, Deserialize, Default)]
pub enum ElementType {
    #[default]
    Login,
}

#[derive(Debug, Serialize, Deserialize)]
pub struct Item {
    title: String,
    value: String,
    hide: bool,
    copy: bool,
}

#[derive(Debug, Serialize, Deserialize)]
pub struct Element {
    name: String,
    website: String,
    icon: String,
    element_type: ElementType,
    created: String,
    updated: String,
    favourite: bool,
    fields: Vec<Item>,
    extra_fields: Vec<Item>,
}
