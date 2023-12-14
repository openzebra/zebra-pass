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
    pub dicts: [Language; 1],
    pub dict: Language,
    pub err_message: Option<String>,
    pub error_indexs: [bool; 24],
    words: Vec<String>,
    core: Arc<Mutex<Core>>,
}

#[derive(Debug, Clone)]
pub enum RestoreMessage {
    Back,
    Next,
    InputChanged((usize, String)),
    InputPaste((usize, String)),
    CountSelected(usize),
    LanguageSelected(Language),
}

impl Page for Restore {
    type Message = RestoreMessage;

    fn new(core: Arc<Mutex<Core>>) -> Result<Self, ZebraErrors> {
        let counts = [12, 15, 18, 21, 24];
        let count = 24; // number of words
        let err_message = None;
        let words = vec![String::new(); count];
        let dict = Language::English;
        let dicts = [dict.clone()];
        let error_indexs = [false; 24];

        Ok(Self {
            core,
            error_indexs,
            dicts,
            dict,
            err_message,
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
            RestoreMessage::Next => {
                self.error_indexs = [false; 24];
                self.err_message = None;

                let words: Vec<String> = self
                    .words
                    .iter()
                    .filter_map(|s| {
                        if s.is_empty() {
                            None
                        } else {
                            Some(s.to_string())
                        }
                    })
                    .collect();

                if words.len() < self.counts[0] {
                    self.err_message =
                        Some(t!("restore_invalid_words_count", count => self.counts[0]));

                    return Command::none();
                }

                let current_dict = self.dict;

                for (i, word) in self.words.iter().enumerate() {
                    match Language::find_out_dict_by_word(word) {
                        Ok(dict) => {
                            if dict == current_dict {
                                continue;
                            }

                            self.error_indexs[i] = true;
                            self.err_message = Some(t!("invalid_dict_bip39"));
                        }
                        Err(_) => {
                            self.error_indexs[i] = true;
                            self.err_message = Some(t!("not_found_word_in_dict", word => word));
                        }
                    }
                }

                if self.err_message.is_some() {
                    return Command::none();
                }

                let words_str = words.join(" ");
                match Mnemonic::mnemonic_to_entropy(self.dict, &words_str) {
                    Ok(_m) => {
                        //TODO: make it workds route
                        return Command::none();
                    }
                    Err(e) => {
                        self.err_message = Some(t!("secret_phrase_invalid", code => e.to_string()));
                        return Command::none();
                    }
                };
            }
            RestoreMessage::InputChanged((index, value)) => {
                self.err_message = None;
                self.words[index] = value;

                Command::none()
            }
            RestoreMessage::InputPaste((index, v)) => {
                self.err_message = None;
                let words: Vec<String> = v.split(" ").map(|s| s.to_string()).collect();

                if words.len() == 1 {
                    self.words[index] = v.clone();
                }

                if let Some(word) = words.first() {
                    match Language::find_out_dict_by_word(word) {
                        Ok(l) => {
                            self.dict = l;
                        }
                        Err(_) => {
                            return Command::none();
                        }
                    }
                } else {
                    return Command::none();
                }

                if words.len() >= self.counts[0] {
                    self.words = words;
                }

                Command::none()
            }
            RestoreMessage::CountSelected(count) => {
                self.count = count;

                if self.count > self.words.len() {
                    let need_add = self.count - self.words.len();

                    self.words.extend(vec![String::new(); need_add])
                } else {
                    self.words.truncate(count);
                }

                Command::none()
            }
            RestoreMessage::LanguageSelected(lang) => {
                self.dict = lang;

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
        let forward_icon = zebra_ui::image::forward_icon()
            .height(50)
            .width(50)
            .style(zebra_ui::style::svg::Svg::Primary);
        let back_btn = Button::new(zebra_ui::image::back_icon().height(50).width(50))
            .padding(0)
            .style(zebra_ui::style::button::Button::Transparent)
            .on_press(RestoreMessage::Back);
        let forward_btn = Button::new(forward_icon)
            .padding(0)
            .style(zebra_ui::style::button::Button::Transparent)
            .on_press(RestoreMessage::Next);
        let btns_row = Row::new().push(back_btn).push(forward_btn);
        let error_message = Text::new(self.err_message.clone().unwrap_or(String::new()))
            .size(16)
            .style(zebra_ui::style::text::Text::Dabger)
            .horizontal_alignment(Horizontal::Center);
        let content_col = Column::new()
            .width(Length::Fill)
            .height(Length::Fill)
            .align_items(iced::Alignment::Center)
            .push(title)
            .push(Space::new(0, 20))
            .push(self.view_top_row())
            .push(Space::new(0, 20))
            .push(self.view_content())
            .push(error_message)
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
        let language_pick_list = pick_list(
            self.dicts.as_slice(),
            Some(self.dict),
            RestoreMessage::LanguageSelected,
        )
        .text_size(16)
        .padding(4)
        .width(150)
        .style(zebra_ui::style::pick_list::PickList::OutlineLight);

        Row::new()
            .push(count_pick_list)
            .push(language_pick_list)
            .spacing(10)
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
                        let element_index = (index * CHUNKS) + chunk_index;
                        text_input(&placeholder, w)
                            .size(14)
                            .width(90)
                            .style(match self.error_indexs[element_index] {
                                true => zebra_ui::style::text_input::TextInput::Danger,
                                false => zebra_ui::style::text_input::TextInput::Primary,
                            })
                            .on_input(move |v| RestoreMessage::InputChanged((element_index, v)))
                            .on_paste(move |v| RestoreMessage::InputPaste((element_index, v)))
                            .into()
                    })
                    .collect();
                Row::with_children(words_chunk).spacing(5).into()
            })
            .collect();
        Column::with_children(words_row)
            .spacing(5)
            .height(220)
            .width(Length::Fill)
            .align_items(iced::Alignment::Center)
    }
}
