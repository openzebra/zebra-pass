//! -- Copyright (c) 2023 Rina Khasanshin
//! -- Email: hicarus@yandex.ru
//! -- Licensed under the GNU General Public License Version 3.0 (GPL-3.0)
use super::Theme;
use iced::widget::text;

#[derive(Clone, Copy, Default)]
pub enum Text {
    #[default]
    Default,
    Dabger,
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
        let palette = match self {
            Theme::Dark(p) => p,
            Theme::Light(p) => p,
        };
        match style {
            Text::Default => Default::default(),
            Text::Dabger => text::Appearance {
                color: palette.danger.into(),
            },
            Text::Color(c) => text::Appearance { color: Some(c) },
        }
    }
}
