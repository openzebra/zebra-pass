//! -- Copyright (c) 2024 Rina Khasanshin
//! -- Email: hicarus@yandex.ru
//! -- Licensed under the GNU General Public License Version 3.0 (GPL-3.0)
use iced::widget::{component, Button, Checkbox, Column, Component, Container, Row, Space, Text};
use iced::{Element, Length, Padding, Renderer, Theme};
use rust_i18n::t;

use super::smart_input::SmartInput;
use zebra_lib::core::record::Item;

pub struct CustomFields<'a, Message>
where
    Message: Clone,
{
    on_input: Option<Box<dyn Fn(Vec<Item>) -> Message + 'a>>,
    on_copy: Option<Box<dyn Fn(String) -> Message + 'a>>,
    check_box_secure: bool,
    input_padding: u16,
    container_padding: Padding,
    list: &'a [Item],
    label: String,
}

#[derive(Debug, Clone)]
pub enum Event {
    AddNewField,
    InputCopy(usize),
    InputCustomField((usize, String)),
    RemoveCustomField(usize),
    CheckedSecure(bool),
    InputLabel(String),
}

impl<'a, Message> Default for CustomFields<'a, Message>
where
    Message: Clone,
{
    fn default() -> Self {
        Self::new()
    }
}

impl<'a, Message> CustomFields<'a, Message>
where
    Message: Clone,
{
    pub fn new() -> Self {
        Self {
            input_padding: 8,
            container_padding: Padding::ZERO,
            on_input: None,
            on_copy: None,
            list: &[],
            check_box_secure: false,
            label: String::new(),
        }
    }

    pub fn set_padding<P: Into<Padding>>(mut self, padding: P) -> Self {
        self.container_padding = padding.into();
        self
    }

    pub fn set_list(mut self, list: &'a [Item]) -> Self {
        self.list = list;

        self
    }

    pub fn on_input<F>(mut self, callback: F) -> Self
    where
        F: 'a + Fn(Vec<Item>) -> Message,
    {
        self.on_input = Some(Box::new(callback));

        self
    }

    pub fn on_copy<F>(mut self, callback: F) -> Self
    where
        F: 'a + Fn(String) -> Message,
    {
        self.on_copy = Some(Box::new(callback));

        self
    }
}

impl<'a, Message> Component<Message, Theme, Renderer> for CustomFields<'a, Message>
where
    Message: Clone,
{
    type State = ();
    type Event = Event;

    fn update(&mut self, _state: &mut Self::State, event: Self::Event) -> Option<Message> {
        match event {
            Event::InputCopy(index) => match self.list.get(index) {
                Some(item) => self.on_copy.as_ref().map(|cb| cb(item.value.clone())),
                None => None,
            },
            Event::AddNewField => {
                if let Some(cb) = &self.on_input {
                    let mut new_list = self.list.to_vec();

                    new_list.push(Item {
                        title: self.label.to_string(),
                        value: String::new(),
                        hide: self.check_box_secure,
                        copy: true,
                        reload: false,
                    });

                    Some(cb(new_list))
                } else {
                    None
                }
            }
            Event::InputCustomField((index, value)) => {
                if let Some(cb) = &self.on_input {
                    let mut new_list = self.list.to_vec();

                    match new_list.get_mut(index) {
                        Some(v) => {
                            v.value = value;
                        }
                        None => return None,
                    };

                    Some(cb(new_list))
                } else {
                    None
                }
            }
            Event::CheckedSecure(v) => {
                self.check_box_secure = v;

                None
            }
            Event::RemoveCustomField(index) => {
                if let Some(cb) = &self.on_input {
                    let mut new_list = self.list.to_vec();

                    new_list.remove(index);

                    Some(cb(new_list))
                } else {
                    None
                }
            }
            Event::InputLabel(value) => {
                self.label = value;

                None
            }
        }
    }

    fn view(
        &self,
        _state: &Self::State,
    ) -> iced::advanced::graphics::core::Element<'_, Self::Event, Theme, Renderer> {
        let custom_fields: Vec<
            iced::advanced::graphics::core::Element<'_, Self::Event, Theme, Renderer>,
        > = self
            .list
            .iter()
            .enumerate()
            .map(|(index, field)| {
                let trash_btn = Button::new(
                    zebra_ui::image::trash_icon()
                        .style(zebra_ui::styles::svg::primary_hover)
                        .height(30)
                        .width(30),
                )
                .padding(0)
                .on_press(Event::RemoveCustomField(index))
                .width(30)
                .style(zebra_ui::styles::button::transparent);
                let mut new_field: SmartInput<'_, Event> = SmartInput::new()
                    .set_value(&field.value)
                    .set_label(&field.title)
                    .padding(self.input_padding)
                    .on_input(move |v| Event::InputCustomField((index, v)))
                    .set_secure(field.hide);

                if !field.value.is_empty() {
                    new_field = new_field.set_copy(Event::InputCopy(index));
                }

                let field = Container::new(new_field).width(Length::FillPortion(2));
                let field_row = Row::new()
                    .push(field)
                    .push(Space::new(5, 0))
                    .push(trash_btn)
                    .align_items(iced::Alignment::Center)
                    .width(Length::Fill);

                Container::new(field_row).into()
            })
            .collect();
        let custom_field_col = Column::with_children(custom_fields).spacing(8);

        let add_field_label = Text::new(t!("label_new_field"))
            .size(14)
            .style(zebra_ui::styles::text::muted)
            .width(Length::Fill)
            .horizontal_alignment(iced::alignment::Horizontal::Left);
        let add_btn = Button::new(
            zebra_ui::image::add_icon()
                .style(zebra_ui::styles::svg::primary_hover)
                .height(30)
                .width(30),
        )
        .padding(0)
        .on_press(Event::AddNewField)
        .style(zebra_ui::styles::button::transparent);

        let label_field: SmartInput<'_, Event> = SmartInput::new()
            .set_value(&self.label)
            .set_placeholder(t!("placeholder_label"))
            .padding(self.input_padding)
            .on_submit(Event::AddNewField)
            .on_input(Event::InputLabel);
        let label_field = Container::new(label_field).width(200);
        let secure_checkbox = Checkbox::new(t!("placeholder_password"), self.check_box_secure)
            .on_toggle(Event::CheckedSecure)
            .text_size(14);
        let row_new_item = Row::new()
            .push(add_btn)
            .push(Space::new(8, 0))
            .push(secure_checkbox)
            .push(Space::new(8, 0))
            .push(label_field)
            .align_items(iced::Alignment::Center);
        let col_new_item = Column::new()
            .push(add_field_label)
            .push(Space::new(0, 8))
            .push(row_new_item)
            .push(Space::new(0, 8))
            .push(custom_field_col);

        Container::new(col_new_item)
            .padding(self.container_padding)
            .width(Length::Fill)
            .style(zebra_ui::styles::container::primary_bordered_hover)
            .into()
    }
}

impl<'a, Message> From<CustomFields<'a, Message>> for Element<'a, Message>
where
    Message: 'a + Clone,
{
    fn from(form: CustomFields<'a, Message>) -> Self {
        component(form)
    }
}
