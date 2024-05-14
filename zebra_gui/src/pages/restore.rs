//! -- Copyright (c) 2023 Rina Khasanshin
//! -- Email: hicarus@yandex.ru
//! -- Licensed under the GNU General Public License Version 3.0 (GPL-3.0)
use std::sync::{Arc, Mutex};
use zebra_lib::{
    bip39::mnemonic::{Language, Mnemonic},
    core::Core,
    errors::ZebraErrors,
};
use zebra_ui::config::PRINT_WIDTH;

use crate::{
    gui::{GlobalMessage, Routers},
    rust_i18n::t,
};

use super::{
    error::ErrorPage,
    options::Options,
    password_setup::{LastRoute, PasswordSetup},
    Page,
};
use iced::{alignment::Horizontal, Command, Length, Subscription};
use iced::{
    keyboard::{self, key::Named},
    Element,
};
use iced::{
    overlay::menu,
    widget::{pick_list, scrollable, text_input, Button, Column, Container, Row, Space, Text},
};

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
    TabPressed(bool),
}

impl Page for Restore {
    type Message = RestoreMessage;

    fn new(core: Arc<Mutex<Core>>) -> Result<Self, ZebraErrors> {
        let counts = [12, 15, 18, 21, 24];
        let count = 24; // number of words
        let err_message = None;
        let words = vec![String::new(); count];
        let dict = Language::English;
        let dicts = [dict];
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
        keyboard::on_key_press(|key_code, modifiers| match (key_code, modifiers) {
            (keyboard::Key::Named(Named::Tab), _) => {
                Some(RestoreMessage::TabPressed(modifiers.shift()))
            }
            _ => None,
        })
    }

    fn update(&mut self, message: Self::Message) -> Command<GlobalMessage> {
        match message {
            RestoreMessage::TabPressed(shift) => {
                if shift {
                    iced::widget::focus_previous()
                } else {
                    iced::widget::focus_next()
                }
            }
            RestoreMessage::Back => match Options::new(Arc::clone(&self.core)) {
                Ok(options) => {
                    let route = Routers::Options(options);
                    Command::perform(std::future::ready(1), |_| GlobalMessage::Route(route))
                }
                Err(e) => {
                    let route = Routers::ErrorPage(ErrorPage::from(e.to_string()));
                    Command::perform(std::future::ready(1), |_| GlobalMessage::Route(route))
                }
            },
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
                    self.err_message = Some(
                        t!("restore_invalid_words_count", count => self.counts[0]).to_string(),
                    );

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
                            self.err_message = Some(t!("invalid_dict_bip39").to_string());
                        }
                        Err(_) => {
                            self.error_indexs[i] = true;
                            self.err_message =
                                Some(t!("not_found_word_in_dict", word => word).to_string());
                        }
                    }
                }

                if self.err_message.is_some() {
                    return Command::none();
                }

                let words_str = words.join(" ");
                match Mnemonic::mnemonic_to_entropy(self.dict, &words_str) {
                    Ok(m) => {
                        let mut password_setup =
                            PasswordSetup::new(Arc::clone(&self.core)).unwrap();

                        password_setup.set_mnemonic(m);
                        password_setup.last_route = LastRoute::Restore;

                        let route = Routers::PasswordSetup(password_setup);
                        Command::perform(std::future::ready(1), |_| GlobalMessage::Route(route))
                    }
                    Err(e) => {
                        self.err_message =
                            Some(t!("secret_phrase_invalid", code => e.to_string()).to_string());
                        Command::none()
                    }
                }
            }
            RestoreMessage::InputChanged((index, value)) => {
                self.err_message = None;
                self.words[index] = value.to_lowercase();

                Command::none()
            }
            RestoreMessage::InputPaste((index, v)) => {
                self.err_message = None;
                let words: Vec<String> =
                    v.to_lowercase().split(' ').map(|s| s.to_string()).collect();

                if words.len() == 1 {
                    self.words[index] = v.clone().to_lowercase();
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
            .width(PRINT_WIDTH)
            .height(Length::Fill)
            .push(zebra_print);
        let title = Text::new(t!("restore_page_title"))
            .size(24)
            .horizontal_alignment(Horizontal::Center);
        let forward_icon = zebra_ui::image::forward_icon().height(50).width(50);
        let back_btn = Button::new(zebra_ui::image::back_icon().height(50).width(50))
            .padding(0)
            .style(zebra_ui::styles::button::transparent)
            .on_press(RestoreMessage::Back);
        let forward_btn = Button::new(forward_icon)
            .padding(0)
            .style(zebra_ui::styles::button::transparent)
            .on_press(RestoreMessage::Next);
        let btns_row = Row::new().push(back_btn).push(forward_btn);
        let error_message = Text::new(self.err_message.clone().unwrap_or_default())
            .size(16)
            .style(zebra_ui::styles::text::danger)
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
        .style(pick_list::Style {
            field: Box::new(zebra_ui::styles::pick_list::primary_field),
            menu: menu::Style {
                list: Box::new(zebra_ui::styles::menu::primary_menu),
                scrollable: Box::new(scrollable::default),
            },
        })
        .width(80);
        let language_pick_list = pick_list(
            self.dicts.as_slice(),
            Some(self.dict),
            RestoreMessage::LanguageSelected,
        )
        .text_size(16)
        .padding(4)
        .style(pick_list::Style {
            field: Box::new(zebra_ui::styles::pick_list::primary_field),
            menu: menu::Style {
                list: Box::new(zebra_ui::styles::menu::primary_menu),
                scrollable: Box::new(scrollable::default),
            },
        })
        .width(150);

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
                                true => zebra_ui::styles::input::danger,
                                false => zebra_ui::styles::input::primary,
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
