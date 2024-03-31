//! -- Copyright (c) 2023 Rina Khasanshin
//! -- Email: hicarus@yandex.ru
//! -- Licensed under the GNU General Public License Version 3.0 (GPL-3.0)

pub mod color;
pub mod components;
pub mod image;
pub mod styles;

pub mod widget {
    pub type Renderer = iced::Renderer;
    pub type Element<'a, Message> = iced::Element<'a, Message, iced::Theme>;
    pub type Container<'a, Message> = iced::widget::Container<'a, Message, iced::Theme>;
    pub type Column<'a, Message> = iced::widget::Column<'a, Message, iced::Theme>;
    pub type Row<'a, Message> = iced::widget::Row<'a, Message, iced::Theme>;
    pub type Button<'a, Message> = iced::widget::Button<'a, Message, iced::Theme>;
    pub type Text<'a> = iced::widget::Text<'a, iced::Theme>;
    pub type TextInput<'a, Message> = iced::widget::TextInput<'a, Message, iced::Theme, Renderer>;
    pub type Tooltip<'a> = iced::widget::Tooltip<'a, iced::Theme>;
    pub type ProgressBar<'a> = iced::widget::ProgressBar<'a, iced::Theme>;
    pub type PickList<'a, T, L, V, Message> =
        iced::widget::PickList<'a, T, L, V, Message, iced::Theme>;
    pub type Scrollable<'a, Message> = iced::widget::Scrollable<'a, Message, iced::Theme>;
    pub type Slider<'a, Message> = iced::widget::Slider<'a, Message, iced::Theme>;
    pub type Svg<'a> = iced::widget::Svg<'a, iced::Theme>;
}
