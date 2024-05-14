//! -- Copyright (c) 2024 Rina Khasanshin
//! -- Email: hicarus@yandex.ru
//! -- Licensed under the GNU General Public License Version 3.0 (GPL-3.0)
use crate::components::custom_field::CustomFields;
use crate::components::passgen::{PassGenForm, PassGenState};

use chrono::{Local, MappedLocalTime, TimeZone};
use iced::widget::{
    component, text_editor, Button, Column, Component, Container, Row, Scrollable, Space, Text,
};
use iced::{Element, Length, Renderer, Theme};
use rust_i18n::t;
use std::sync::{Arc, Mutex};

use super::modal::Modal;
use super::smart_input::SmartInput;
use zebra_lib::core::record;

const DATA_FORMAT: &str = "%d.%m.%Y %H:%M:%S";

pub struct AddRecordForm<'a, Message>
where
    Message: Clone,
{
    element: &'a record::Element,
    title: String,
    on_input: Option<Box<dyn Fn(record::Element) -> Message + 'a>>,
    on_copy: Option<Box<dyn Fn(String) -> Message + 'a>>,
    on_save: Option<Message>,
    on_edit: Option<Message>,
    content: text_editor::Content,
    read_only: bool,
    password_modal: bool,
    modal_index_element: usize,
    pass_gen_state: Arc<Mutex<PassGenState>>,
}

