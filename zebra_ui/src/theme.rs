//! -- Copyright (c) 2023 Rina Khasanshin
//! -- Email: hicarus@yandex.ru
//! -- Licensed under the GNU General Public License Version 3.0 (GPL-3.0)

use iced::{
    application,
    widget::{
        button, checkbox, combo_box, container, pick_list, radio, scrollable, slider, svg, text,
        text_input,
    },
    BorderRadius, Color,
};

use super::color::ZebraPalette;

#[derive(Debug, Copy, Clone, PartialEq)]
pub enum Theme {
    Light(ZebraPalette),
    Dark(ZebraPalette),
}

impl Default for Theme {
    fn default() -> Self {
        Self::Light(Default::default())
    }
}

impl application::StyleSheet for Theme {
    type Style = ();

    fn appearance(&self, _style: &Self::Style) -> application::Appearance {
        match self {
            Theme::Light(palette) => application::Appearance {
                background_color: palette.window_background,
                text_color: palette.window_background_inverse,
            },
            Theme::Dark(palette) => application::Appearance {
                background_color: palette.window_background,
                text_color: palette.window_background_inverse,
            },
        }
    }
}

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

#[derive(Debug, Copy, Clone, Default)]
pub enum Container {
    #[default]
    Transparent,
    Background,
    Custom(iced::Color),
}

impl container::StyleSheet for Theme {
    type Style = Container;
    fn appearance(&self, style: &Self::Style) -> iced::widget::container::Appearance {
        match self {
            Theme::Light(p) => match style {
                Container::Transparent => container::Appearance {
                    background: Some(iced::Background::Color(Color::TRANSPARENT)),
                    ..container::Appearance::default()
                },
                Container::Background => container::Appearance {
                    background: Some(iced::Background::Color(p.danger)),
                    ..container::Appearance::default()
                },
                Container::Custom(c) => container::Appearance {
                    background: Some(iced::Background::Color(*c)),
                    ..container::Appearance::default()
                },
            },
            Theme::Dark(_p) => match style {
                Container::Transparent => container::Appearance {
                    background: Some(iced::Background::Color(Color::TRANSPARENT)),
                    ..container::Appearance::default()
                },
                Container::Background => container::Appearance {
                    background: Some(iced::Background::Color(Color::TRANSPARENT)),
                    ..container::Appearance::default()
                },
                Container::Custom(c) => container::Appearance {
                    background: Some(iced::Background::Color(*c)),
                    ..container::Appearance::default()
                },
            },
        }
    }
}

#[derive(Default)]
pub struct Radio {} // TODO: add primary, secondary...
impl radio::StyleSheet for Theme {
    type Style = Radio;

    fn active(&self, _style: &Self::Style, _is_selected: bool) -> radio::Appearance {
        let palette = match self {
            Theme::Dark(p) => p,
            Theme::Light(p) => p,
        };

        radio::Appearance {
            background: iced::Color::TRANSPARENT.into(),
            dot_color: palette.primary,
            border_width: 1.0,
            border_color: palette.primary,
            text_color: None,
        }
    }

    fn hovered(&self, style: &Self::Style, is_selected: bool) -> radio::Appearance {
        let active = self.active(style, is_selected);
        let palette = match self {
            Theme::Dark(p) => p,
            Theme::Light(p) => p,
        };

        radio::Appearance {
            dot_color: palette.secondary,
            background: iced::Color::TRANSPARENT.into(),
            ..active
        }
    }
}

#[derive(Default, Clone)]
pub struct Scrollable {}
impl scrollable::StyleSheet for Theme {
    type Style = Scrollable;

    fn active(&self, _style: &Self::Style) -> scrollable::Scrollbar {
        let palette = match self {
            Theme::Dark(p) => p,
            Theme::Light(p) => p,
        };
        scrollable::Scrollbar {
            background: None,
            border_width: 0.0,
            border_color: palette.primary,
            border_radius: BorderRadius::from(palette.radius),
            scroller: scrollable::Scroller {
                color: palette.secondary,
                border_radius: BorderRadius::from(palette.radius),
                border_width: 0.0,
                border_color: iced::Color::TRANSPARENT,
            },
        }
    }

