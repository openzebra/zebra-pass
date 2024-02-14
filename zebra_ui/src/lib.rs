//! -- Copyright (c) 2023 Rina Khasanshin
//! -- Email: hicarus@yandex.ru
//! -- Licensed under the GNU General Public License Version 3.0 (GPL-3.0)

pub mod color;
pub mod components;
pub mod image;
pub mod style;

pub mod widget {
    pub type Renderer = iced::Renderer;
    pub type Element<'a, Message> = iced::Element<'a, Message, crate::style::Theme>;
    pub type Container<'a, Message> = iced::widget::Container<'a, Message, crate::style::Theme>;
    pub type Column<'a, Message> = iced::widget::Column<'a, Message, crate::style::Theme>;
    pub type Row<'a, Message> = iced::widget::Row<'a, Message, crate::style::Theme>;
    pub type Button<'a, Message> = iced::widget::Button<'a, Message, crate::style::Theme>;
    pub type Text<'a> = iced::widget::Text<'a, crate::style::Theme>;
    pub type TextInput<'a, Message> =
        iced::widget::TextInput<'a, Message, crate::style::Theme, Renderer>;
    pub type Tooltip<'a> = iced::widget::Tooltip<'a, crate::style::Theme>;
    pub type ProgressBar = iced::widget::ProgressBar<crate::style::Theme>;
    pub type PickList<'a, Message> = iced::widget::PickList<'a, Message, crate::style::Theme>;
    pub type Scrollable<'a, Message> = iced::widget::Scrollable<'a, Message, crate::style::Theme>;
    pub type Slider<'a, Message> = iced::widget::Slider<'a, Message, crate::style::Theme>;
    pub type Svg = iced::widget::Svg<crate::style::Theme>;
}
