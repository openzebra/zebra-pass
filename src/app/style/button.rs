//! -- Copyright (c) 2023 Rina Khasanshin
//! -- Email: hicarus@yandex.ru
//! -- Licensed under the GNU General Public License Version 3.0 (GPL-3.0)

#[derive(Default)]
pub enum ZButtonStyle {
    #[default]
    Primary,
    OutlinePrimary,
    ListItem,
}

pub struct ZButton(ZButtonStyle);

impl Default for ZButton {
    fn default() -> Self {
        Self(ZButtonStyle::Primary)
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
            ZButtonStyle::Primary => style.palette().primary,
            ZButtonStyle::OutlinePrimary => iced::Color::TRANSPARENT,
            ZButtonStyle::ListItem => iced::Color::TRANSPARENT,
        };
        let border_color = match self.0 {
            ZButtonStyle::Primary => iced::Color::TRANSPARENT,
            ZButtonStyle::OutlinePrimary => style.palette().primary,
            ZButtonStyle::ListItem => iced::Color::TRANSPARENT,
        };
        let border_width = match self.0 {
            ZButtonStyle::Primary => 1.0,
            ZButtonStyle::OutlinePrimary => 1.0,
            ZButtonStyle::ListItem => 0.0,
        };
        let text_color = match self.0 {
            ZButtonStyle::Primary => style.palette().text,
            ZButtonStyle::OutlinePrimary => style.palette().primary,
            ZButtonStyle::ListItem => style.palette().text,
        };

        iced::widget::button::Appearance {
            text_color,
            background: Some(background_color.into()),
            border_color,
            border_width,
            border_radius: 6.0.into(),
            ..Default::default()
        }
    }

    fn hovered(&self, style: &Self::Style) -> iced::widget::button::Appearance {
        let background_color = match self.0 {
            ZButtonStyle::Primary => iced::Color::TRANSPARENT,
            ZButtonStyle::OutlinePrimary => style.palette().primary,
            ZButtonStyle::ListItem => iced::Color::TRANSPARENT,
        };
        let border_color = match self.0 {
            ZButtonStyle::Primary => style.palette().primary,
            ZButtonStyle::OutlinePrimary => style.palette().primary,
            ZButtonStyle::ListItem => iced::Color::TRANSPARENT,
        };
        let border_width = match self.0 {
            ZButtonStyle::Primary => 1.0,
            ZButtonStyle::OutlinePrimary => 1.0,
            ZButtonStyle::ListItem => 0.0,
        };
        let text_color = match self.0 {
            ZButtonStyle::Primary => style.palette().primary,
            ZButtonStyle::OutlinePrimary => style.palette().text,
            ZButtonStyle::ListItem => style.palette().text,
        };

        iced::widget::button::Appearance {
            text_color,
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

    pub fn outline_primary() -> Self {
        Self(ZButtonStyle::OutlinePrimary)
    }

    pub fn list_item() -> Self {
        Self(ZButtonStyle::ListItem)
    }
}