    fn hovered(&self, style: &Self::Style, _is_hovered: bool) -> scrollable::Scrollbar {
        let active = self.active(style);
        scrollable::Scrollbar { ..active }
    }
}

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

#[derive(Default)]
pub struct CheckBox {}
impl checkbox::StyleSheet for Theme {
    type Style = CheckBox;

    fn active(&self, _style: &Self::Style, is_selected: bool) -> checkbox::Appearance {
        let palette = match self {
            Theme::Dark(p) => p,
            Theme::Light(p) => p,
        };

        if is_selected {
            checkbox::Appearance {
                background: palette.window_background_inverse.into(),
                border_width: 0.0,
                border_color: iced::Color::TRANSPARENT,
                icon_color: palette.window_background_inverse,
                text_color: None,
                border_radius: BorderRadius::from(palette.radius),
            }
        } else {
            checkbox::Appearance {
                background: palette.danger.into(),
                border_width: 0.0,
                border_color: iced::Color::TRANSPARENT,
                icon_color: palette.window_background_inverse,
                text_color: None,
                border_radius: BorderRadius::from(palette.radius),
            }
        }
    }

    fn hovered(&self, style: &Self::Style, is_selected: bool) -> checkbox::Appearance {
        self.active(style, is_selected)
    }
}

#[derive(Default)]
pub enum Button {
    #[default]
    Primary,
    OutlinePrimary,
}

impl button::StyleSheet for Theme {
    type Style = Button;

