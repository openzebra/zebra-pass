//! -- Copyright (c) 2024 Rina Khasanshin
//! -- Email: hicarus@yandex.ru
//! -- Licensed under the GNU General Public License Version 3.0 (GPL-3.0)
use iced::widget::{component, Button, Column, Component, Container, Row, Scrollable, Space, Text};
use iced::{Element, Length, Renderer, Theme};

use super::home_nav_bar::LINE_ALFA_CHANNEL;

#[derive(Debug)]
pub struct SelectListField<Data> {
    pub text: String,
    pub value: Data,
}

pub struct SelectList<'a, Message, Data>
where
    Message: Clone + 'a,
{
    on_select: Option<Box<dyn Fn(usize) -> Message + 'a>>,
    field_padding: u16,
    font_size: u16,
    line_gap: u16,
    fields: &'a [SelectListField<Data>],
    text_horizontal_alignmen: iced::alignment::Horizontal,
    selected_index: usize,
}

#[derive(Debug, Clone)]
pub enum Event {
    HandleSelect(usize),
}

impl<'a, Message, Data> SelectList<'a, Message, Data>
where
    Message: Clone,
{
    pub fn from(fields: &'a [SelectListField<Data>]) -> Self {
        let field_padding = 0;
        let font_size = 14;
        let line_gap = 5;
        let selected_index = 0;
        let text_horizontal_alignmen = iced::alignment::Horizontal::Center;

        Self {
            selected_index,
            text_horizontal_alignmen,
            fields,
            line_gap,
            field_padding,
            font_size,
            on_select: None,
        }
    }

    pub fn set_text_horizontal_alignmen(mut self, alignment: iced::alignment::Horizontal) -> Self {
        self.text_horizontal_alignmen = alignment;

        self
    }

    pub fn set_font_size(mut self, amount: u16) -> Self {
        self.font_size = amount;

        self
    }

    pub fn set_line_gap(mut self, amount: u16) -> Self {
        self.line_gap = amount;

        self
    }

    pub fn set_selected_index(mut self, index: usize) -> Self {
        self.selected_index = index;

        self
    }

    pub fn on_select<F>(mut self, callback: F) -> Self
    where
        F: 'a + Fn(usize) -> Message,
    {
        self.on_select = Some(Box::new(callback));

        self
    }

    pub fn set_field_padding(mut self, amount: u16) -> Self {
        self.field_padding = amount;

        self
    }
}

impl<'a, Message, Data> Component<Message, Theme, Renderer> for SelectList<'a, Message, Data>
where
    Message: Clone,
{
    type State = ();
    type Event = Event;

    fn update(&mut self, _state: &mut Self::State, event: Self::Event) -> Option<Message> {
        match event {
            Event::HandleSelect(index) => {
                if let Some(event) = &self.on_select {
                    Some(event(index))
                } else {
                    None
                }
            }
        }
    }

    fn view(
        &self,
        _state: &Self::State,
    ) -> iced::advanced::graphics::core::Element<'_, Self::Event, Theme, Renderer> {
        let fields: Vec<Element<'_, Event>> = self
            .fields
            .iter()
            .enumerate()
            .map(|(i, data)| {
                let hline = zebra_ui::components::line::Linear::new()
                    .height(Length::Fixed(0.5))
                    .width(Length::Fill)
                    .style(if i != self.fields.len() - 1 && self.selected_index != i {
                        zebra_ui::styles::line::line_secondary
                    } else {
                        zebra_ui::styles::line::line_transparent
                    })
                    .alfa(LINE_ALFA_CHANNEL);
                let row = Row::new()
                    .push(Space::new(self.line_gap, 0))
                    .push(hline)
                    .push(Space::new(self.line_gap, 0));

                Column::new()
                    .push(
                        Button::new(
                            Text::new(&data.text)
                                .horizontal_alignment(self.text_horizontal_alignmen)
                                .width(Length::Fill),
                        )
                        .padding(self.field_padding)
                        .style(if self.selected_index == i {
                            zebra_ui::styles::button::primary_rude
                        } else {
                            zebra_ui::styles::button::ref_primary
                        })
                        .width(Length::Fill)
                        .on_press(Event::HandleSelect(i)),
                    )
                    .push(row)
                    .into()
            })
            .collect();
        let ul = Column::with_children(fields).align_items(iced::Alignment::Center);
        let scrolling = Scrollable::new(ul)
            .height(Length::Fill)
            .style(zebra_ui::styles::scrollable::scroll_transparent);

        Container::new(scrolling).into()
    }
}

impl<'a, Message, Data> From<SelectList<'a, Message, Data>> for Element<'a, Message>
where
    Message: 'a + Clone,
    Data: 'a,
{
    fn from(form: SelectList<'a, Message, Data>) -> Self {
        component(form)
    }
}
