//! -- Copyright (c) 2024 Rina Khasanshin
//! -- Email: hicarus@yandex.ru
//! -- Licensed under the GNU General Public License Version 3.0 (GPL-3.0)
use crate::rust_i18n::t;
use iced::widget::{
    component, text_editor, Button, Column, Component, Container, Row, Scrollable, Space, Text,
};
use iced::{Element, Length, Renderer, Theme};

use super::smart_input::SmartInput;

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
    HandleActionNote(text_editor::Action),
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
