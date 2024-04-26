//! -- Copyright (c) 2024 Rina Khasanshin
//! -- Email: hicarus@yandex.ru
//! -- Licensed under the GNU General Public License Version 3.0 (GPL-3.0)
use crate::rust_i18n::t;
use iced::widget::{
    component, text_editor, Button, Checkbox, Column, Component, Container, Row, Scrollable, Space,
    Text,
};
use iced::{Element, Length, Renderer, Theme};

use super::smart_input::SmartInput;

pub struct AdditionField {
    value: String,
    secure: bool,
}

pub struct AddLogin<'a, Message>
where
    Message: Clone,
{
    title: String,
    on_submit: Option<Box<dyn Fn(String) -> Message + 'a>>,
    name: String,
    username: String,
    email: String,
    password: String,
    domain: String,
    content: text_editor::Content,
    addition_fields: Vec<AdditionField>,
    addition_check_box_secure: bool,
}

#[derive(Debug, Clone)]
pub enum Event {
    HandleReloadPassword,
    HandleSave,
    HandleInputName(String),
    HandleInputUserName(String),
    HandleInputEmail(String),
    HandleInputPassword(String),
    HandleInputDomain(String),
    HandleInputCustomField((usize, String)),
    HandleActionNote(text_editor::Action),
    HandleCheckedSecure(bool),
    HandleAddNewField,
}

impl<'a, Message: Clone> AddLogin<'a, Message>
where
    Message: Clone,
{
    pub fn new() -> Self {
        Self {
            title: String::new(),
            on_submit: None,
            name: String::new(),
            username: String::new(),
            email: String::new(),
            password: String::new(),
            domain: String::new(),
            content: text_editor::Content::new(),
            addition_fields: Vec::new(),
            addition_check_box_secure: false,
        }
    }

    pub fn set_title(mut self, title: String) -> Self {
        self.title = title;

        self
    }
}

