//! -- Copyright (c) 2023 Rina Khasanshin
//! -- Email: hicarus@yandex.ru
//! -- Licensed under the GNU General Public License Version 3.0 (GPL-3.0)
use super::Theme;
use iced::{widget::text_input, Border};

#[derive(Debug, Copy, Clone, Default)]
pub enum TextInput {
    #[default]
    Primary,
    Danger,
}

impl text_input::StyleSheet for Theme {
    type Style = TextInput;

    fn active(&self, style: &Self::Style) -> text_input::Appearance {
        let palette = match self {
            Theme::Dark(p) => p,
            Theme::Light(p) => p,
        };
        let border_radius = palette.radius;
        let border_width = 1.0;

        match style {
            TextInput::Primary => {
                let mut alfa_primary = palette.primary;

                alfa_primary.a = 0.8;

                text_input::Appearance {
                    icon_color: palette.primary,
                    background: iced::Background::Color(iced::Color::TRANSPARENT),
                    border: Border {
                        width: border_width,
                        radius: border_radius,
                        color: alfa_primary,
                    },
                }
            }
            TextInput::Danger => text_input::Appearance {
                icon_color: palette.danger,
                background: iced::Background::Color(iced::Color::TRANSPARENT),
                border: Border {
                    width: border_width,
                    radius: border_radius,
                    color: palette.danger,
                },
            },
        }
    }

    fn disabled(&self, style: &Self::Style) -> text_input::Appearance {
        text_input::Appearance {
            ..self.active(style)
        }
    }

    fn focused(&self, style: &Self::Style) -> text_input::Appearance {
        let palette = match self {
            Theme::Dark(p) => p,
            Theme::Light(p) => p,
        };
        let border_radius = palette.radius;
        let border_width = 1.0;

        match style {
            TextInput::Primary => text_input::Appearance {
                icon_color: palette.primary,
                background: iced::Background::Color(iced::Color::TRANSPARENT),
                border: Border {
                    width: border_width,
                    radius: border_radius,
                    color: palette.primary,
                },
            },
            TextInput::Danger => text_input::Appearance {
                icon_color: palette.danger,
                background: iced::Background::Color(iced::Color::TRANSPARENT),
                border: Border {
                    width: border_width,
                    radius: border_radius,
                    color: palette.danger,
                },
            },
        }
    }

    fn disabled_color(&self, _style: &Self::Style) -> iced::Color {
        let palette = match self {
            Theme::Dark(p) => p,
            Theme::Light(p) => p,
        };

        palette.primary
    }

    fn placeholder_color(&self, style: &Self::Style) -> iced::Color {
        let palette = match self {
            Theme::Dark(p) => p,
            Theme::Light(p) => p,
        };

        match style {
            TextInput::Primary => {
                let mut alfa_primary = palette.primary;
                alfa_primary.a = 0.1;

                alfa_primary
            }
            TextInput::Danger => palette.danger,
        }
    }

    fn value_color(&self, style: &Self::Style) -> iced::Color {
        let palette = match self {
            Theme::Dark(p) => p,
            Theme::Light(p) => p,
        };

        match style {
            TextInput::Primary => palette.primary,
            TextInput::Danger => palette.danger,
        }
    }

    fn selection_color(&self, style: &Self::Style) -> iced::Color {
        let palette = match self {
            Theme::Dark(p) => p,
            Theme::Light(p) => p,
        };

        match style {
            TextInput::Primary => palette.primary,
            TextInput::Danger => palette.danger,
        }
    }
}