#[derive(Debug, Clone)]
pub enum Event {
    ReloadPassword,
    HandleSave,
    HandleEdit,
    HandleHidePasswordModal,
    HandleInputNameFieldCopy,
    HandleInputFieldName(String),
    HandleInputFieldValue(usize, String),
    HandleInputFieldCopy(usize),
    HandleInputExtraFieldCopy(String),
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
            length: 42,
        }));

        Self {
            element,
            pass_gen_state,
            read_only: false,
            title: String::new(),
            on_input: None,
            on_copy: None,
            on_save: None,
            on_edit: None,
            content: text_editor::Content::with_text(&element.note),
            password_modal: false,
            modal_index_element: 0,
        }
    }

    pub fn set_save(mut self, msg: Message) -> Self {
        self.on_save = Some(msg);

        self
    }

    pub fn set_edit(mut self, msg: Message) -> Self {
        self.on_edit = Some(msg);

        self
    }

    pub fn set_title(mut self, title: String) -> Self {
        self.title = title;

        self
    }

    pub fn set_read_only(mut self, value: bool) -> Self {
        self.read_only = value;

        self
    }

    pub fn on_copy<F>(mut self, callback: F) -> Self
    where
        F: 'a + Fn(String) -> Message,
    {
        self.on_copy = Some(Box::new(callback));

        self
    }

    pub fn on_input<F>(mut self, callback: F) -> Self
    where
        F: 'a + Fn(record::Element) -> Message,
    {
        self.on_input = Some(Box::new(callback));

        self
    }

    pub fn view_timestamp(&self) -> Option<Container<'a, Event, Theme, Renderer>> {
        const TEXT_SIZE: u16 = 13;
        if self.element.created == 0 {
            return None;
        }

        let timestamp_created = match Local.timestamp_opt(self.element.created, 0) {
            MappedLocalTime::Single(t) => t,
            _ => return None,
        };
        let timestamp_updated = match Local.timestamp_opt(self.element.updated, 0) {
            MappedLocalTime::Single(t) => t,
            _ => return None,
        };
        let created_at_date = format!(
            "{}: {}",
            t!("created_at"),
            timestamp_created.format(DATA_FORMAT)
        );
        let updated_at_date = format!(
            "{}: {}",
            t!("updated_at"),
            timestamp_updated.format(DATA_FORMAT)
        );

        let upadted_at = Text::new(updated_at_date)
            .size(TEXT_SIZE)
            .style(zebra_ui::styles::text::muted);
        let created_at = Text::new(created_at_date)
            .size(TEXT_SIZE)
            .style(zebra_ui::styles::text::muted);

        let col = Column::new()
            .push(created_at)
            .push(upadted_at)
            .width(Length::Fill)
            .align_items(iced::Alignment::Start);

        Some(Container::new(col))
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
            Event::HandleInputFieldName(value) => {
                if let Some(on_submit) = &self.on_input {
                    let mut new_element = self.element.clone();

                    if new_element.created == 0 {
                        new_element.created = Local::now().timestamp();
                    }

                    new_element.updated = Local::now().timestamp();
                    new_element.name = value;

                    Some(on_submit(new_element))
                } else {
                    None
                }
            }
            Event::HandleInputNameFieldCopy => {
                if let Some(on_copy) = &self.on_copy {
                    Some(on_copy(self.element.name.clone()))
                } else {
                    None
                }
            }
            Event::HandleHidePasswordModal => {
                self.password_modal = false;

                None
            }
            Event::ReloadPassword => {
                self.password_modal = true;

                None
            }
            Event::HandleInputFieldValue(index, value) => {
                if let Some(on_submit) = &self.on_input {
                    let mut new_element = self.element.clone();

                    new_element.updated = Local::now().timestamp();

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

                if let Some(on_submit) = &self.on_input {
                    let mut new_element = self.element.clone();

                    new_element.updated = Local::now().timestamp();
                    new_element.note = self.content.text();

                    Some(on_submit(new_element))
                } else {
                    None
                }
            }
            Event::HandleSave => self.on_save.clone(),
            Event::HandleSavePassword => {
                if let Ok(state) = self.pass_gen_state.lock() {
                    let mut new_element = self.element.clone();

                    new_element.updated = Local::now().timestamp();
                    self.password_modal = false;

                    match new_element.fields.get_mut(self.modal_index_element) {
                        Some(el) => {
                            el.value = state.value.to_string();

                            return self.on_input.as_ref().map(|on_input| on_input(new_element));
                        }
                        None => return None,
                    }
                }

                None
            }
            Event::HandleChangeCustomField(new_list) => {
                if let Some(on_submit) = &self.on_input {
                    let mut new_element = self.element.clone();

                    new_element.updated = Local::now().timestamp();
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
            Event::HandleInputFieldCopy(index) => {
                if let Some(on_copy) = &self.on_copy {
                    self.element
                        .fields
                        .get(index)
                        .map(|el| on_copy(el.value.clone()))
                } else {
                    None
                }
            }
            Event::HandleInputExtraFieldCopy(value) => {
                self.on_copy.as_ref().map(|on_copy| on_copy(value.clone()))
            }
            Event::HandleEdit => self.on_edit.clone(),
        }
    }

    fn view(
        &self,
        _state: &Self::State,
    ) -> iced::advanced::graphics::core::Element<'_, Self::Event, Theme, Renderer> {
        const INPUT_PADDING: u16 = 12;
        const INDENT_HEAD: u16 = 16;
        const ITEM_SPACING: u16 = 8;

        let title = Text::new(&self.title)
            .size(24)
            .width(Length::Fill)
            .horizontal_alignment(iced::alignment::Horizontal::Left);
        let save_button = Button::new(
            Text::new(if self.read_only {
                t!("edit_record")
            } else {
                t!("save_record")
            })
            .size(16),
        )
        .style(zebra_ui::styles::button::outline_primary)
        .on_press_maybe(if !self.element.name.is_empty() {
            Some(if self.read_only {
                Event::HandleEdit
            } else {
                Event::HandleSave
            })
        } else {
            None
        });
        let head_row = Row::new()
            .push(Space::new(INDENT_HEAD, 0))
            .push(title)
            .push(save_button)
            .push(Space::new(INDENT_HEAD, 0))
            .align_items(iced::Alignment::Center);

        let mut name_field = SmartInput::new()
            .set_value(&self.element.name)
            .padding(INPUT_PADDING)
            .on_input(Event::HandleInputFieldName)
            .set_placeholder(t!("placeholder_name"));

        if !self.element.name.is_empty() {
            name_field = name_field.set_copy(Event::HandleInputNameFieldCopy);
        }

        let mut fields: Vec<
            iced::advanced::graphics::core::Element<'_, Self::Event, Theme, Renderer>,
        > = self
            .element
            .fields
            .iter()
            .filter(|&x| !self.read_only || !x.value.is_empty())
            .enumerate()
            .map(|(index, field)| {
                let mut input = SmartInput::new()
                    .set_value(&field.value)
                    .padding(INPUT_PADDING)
                    .set_secure(field.hide)
                    .on_input(move |v| Event::HandleInputFieldValue(index, v))
                    .set_placeholder(field.title.clone().into());

                if field.reload && !self.read_only {
                    input = input.set_reload(Event::HandleReloadInput(index));
                }

                if field.copy && !field.value.is_empty() {
                    input = input.set_copy(Event::HandleInputFieldCopy(index));
                }

                input.into()
            })
            .collect();

        fields.insert(0, name_field.into());

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

        let custom_fields = if self.read_only {
            let custom_fields: Vec<
                iced::advanced::graphics::core::Element<'_, Self::Event, Theme, Renderer>,
            > = self
                .element
                .extra_fields
                .iter()
                .map(|field| {
                    let mut new_field: SmartInput<'_, Event> = SmartInput::new()
                        .set_value(&field.value)
                        .set_label(&field.title)
                        // a hack event because wayland has issue with cusor
                        .on_input(Event::HandleInputExtraFieldCopy)
                        .padding(5)
                        .set_secure(field.hide);

                    if !field.value.is_empty() {
                        new_field = new_field
                            .set_copy(Event::HandleInputExtraFieldCopy(field.value.clone()));
                    }

                    new_field.into()
                })
                .collect();

            Container::new(
                Column::with_children(custom_fields)
                    .width(Length::Fill)
                    .spacing(ITEM_SPACING),
            )
        } else {
            let custom_fields = CustomFields::new()
                .set_padding(INDENT_HEAD)
                .on_input(Event::HandleChangeCustomField)
                .set_list(&self.element.extra_fields);

            Container::new(custom_fields)
        };

        let scrol_col = Column::with_children(fields)
            .spacing(ITEM_SPACING)
            .padding(INDENT_HEAD)
            .width(Length::Fill)
            .align_items(iced::Alignment::Center)
            .push(Space::new(0, INDENT_HEAD))
            .push(custom_fields)
            .push(Space::new(0, INDENT_HEAD))
            .push_maybe(if self.read_only && self.element.note.is_empty() {
                None
            } else {
                Some(note_label)
            })
            .push_maybe(if self.read_only && self.element.note.is_empty() {
                None
            } else {
                Some(notes)
            })
            .push_maybe(self.view_timestamp())
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

        if self.password_modal && !self.read_only {
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

            let pass_gen = match PassGenForm::new(Arc::clone(&self.pass_gen_state)) {
                Ok(pass) => pass.height(200),
                Err(e) => {
                    let error = Text::new(e.to_string())
                        .style(zebra_ui::styles::text::danger)
                        .size(24);

                    return Container::new(error).into();
                }
            };
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
                .size(ITEM_SPACING * 2)
                .horizontal_alignment(iced::alignment::Horizontal::Center),
            )
            .style(zebra_ui::styles::button::outline_primary)
            .padding(ITEM_SPACING)
            .on_press(Event::HandleSavePassword);

            let main_modal_col = Column::new()
                .push(row_header)
                .push(pass_gen)
                .push(save_btn)
                .push(Space::new(0, ITEM_SPACING))
                .padding(ITEM_SPACING)
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
