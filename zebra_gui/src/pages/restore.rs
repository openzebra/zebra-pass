//! -- Copyright (c) 2023 Rina Khasanshin
//! -- Email: hicarus@yandex.ru
//! -- Licensed under the GNU General Public License Version 3.0 (GPL-3.0)
use std::sync::{Arc, Mutex};
use zebra_lib::{
    bip39::mnemonic::{Language, Mnemonic},
    core::core::Core,
    errors::ZebraErrors,
};

use crate::{
    gui::{GlobalMessage, Routers},
    rust_i18n::t,
};

use super::{options::Options, Page};
use iced::{
    alignment::Horizontal,
    widget::{pick_list, text_input, Space},
    Command, Length, Subscription,
};
use zebra_ui::widget::*;

#[derive(Debug)]
pub struct Restore {
    pub count: usize,
    pub counts: [usize; 5],
    pub right_words: bool,
    pub err_message: Option<String>,
    words: Vec<String>,
    core: Arc<Mutex<Core>>,
}

#[derive(Debug, Clone)]
pub enum RestoreMessage {
    Back,
    Next,
    InputChanged((usize, String)),
    InputPaste(String),
    CountSelected(usize),
}

impl Page for Restore {
    type Message = RestoreMessage;

    fn new(core: Arc<Mutex<Core>>) -> Result<Self, ZebraErrors> {
        let words = vec![String::new(); 24];
        let counts = [12, 15, 18, 21, 24];
        let count = 24; // number of words
        let right_words = false;
        let err_message = None;

        Ok(Self {
            core,
            err_message,
            right_words,
            words,
            count,
            counts,
        })
    }

    fn subscription(&self) -> Subscription<Self::Message> {
        Subscription::none()
    }

    fn update(&mut self, message: Self::Message) -> Command<GlobalMessage> {
        match message {
            RestoreMessage::Back => {
                // TODO: remove unwrap!
                let options = Options::new(Arc::clone(&self.core)).unwrap();
                let route = Routers::Options(options);
                Command::perform(std::future::ready(1), |_| GlobalMessage::Route(route))
            }
            RestoreMessage::Next => Command::none(),
            RestoreMessage::InputChanged((index, value)) => {
                self.words[index] = value;
                let words = self.words.join(" ");

                match Mnemonic::mnemonic_to_entropy(Language::English, &words) {
                    Ok(_) => {
                        self.right_words = true;
                    }
                    Err(e) => {
                        dbg!(e);
                        self.right_words = false;
                        // TODO: make error hanlder!
                    }
                };
                Command::none()
            }
            RestoreMessage::InputPaste(v) => {
                match Mnemonic::mnemonic_to_entropy(Language::English, &v) {
                    Ok(m) => {
                        self.words = m.get_vec().iter().map(|s| s.to_string()).collect();
                        self.right_words = true;
                    }
                    Err(e) => {
                        dbg!(e);
                        self.right_words = false;
                        // TODO: make error hanlder!
                    }
                };

                Command::none()
            }
            RestoreMessage::CountSelected(count) => {
                self.count = count;

                Command::none()
            }
        }
    }

    fn view(&self) -> Element<Self::Message> {
        let zebra_print = zebra_ui::image::zebra_print_view();
        let print_col = Column::new()
            .width(220)
            .height(Length::Fill)
            .push(zebra_print);
        let title = Text::new(t!("restore_page_title"))
            .size(34)
            .horizontal_alignment(Horizontal::Center);
        let forward_icon =
            zebra_ui::image::forward_icon()
                .height(50)
                .width(50)
                .style(if self.right_words {
                    zebra_ui::style::svg::Svg::Primary
                } else {
                    zebra_ui::style::svg::Svg::PrimaryDisabled
                });
        let back_btn = Button::new(zebra_ui::image::back_icon().height(50).width(50))
            .padding(0)
            .style(zebra_ui::style::button::Button::Transparent)
            .on_press(RestoreMessage::Back);
        let forward_btn = Button::new(forward_icon)
            .padding(0)
            .style(zebra_ui::style::button::Button::Transparent)
            .on_press_maybe(if self.right_words {
                Some(RestoreMessage::Next)
            } else {
                None
            });
        let btns_row = Row::new().push(back_btn).push(forward_btn);
        let content_col = Column::new()
            .push(title)
            .width(Length::Fill)
            .height(Length::Fill)
            .align_items(iced::Alignment::Center)
            .push(Space::new(0, 15))
            .push(self.view_content())
            .push(btns_row)
            .padding(10);
        let row = Row::new()
            .width(Length::Fill)
            .push(print_col)
            .push(content_col);

        Container::new(row)
            .height(Length::Fill)
            .width(Length::Fill)
            .into()
    }
}

impl Restore {
    pub fn view_top_row(&self) -> Row<'_, RestoreMessage> {
        let count_pick_list = pick_list(
            self.counts.as_slice(),
            Some(self.count),
            RestoreMessage::CountSelected,
        )
        .text_size(16)
        .padding(4)
        .width(80)
        .style(zebra_ui::style::pick_list::PickList::OutlineLight);

        Row::new().push(count_pick_list)
    }
    pub fn view_content(&self) -> Column<'_, RestoreMessage> {
        const CHUNKS: usize = 4;
        let words_row: Vec<Element<'_, RestoreMessage>> = self
            .words
            .chunks(CHUNKS)
            .enumerate()
            .map(|(index, chunk)| {
                let words_chunk: Vec<Element<'_, RestoreMessage>> = chunk
                    .iter()
                    .enumerate()
                    .map(|(chunk_index, w)| {
                        let placeholder = format!("#{}", (index * CHUNKS) + chunk_index + 1);
                        text_input(&placeholder, w)
                            .size(14)
                            .width(90)
                            .style(zebra_ui::style::text_input::TextInput::Primary)
                            .on_input(move |v| {
                                RestoreMessage::InputChanged(((index * CHUNKS) + chunk_index, v))
                            })
                            .on_paste(RestoreMessage::InputPaste)
                            .into()
                    })
                    .collect();
                Row::with_children(words_chunk).spacing(5).into()
            })
            .collect();
        Column::with_children(words_row)
            .spacing(5)
            .width(Length::Fill)
            .height(Length::Fill)
            .align_items(iced::Alignment::Center)
    }
}
