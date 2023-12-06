//! -- Copyright (c) 2023 Rina Khasanshin
//! -- Email: hicarus@yandex.ru
//! -- Licensed under the GNU General Public License Version 3.0 (GPL-3.0)
use serde::{Deserialize, Serialize};

#[derive(Debug, Serialize, Deserialize, PartialEq, Eq, Default, Clone)]
pub enum ElementType {
    #[default]
    Login,
}

#[derive(Debug, Serialize, Deserialize, PartialEq, Eq, Clone)]
pub struct Item {
    pub title: String,
    pub value: String,
    pub hide: bool,
    pub copy: bool,
}

#[derive(Debug, Serialize, Deserialize, PartialEq, Eq, Clone)]
pub struct Element {
    pub name: String,
    pub website: String,
    pub icon: String,
    pub element_type: ElementType,
    pub created: String,
    pub updated: String,
    pub favourite: bool,
    pub fields: Vec<Item>,
    pub extra_fields: Vec<Item>,
}
