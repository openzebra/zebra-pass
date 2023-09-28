pub mod english;

pub enum Language {
    English,
}

impl Default for Language {
    fn default() -> Self {
        Language::English
    }
}
