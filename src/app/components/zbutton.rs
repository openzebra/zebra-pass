//! -- Copyright (c) 2023 Rina Khasanshin
//! -- Email: hicarus@yandex.ru
//! -- Licensed under the GNU General Public License Version 3.0 (GPL-3.0)

use iced::Color;

#[derive(Default)]
pub enum ZButtonStyle {
    #[default]
    Default,
    LightlyBordered,
}

pub struct ZButton(ZButtonStyle);

impl Default for ZButton {
    fn default() -> Self {
        Self(ZButtonStyle::Default)
    }
}

impl std::convert::From<ZButton> for iced::theme::Button {
    fn from(value: ZButton) -> Self {
        iced::theme::Button::Custom(Box::new(value))
    }
}

impl iced::widget::button::StyleSheet for ZButton {
    type Style = iced::theme::Theme;

    fn active(&self, style: &Self::Style) -> iced::widget::button::Appearance {
        let background_color = match self.0 {
            ZButtonStyle::Default => style.palette().background,
            ZButtonStyle::LightlyBordered => iced::Color::TRANSPARENT,
        };

        let border_color = match self.0 {
            ZButtonStyle::Default => iced::Color::TRANSPARENT,
            ZButtonStyle::LightlyBordered => iced::Color {
                a: 0.1,
                ..style.palette().text
            },
        };

        let border_width = match self.0 {
            ZButtonStyle::Default => 0.0,
            ZButtonStyle::LightlyBordered => 1.0,
        };

        iced::widget::button::Appearance {
            text_color: style.palette().text,
            background: Some(background_color.into()),
            border_color,
            border_width,
            border_radius: 6.0.into(),
            ..Default::default()
        }
    }
}

impl ZButton {
    pub fn new() -> Self {
        Self::default()
    }

    pub fn new_bordered() -> Self {
        Self::lightly_bordered()
    }

    pub fn lightly_bordered() -> Self {
        Self(ZButtonStyle::LightlyBordered)
    }
}
