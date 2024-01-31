//! -- Copyright (c) 2024 Rina Khasanshin
//! -- Email: hicarus@yandex.ru
//! -- Licensed under the GNU General Public License Version 3.0 (GPL-3.0)
use crate::style::Theme;
use crate::widget::Renderer;

use iced::advanced::layout::{self, Layout};
use iced::advanced::renderer;
use iced::advanced::widget::{self, Widget};
use iced::mouse;
use iced::{Border, Color, Element, Length, Rectangle, Size};

pub struct Line<Theme>
where
    Theme: StyleSheet,
{
    width: Length,
    height: Length,
    style: <Theme as StyleSheet>::Style,
}

impl<'a, Theme> Line<Theme>
where
    Theme: StyleSheet,
{
    pub fn new() -> Self {
        Self {
            width: Length::Fill,
            height: Length::Fill,
            style: <Theme as StyleSheet>::Style::default(),
        }
    }

    /// Sets the style variant of this [`Line`].
    pub fn style(mut self, style: <Theme as StyleSheet>::Style) -> Self {
        self.style = style;
        self
    }

    /// Sets the width of the [`Line`].
    pub fn width(mut self, width: Length) -> Self {
        self.width = width;

        self
    }

    /// Sets the height of the [`Line`].
    pub fn height(mut self, height: Length) -> Self {
        self.height = height;

        self
    }
}

impl<Message, Theme, Renderer> Widget<Message, Theme, Renderer> for Line<Theme>
where
    Renderer: renderer::Renderer,
    Theme: StyleSheet,
{
    fn size(&self) -> Size<Length> {
        Size {
            width: self.width,
            height: self.height,
        }
    }

    fn layout(
        &self,
        _tree: &mut widget::Tree,
        _renderer: &Renderer,
        limits: &layout::Limits,
    ) -> layout::Node {
        layout::atomic(limits, self.width, self.height)
        // layout::Node::new(Size {
        //     width: 0.0,
        //     height: 0.0,
        // })
    }

    fn draw(
        &self,
        _state: &widget::Tree,
        renderer: &mut Renderer,
        theme: &Theme,
        _style: &renderer::Style,
        layout: Layout<'_>,
        _cursor: mouse::Cursor,
        _viewport: &Rectangle,
    ) {
        let custom_style = <Theme as StyleSheet>::appearance(theme, &self.style);

        renderer.fill_quad(
            renderer::Quad {
                bounds: layout.bounds(),
                border: Border {
                    radius: 0.into(),
                    width: 0.0,
                    color: Color::TRANSPARENT,
                },
                shadow: Default::default(),
            },
            custom_style.color,
        );
    }
}

impl<'a, Message, Theme> From<Line<Theme>> for Element<'a, Message, Theme, Renderer>
where
    Message: Clone + 'a,
    Theme: StyleSheet + 'a,
{
    fn from(circular: Line<Theme>) -> Self {
        Self::new(circular)
    }
}

#[derive(Debug, Clone, Copy)]
pub struct Appearance {
    pub color: Color,
}

impl std::default::Default for Appearance {
    fn default() -> Self {
        Self {
            color: Color::BLACK,
        }
    }
}

/// A set of rules that dictate the style of an indicator.
pub trait StyleSheet {
    /// The supported style of the [`StyleSheet`].
    type Style: Default;

    /// Produces the active [`Appearance`] of a indicator.
    fn appearance(&self, style: &Self::Style) -> Appearance;
}

impl StyleSheet for Theme {
    type Style = ();

    fn appearance(&self, _style: &Self::Style) -> Appearance {
        let palette = match self {
            Theme::Dark(p) => p,
            Theme::Light(p) => p,
        };

        Appearance {
            color: palette.window_background_inverse,
        }
    }
}
