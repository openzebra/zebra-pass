//! -- Copyright (c) 2024 Rina Khasanshin
//! -- Email: hicarus@yandex.ru
//! -- Licensed under the GNU General Public License Version 3.0 (GPL-3.0)
use iced::advanced::layout;
use iced::advanced::renderer;
use iced::advanced::widget::tree::Tree;
use iced::advanced::{self, Layout, Widget};
use iced::mouse;
use iced::Element;
use iced::Theme;
use iced::{Background, Color, Length, Rectangle, Size};

pub struct Linear<'a, Theme> {
    width: Length,
    height: Length,
    style: Style<'a, Theme>,
    alfa: f32,
}

impl<'a, Theme> Default for Linear<'a, Theme>
where
    Theme: DefaultStyle + 'a,
{
    fn default() -> Self {
        Self::new()
    }
}

impl<'a, Theme> Linear<'a, Theme> {
    /// Creates a new [`Linear`] with the given content.
    pub fn new() -> Self
    where
        Theme: DefaultStyle + 'a,
    {
        Linear {
            width: Length::Fixed(100.0),
            height: Length::Fixed(4.0),
            style: Box::new(Theme::default_style),
            alfa: 1.0,
        }
    }

    /// Sets the width of the [`Linear`].
    pub fn width(mut self, width: impl Into<Length>) -> Self {
        self.width = width.into();
        self
    }

    /// Sets the height of the [`Linear`].
    pub fn height(mut self, height: impl Into<Length>) -> Self {
        self.height = height.into();
        self
    }

    pub fn style(mut self, style: impl Fn(&Theme) -> Appearance + 'a) -> Self {
        self.style = Box::new(style);
        self
    }

    pub fn alfa(mut self, alfa: f32) -> Self {
        self.alfa = alfa;

        self
    }
}

impl<'a, Message, Theme, Renderer> Widget<Message, Theme, Renderer> for Linear<'a, Theme>
where
    Message: Clone + 'a,
    Renderer: advanced::Renderer + 'a,
{
    fn size(&self) -> Size<Length> {
        Size {
            width: self.width,
            height: self.height,
        }
    }

    fn layout(
        &self,
        _tree: &mut Tree,
        _renderer: &Renderer,
        limits: &layout::Limits,
    ) -> layout::Node {
        layout::atomic(limits, self.width, self.height)
    }

    fn draw(
        &self,
        _tree: &Tree,
        renderer: &mut Renderer,
        theme: &Theme,
        _style: &renderer::Style,
        layout: Layout<'_>,
        _cursor: mouse::Cursor,
        _viewport: &Rectangle,
    ) {
        let bounds = layout.bounds();
        let mut styling = (self.style)(theme);

        styling.bar_color.a = self.alfa;

        renderer.fill_quad(
            renderer::Quad {
                bounds: Rectangle {
                    x: bounds.x,
                    y: bounds.y,
                    width: bounds.width,
                    height: bounds.height,
                },
                ..renderer::Quad::default()
            },
            Background::Color(styling.bar_color),
        );
    }
}

impl<'a, Message, Theme> From<Linear<'a, Theme>> for Element<'a, Message, Theme>
where
    Message: Clone + 'a,
    Theme: 'a,
{
    fn from(button: Linear<'a, Theme>) -> Self {
        Self::new(button)
    }
}

pub type Style<'a, Theme> = Box<dyn Fn(&Theme) -> Appearance + 'a>;

#[derive(Debug, Clone, Copy)]
pub struct Appearance {
    pub bar_color: Color,
}

impl std::default::Default for Appearance {
    fn default() -> Self {
        Self {
            bar_color: Color::BLACK,
        }
    }
}

pub trait DefaultStyle {
    fn default_style(&self) -> Appearance;
}

impl DefaultStyle for Theme {
    fn default_style(&self) -> Appearance {
        crate::styles::line::line_inverse(self)
    }
}

impl DefaultStyle for Appearance {
    fn default_style(&self) -> Appearance {
        *self
    }
}
