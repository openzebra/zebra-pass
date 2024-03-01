//! -- Copyright (c) 2023 Rina Khasanshin
//! -- Email: hicarus@yandex.ru
//! -- Licensed under the GNU General Public License Version 3.0 (GPL-3.0)
use super::Theme;
use iced::{
    widget::{
        container,
        scrollable::{self, Scroller},
    },
    Border, Shadow,
};

#[derive(Default, Clone)]
pub struct Scrollable {}

impl scrollable::StyleSheet for Theme {
    type Style = Scrollable;

    fn active(&self, _style: &Self::Style) -> scrollable::Appearance {
        let palette = match self {
            Theme::Dark(p) => p,
            Theme::Light(p) => p,
        };
        scrollable::Appearance {
            container: container::Appearance {
                text_color: palette.success.into(),
                background: Some(palette.light.into()),
                border: Border {
                    color: palette.window_background_inverse,
                    width: 1.0,
                    radius: 8.0.into(),
                },
                shadow: Shadow {
                    color: palette.secondary,
                    offset: Default::default(),
                    blur_radius: 0.0,
                },
            },
            gap: Some(iced::Color::TRANSPARENT.into()),
            scrollbar: scrollable::Scrollbar {
                background: Some(palette.secondary.into()),
                border: Border {
                    color: palette.primary,
                    width: 0.0,
                    radius: 8.0.into(),
                },
                scroller: Scroller {
                    color: palette.primary,
                    border: Border {
                        color: palette.primary,
                        width: 2.0,
                        radius: 8.0.into(),
                    },
                },
            },
        }
    }

    fn hovered(&self, style: &Self::Style, _is_hovered: bool) -> scrollable::Appearance {
        let active = self.active(style);

        active
    }
}