    fn active(&self, style: &Self::Style) -> iced::widget::button::Appearance {
        let palette = match self {
            Theme::Dark(p) => p,
            Theme::Light(p) => p,
        };
        let background_color = match style {
            Button::Primary => palette.primary,
            Button::OutlinePrimary => iced::Color::TRANSPARENT,
        };
        let border_color = match style {
            Button::Primary => iced::Color::TRANSPARENT,
            Button::OutlinePrimary => palette.primary,
        };
        let border_width = match style {
            Button::Primary => 1.0,
            Button::OutlinePrimary => 1.0,
        };
        let text_color = match style {
            Button::Primary => palette.window_background_inverse,
            Button::OutlinePrimary => palette.primary,
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
        let palette = match self {
            Theme::Dark(p) => p,
            Theme::Light(p) => p,
        };
        let background_color = match style {
            Button::Primary => iced::Color::TRANSPARENT,
            Button::OutlinePrimary => palette.primary,
        };
        let border_color = match style {
            Button::Primary => palette.primary,
            Button::OutlinePrimary => palette.primary,
        };
        let border_width = match style {
            Button::Primary => 1.0,
            Button::OutlinePrimary => 1.0,
        };
        let text_color = match style {
            Button::Primary => palette.primary,
            Button::OutlinePrimary => palette.window_background_inverse,
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

#[derive(Debug, Copy, Clone, Default)]
pub enum Form {
    #[default]
    Simple,
    Invalid,
}

impl text_input::StyleSheet for Theme {
    type Style = Form;
    fn active(&self, style: &Self::Style) -> text_input::Appearance {
        let palette = match self {
            Theme::Dark(p) => p,
            Theme::Light(p) => p,
        };
        let border_radius = BorderRadius::from(18.0);
        let border_width = 1.0;
        match style {
            Form::Simple => text_input::Appearance {
                border_width,
                border_radius,
                icon_color: palette.info,
                background: iced::Background::Color(iced::Color::TRANSPARENT),
                border_color: palette.primary,
            },
            Form::Invalid => text_input::Appearance {
                border_radius,
                border_width,
                icon_color: palette.info,
                background: iced::Background::Color(iced::Color::TRANSPARENT),
                border_color: palette.secondary,
            },
        }
    }

    fn disabled(&self, style: &Self::Style) -> text_input::Appearance {
        text_input::Appearance {
            ..self.active(style)
        }
    }

    fn focused(&self, style: &Self::Style) -> text_input::Appearance {
        text_input::Appearance {
            ..self.active(style)
        }
    }

    fn disabled_color(&self, _style: &Self::Style) -> iced::Color {
        let palette = match self {
            Theme::Dark(p) => p,
            Theme::Light(p) => p,
        };

        palette.primary
    }

    fn placeholder_color(&self, _style: &Self::Style) -> iced::Color {
        let palette = match self {
            Theme::Dark(p) => p,
            Theme::Light(p) => p,
        };

        palette.secondary
    }

    fn value_color(&self, _style: &Self::Style) -> iced::Color {
        let palette = match self {
            Theme::Dark(p) => p,
            Theme::Light(p) => p,
        };

        palette.danger
    }

    fn selection_color(&self, _style: &Self::Style) -> iced::Color {
        let palette = match self {
            Theme::Dark(p) => p,
            Theme::Light(p) => p,
        };

        palette.warn
    }
}

#[derive(Debug, Copy, Clone, Default)]
pub enum Slider {
    #[default]
    Primary,
}

impl slider::StyleSheet for Theme {
    type Style = Slider;
    fn active(&self, style: &Self::Style) -> slider::Appearance {
        let palette = match self {
            Theme::Dark(p) => p,
            Theme::Light(p) => p,
        };
        let handle = slider::Handle {
            shape: slider::HandleShape::Rectangle {
                width: 8,
                border_radius: BorderRadius::from(4.0),
            },
            color: palette.window_background_inverse,
            border_color: palette.window_background_inverse,
            border_width: 1.0,
        };

        match style {
            Slider::Primary => slider::Appearance {
                rail: slider::Rail {
                    colors: (palette.primary, iced::Color::TRANSPARENT),
                    width: 2.0,
                    border_radius: BorderRadius::from(19.0),
                },
                handle,
            },
        }
    }
    fn hovered(&self, style: &Self::Style) -> slider::Appearance {
        let palette = match self {
            Theme::Dark(p) => p,
            Theme::Light(p) => p,
        };
        let handle = slider::Handle {
            shape: slider::HandleShape::Rectangle {
                width: 8,
                border_radius: BorderRadius::from(4.0),
            },
            color: palette.primary,
            border_color: palette.window_background_inverse,
            border_width: 1.0,
        };

        match style {
            Slider::Primary => slider::Appearance {
                rail: slider::Rail {
                    colors: (palette.primary, iced::Color::TRANSPARENT),
                    width: 2.0,
                    border_radius: BorderRadius::from(19.0),
                },
                handle,
            },
        }
    }
    fn dragging(&self, style: &Self::Style) -> slider::Appearance {
        let palette = match self {
            Theme::Dark(p) => p,
            Theme::Light(p) => p,
        };
        let handle = slider::Handle {
            shape: slider::HandleShape::Rectangle {
                width: 8,
                border_radius: BorderRadius::from(4.0),
            },
            color: palette.primary,
            border_color: palette.secondary,
            border_width: 1.0,
        };

        match style {
            Slider::Primary => slider::Appearance {
                rail: slider::Rail {
                    colors: (palette.primary, iced::Color::TRANSPARENT),
                    width: 2.0,
                    border_radius: BorderRadius::from(4.0),
                },
                handle,
            },
        }
    }
}

#[derive(Debug, Copy, Clone, Default)]
pub enum Svg {
    #[default]
    Primary,
    Inverse,
}

impl svg::StyleSheet for Theme {
    type Style = Svg;

    fn appearance(&self, style: &Self::Style) -> svg::Appearance {
        let palette = match self {
            Theme::Dark(p) => p,
            Theme::Light(p) => p,
        };
        match style {
            Svg::Primary => svg::Appearance {
                color: Some(palette.primary),
            },
            Svg::Inverse => svg::Appearance {
                color: Some(palette.window_background_inverse),
            },
        }
    }
}
