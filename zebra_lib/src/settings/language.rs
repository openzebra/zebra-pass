//! -- Copyright (c) 2023 Rina Khasanshin
//! -- Email: hicarus@yandex.ru
//! -- Licensed under the GNU General Public License Version 3.0 (GPL-3.0)
use serde::{Deserialize, Serialize};

#[derive(Debug, Clone, Copy, PartialEq, Eq, Default, Serialize, Deserialize)]
pub enum Language {
    #[default]
    English,
    Russian,
    Danish,
    French,
    German,
    Italian,
    Portuguese,
    Spanish,
}

impl Language {
    pub const ALL: [Language; 8] = [
        Language::Russian,
        Language::Danish,
        Language::English,
        Language::French,
        Language::German,
        Language::Italian,
        Language::Portuguese,
        Language::Spanish,
    ];

    pub fn symbol(&self) -> &str {
        match self {
            Language::Russian => "ru",
            Language::Danish => "da",
            Language::English => "en",
            Language::French => "fr",
            Language::German => "gr",
            Language::Italian => "it",
            Language::Portuguese => "pr",
            Language::Spanish => "sp",
        }
    }
}

impl std::fmt::Display for Language {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        write!(
            f,
            "{}",
            match self {
                Language::Russian => "Russian",
                Language::Danish => "Danish",
                Language::English => "English",
                Language::French => "French",
                Language::German => "German",
                Language::Italian => "Italian",
                Language::Portuguese => "Portuguese",
                Language::Spanish => "Spanish",
            }
        )
    }
}
