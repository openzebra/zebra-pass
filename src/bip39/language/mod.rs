pub mod english;

pub enum Language {
    English,
}

impl Default for Language {
    fn default() -> Self {
        Language::English
    }
}

impl Language {
    #[inline]
    pub fn word_list(&self) -> &'static [&'static str; 2048] {
        match self {
            Language::English => &english::WORDS,
        }
    }

    #[inline]
    pub fn find_word(&self, word: &str) -> Option<u16> {
        self.word_list()
            .iter()
            .position(|w| *w == word)
            .map(|i| i as u16)
    }
}
