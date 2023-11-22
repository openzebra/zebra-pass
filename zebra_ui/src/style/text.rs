//! -- Copyright (c) 2023 Rina Khasanshin
//! -- Email: hicarus@yandex.ru
//! -- Licensed under the GNU General Public License Version 3.0 (GPL-3.0)
use super::Theme;
use iced::widget::text;

#[derive(Clone, Copy, Default)]
pub enum Text {
    #[default]
    Default,
    Color(iced::Color),
}

impl From<iced::Color> for Text {
    fn from(color: iced::Color) -> Self {
        Text::Color(color)
    }
}

impl text::StyleSheet for Theme {
    type Style = Text;

    fn appearance(&self, style: Self::Style) -> text::Appearance {
        match style {
            Text::Default => Default::default(),
            Text::Color(c) => text::Appearance { color: Some(c) },
        }
    }
}
