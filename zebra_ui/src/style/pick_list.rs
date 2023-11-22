//! -- Copyright (c) 2023 Rina Khasanshin
//! -- Email: hicarus@yandex.ru
//! -- Licensed under the GNU General Public License Version 3.0 (GPL-3.0)
use super::Theme;
use iced::{widget::pick_list, BorderRadius};

#[derive(Default, Clone)]
pub enum PickList {
    #[default]
    Primary,
    OutlineLight,
}
impl pick_list::StyleSheet for Theme {
    type Style = PickList;

    fn active(&self, style: &Self::Style) -> pick_list::Appearance {
        let palette = match self {
            Theme::Dark(p) => p,
            Theme::Light(p) => p,
        };
        match style {
            PickList::Primary => pick_list::Appearance {
                placeholder_color: Default::default(),
                handle_color: palette.warn,
                background: palette.primary.into(),
                border_width: 1.0,
                border_color: palette.danger,
                border_radius: BorderRadius::from(palette.radius),
                text_color: iced::Color::BLACK,
            },
            PickList::OutlineLight => pick_list::Appearance {
                placeholder_color: palette.primary,
                handle_color: palette.window_background_inverse,
                background: iced::Color::TRANSPARENT.into(),
                border_width: 1.0,
                border_color: palette.window_background_inverse,
                border_radius: BorderRadius::from(palette.radius),
                text_color: palette.window_background_inverse,
            },
        }
    }

    fn hovered(&self, style: &Self::Style) -> pick_list::Appearance {
        let _palette = match self {
            Theme::Dark(p) => p,
            Theme::Light(p) => p,
        };
        let active = self.active(style);
        match style {
            PickList::Primary => pick_list::Appearance { ..active },
            PickList::OutlineLight => pick_list::Appearance {
                // border_radius: [0., 0., 0., 0.].into(),
                ..active
            },
        }
    }
}

#[derive(Clone, Copy, Default)]
pub enum Overlay {
    #[default]
    Default,
}
impl iced::overlay::menu::StyleSheet for Theme {
    type Style = Overlay;

    fn appearance(&self, _style: &Self::Style) -> iced::overlay::menu::Appearance {
        let palette = match self {
            Theme::Dark(p) => p,
            Theme::Light(p) => p,
        };
        iced::overlay::menu::Appearance {
            text_color: palette.window_background_inverse,
            background: palette.window_background.into(),
            border_width: 0.0,
            border_radius: [0., 0., 0., 0.].into(),
            border_color: palette.window_background_inverse,
            selected_text_color: palette.window_background_inverse,
            selected_background: palette.secondary.into(),
        }
    }
}

impl From<PickList> for Overlay {
    fn from(_p: PickList) -> Overlay {
        Overlay::Default
    }
}
