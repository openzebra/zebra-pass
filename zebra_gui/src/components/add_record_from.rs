//! -- Copyright (c) 2024 Rina Khasanshin
//! -- Email: hicarus@yandex.ru
//! -- Licensed under the GNU General Public License Version 3.0 (GPL-3.0)
use crate::components::custom_field::CustomFields;
use crate::components::passgen::{PassGenForm, PassGenState};
use crate::rust_i18n::t;
use iced::widget::{
    component, text_editor, Button, Column, Component, Container, Row, Scrollable, Space, Text,
};
use iced::{Element, Length, Renderer, Theme};
use std::sync::{Arc, Mutex};

use super::modal::Modal;
use super::smart_input::SmartInput;
use zebra_lib::core::record;

pub struct AddRecordForm<'a, Message>
where
    Message: Clone,
{
    element: &'a record::Element,
    title: String,
    on_input: Option<Box<dyn Fn(record::Element) -> Message + 'a>>,
    content: text_editor::Content,
    password_modal: bool,
    modal_index_element: usize,
    pass_gen_state: Arc<Mutex<PassGenState>>,
}

#[derive(Debug, Clone)]
pub enum Event {
    HandleReloadPassword,
    HandleSave,
    HandleHidePasswordModal,
    HandleInputFieldValue(usize, String),
    HandleReloadInput(usize),
    HandleSavePassword,
    HandleChangeCustomField(Vec<record::Item>),
    HandleActionNote(text_editor::Action),
}

impl<'a, Message: Clone> AddRecordForm<'a, Message>
where
    Message: Clone,
{
    pub fn from(element: &'a record::Element) -> Self {
        let pass_gen_state = Arc::new(Mutex::new(PassGenState {
            value: String::new(),
            length: 45,
        }));

        Self {
            element,
            title: String::new(),
            on_input: None,
            content: text_editor::Content::new(),
            password_modal: false,
            modal_index_element: 0,
            pass_gen_state,
        }
    }

    pub fn set_title(mut self, title: String) -> Self {
        self.title = title;

        self
    }

    pub fn on_input<F>(mut self, callback: F) -> Self
    where
        F: 'a + Fn(record::Element) -> Message,
    {
        self.on_input = Some(Box::new(callback));

        self
    }
}

impl<'a, Message> Component<Message, Theme, Renderer> for AddRecordForm<'a, Message>
where
    Message: Clone,
{
    type State = ();
    type Event = Event;

    fn update(&mut self, _state: &mut Self::State, event: Self::Event) -> Option<Message> {
        match event {
            Event::HandleHidePasswordModal => {
                self.password_modal = false;

                None
            }
            Event::HandleReloadPassword => {
                self.password_modal = true;

                None
            }
            Event::HandleInputFieldValue(index, value) => {
                if let Some(on_submit) = &self.on_input {
                    let mut new_element = self.element.clone();

                    match new_element.fields.get_mut(index) {
                        Some(el) => {
                            el.value = value;
                            Some(on_submit(new_element))
                        }
                        None => None,
                    }
                } else {
                    None
                }
            }
            Event::HandleActionNote(a) => {
                self.content.perform(a);

                None
            }
            Event::HandleSave => {
                if let Some(on_submit) = &self.on_input {
                    let new_element = self.element.clone();

                    Some(on_submit(new_element))
                } else {
                    None
                }
            }
            Event::HandleSavePassword => {
                match self.pass_gen_state.lock() {
                    Ok(state) => {
                        let mut new_element = self.element.clone();

                        self.password_modal = false;

                        match new_element.fields.get_mut(self.modal_index_element) {
                            Some(el) => el.value = state.value.to_string(),
                            None => return None,
                        };

                        if let Some(on_submit) = &self.on_input {
                            Some(on_submit(new_element))
                        } else {
                            None
                        }
                    }
                    Err(e) => {
                        dbg!(e);
                        // TODO: make error hanlde
                        None
                    }
                }
            }
            Event::HandleChangeCustomField(new_list) => {
                if let Some(on_submit) = &self.on_input {
                    let mut new_element = self.element.clone();

                    new_element.extra_fields = new_list;

                    Some(on_submit(new_element))
                } else {
                    None
                }
            }
            Event::HandleReloadInput(index) => {
                self.modal_index_element = index;
                self.password_modal = true;
                None
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

        let fields: Vec<iced::advanced::graphics::core::Element<'_, Self::Event, Theme, Renderer>> =
            self.element
                .fields
                .iter()
                .enumerate()
                .map(|(index, field)| {
                    let mut input = SmartInput::new()
                        .set_value(&field.value)
                        .padding(INPUT_PADDING)
                        .set_secure(field.hide)
                        .on_input(move |v| Event::HandleInputFieldValue(index, v))
                        .set_placeholder(field.title.clone());

                    if field.reload {
                        input = input.set_reload(Event::HandleReloadInput(index));
                    }

                    input.into()
                })
                .collect();

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

        let custom_fields = CustomFields::new()
            .set_padding(INDENT_HEAD)
            .on_input(Event::HandleChangeCustomField)
            .set_list(&self.element.extra_fields);
        let custom_fields = Container::new(custom_fields);

        let scrol_col = Column::with_children(fields)
            .spacing(8)
            .padding(INDENT_HEAD)
            .width(Length::Fill)
            .align_items(iced::Alignment::Center)
            .push(Space::new(0, INDENT_HEAD))
            .push(custom_fields)
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

        if self.password_modal {
            let close_btn = Button::new(
                zebra_ui::image::close_icon()
                    .style(zebra_ui::styles::svg::primary_hover)
                    .height(30)
                    .width(30),
            )
            .padding(0)
            .style(zebra_ui::styles::button::transparent)
            .on_press(Event::HandleHidePasswordModal);
            let close_btn = Column::new()
                .push(close_btn)
                .width(Length::Fill)
                .align_items(iced::Alignment::End);
            let row_header = Row::new().padding(8).push(close_btn).width(Length::Fill);

            // TODO: remoe unwrap
            let pass_gen = PassGenForm::new(Arc::clone(&self.pass_gen_state))
                .unwrap()
                .height(200);
            let pass_gen = Container::new(pass_gen);
            let save_btn = Button::new(
                Text::new(match self.element.fields.get(self.modal_index_element) {
                    Some(el) => {
                        if el.value.is_empty() {
                            t!("save_password")
                        } else {
                            t!("edit_password")
                        }
                    }
                    None => t!("edit_password"),
                })
                .size(16)
                .horizontal_alignment(iced::alignment::Horizontal::Center),
            )
            .style(zebra_ui::styles::button::outline_primary)
            .padding(8)
            .on_press(Event::HandleSavePassword);
            let main_modal_col = Column::new()
                .push(row_header)
                .push(pass_gen)
                .push(save_btn)
                .push(Space::new(0, 8))
                .padding(8)
                .align_items(iced::Alignment::Center);

            let modal = Container::new(main_modal_col)
                .width(400)
                .style(zebra_ui::styles::container::primary_bordered_modal);
            Modal::new(main_col, modal)
                .on_blur(Event::HandleHidePasswordModal)
                .into()
        } else {
            Container::new(main_col).into()
        }
    }
}

impl<'a, Message> From<AddRecordForm<'a, Message>> for Element<'a, Message>
where
    Message: 'a + Clone,
{
    fn from(form: AddRecordForm<'a, Message>) -> Self {
        component(form)
    }
}