impl<'a, Message> Component<Message, Theme, Renderer> for AddLogin<'a, Message>
where
    Message: Clone,
{
    type State = ();
    type Event = Event;

    fn update(&mut self, _state: &mut Self::State, event: Self::Event) -> Option<Message> {
        match event {
            Event::HandleReloadPassword => {
                dbg!("reloaded");

                None
            }
            Event::HandleInputName(v) => {
                self.name = v;
                None
            }
            Event::HandleInputUserName(v) => {
                self.username = v;
                None
            }
            Event::HandleInputEmail(v) => {
                self.email = v;
                None
            }
            Event::HandleInputDomain(v) => {
                self.domain = v;
                None
            }
            Event::HandleInputPassword(v) => {
                self.password = v;
                None
            }
            Event::HandleActionNote(a) => {
                self.content.perform(a);

                None
            }
            Event::HandleSave => None,
            Event::HandleAddNewField => {
                self.addition_fields.push(AdditionField {
                    value: String::new(),
                    secure: self.addition_check_box_secure,
                });

                None
            }
            Event::HandleCheckedSecure(v) => {
                self.addition_check_box_secure = v;
                None
            }
            Event::HandleInputCustomField((index, value)) => {
                match self.addition_fields.get_mut(index) {
                    Some(v) => {
                        v.value = value;

                        return None;
                    }
                    None => return None,
                };
            }
        }
    }

    fn view(
        &self,
        _state: &Self::State,
    ) -> iced::advanced::graphics::core::Element<'_, Self::Event, Theme, Renderer> {
        const INPUT_PADDING: u16 = 12;
        const INDENT_HEAD: u16 = 16;

        let title = Text::new(&self.title)
            .size(24)
            .width(Length::Fill)
            .horizontal_alignment(iced::alignment::Horizontal::Left);
        let save_button = Button::new(Text::new(t!("save_record")).size(16))
            .style(zebra_ui::styles::button::outline_primary)
            .on_press(Event::HandleSave);
        let head_row = Row::new()
            .push(Space::new(INDENT_HEAD, 0))
            .push(title)
            .push(save_button)
            .push(Space::new(INDENT_HEAD, 0))
            .align_items(iced::Alignment::Center);

        let name_input = SmartInput::new()
            .set_value(&self.name)
            .padding(INPUT_PADDING)
            .on_input(Event::HandleInputName)
            .set_placeholder(t!("placeholder_name"));
        let name_input = Container::new(name_input);

        let domain_input = SmartInput::new()
            .set_value(&self.domain)
            .padding(INPUT_PADDING)
            .on_input(Event::HandleInputDomain)
            .set_placeholder(t!("placeholder_domain"));
        let domain_input = Container::new(domain_input);

        let username_input = SmartInput::new()
            .set_value(&self.username)
            .padding(INPUT_PADDING)
            .on_input(Event::HandleInputUserName)
            .set_placeholder(t!("placeholder_username"));
        let username_input = Container::new(username_input);

        let email_input = SmartInput::new()
            .set_value(&self.email)
            .padding(INPUT_PADDING)
            .on_input(Event::HandleInputEmail)
            .set_placeholder(t!("placeholder_username"));
        let email_input = Container::new(email_input);

        let password_input = SmartInput::new()
            .set_value(&self.password)
            .padding(INPUT_PADDING)
            .set_secure(true)
            .set_reload(Event::HandleReloadPassword)
            .on_input(Event::HandleInputPassword)
            .set_placeholder(t!("placeholder_password"));
        let password_input = Container::new(password_input);

        let note_label = Text::new(t!("label_notes"))
            .size(14)
            .style(zebra_ui::styles::text::muted)
            .width(Length::Fill)
            .horizontal_alignment(iced::alignment::Horizontal::Left);
        let notes = text_editor(&self.content)
            .height(120)
            .padding(INPUT_PADDING)
            .style(zebra_ui::styles::text_editor::primary)
            .on_action(Event::HandleActionNote);

        let custom_fields_label = Text::new(t!("custom_fields"))
            .size(14)
            .style(zebra_ui::styles::text::muted)
            .width(Length::Fill)
            .horizontal_alignment(iced::alignment::Horizontal::Left);
        let custom_fields: Vec<
            iced::advanced::graphics::core::Element<'_, Self::Event, Theme, Renderer>,
        > = self
            .addition_fields
            .iter()
            .enumerate()
            .map(|(index, field)| {
                let new_field: SmartInput<'_, Event> = SmartInput::new()
                    .set_value(&field.value)
                    .padding(INPUT_PADDING)
                    .on_input(move |v| Event::HandleInputCustomField((index, v)))
                    .set_secure(field.secure);

                Container::new(new_field).into()
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
        .on_press(Event::HandleAddNewField)
        .style(zebra_ui::styles::button::transparent);
        let secure_checkbox =
            Checkbox::new(t!("placeholder_password"), self.addition_check_box_secure)
                .on_toggle(Event::HandleCheckedSecure)
                .text_size(14);
        let row_new_item = Row::new()
            .push(add_btn)
            .push(Space::new(8, 0))
            .push(secure_checkbox)
            .align_items(iced::Alignment::Center);
        let col_new_item = Column::new()
            .push(add_field_label)
            .push(Space::new(0, 8))
            .push(row_new_item)
            .push(Space::new(0, 8))
            .push(custom_fields_label)
            .push(custom_field_col);
        let new_item_container = Container::new(col_new_item)
            .padding(INDENT_HEAD)
            .width(Length::Fill)
            .style(zebra_ui::styles::container::primary_bordered_hover);

        let scrol_col = Column::new()
            .spacing(8)
            .padding(INDENT_HEAD)
            .width(Length::Fill)
            .align_items(iced::Alignment::Center)
            .push(name_input)
            .push(email_input)
            .push(domain_input)
            .push(username_input)
            .push(password_input)
            .push(Space::new(0, INDENT_HEAD))
            .push(new_item_container)
            .push(Space::new(0, INDENT_HEAD))
            .push(note_label)
            .push(notes)
            .push(Space::new(0, INDENT_HEAD));
        let scrolling = Scrollable::new(scrol_col)
            .height(Length::Fill)
            .style(zebra_ui::styles::scrollable::scroll_transparent);
        let main_col = Column::new()
            .width(Length::Fill)
            .align_items(iced::Alignment::Center)
            .push(Space::new(0, INDENT_HEAD))
            .push(head_row)
            .push(Space::new(0, INDENT_HEAD))
            .push(scrolling);

        Container::new(main_col).into()
    }
}

impl<'a, Message> From<AddLogin<'a, Message>> for Element<'a, Message>
where
    Message: 'a + Clone,
{
    fn from(form: AddLogin<'a, Message>) -> Self {
        component(form)
    }
}
