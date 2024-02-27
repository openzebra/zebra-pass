//! -- Copyright (c) 2023 Rina Khasanshin
//! -- Email: hicarus@yandex.ru
//! -- Licensed under the GNU General Public License Version 3.0 (GPL-3.0)
use std::sync::{Arc, Mutex};

use crate::{
    gui::{GlobalMessage, Routers},
    rust_i18n::t,
};
use iced::{
    alignment::Horizontal,
    widget::{pick_list, Space},
    Command, Length, Subscription,
};
use zebra_lib::settings::language::Language;
use zebra_lib::{core::core::Core, errors::ZebraErrors};
use zebra_ui::widget::*;

use super::{error::ErrorPage, inverview::Interview, Page};

#[derive(Debug)]
pub struct Locale {
    locales: [Language; 8],
    selected: Option<Language>,
    core: Arc<Mutex<Core>>,
}

#[derive(Debug, Clone, Copy)]
pub enum LocaleMessage {
    Next,
    Selected(zebra_lib::settings::language::Language),
}

impl Page for Locale {
    type Message = LocaleMessage;

    fn new(core: Arc<Mutex<Core>>) -> Result<Self, ZebraErrors> {
        let locales = Language::ALL;
        let selected = Some(
            core.lock()
                .or(Err(ZebraErrors::SyncStateLock))?
                .state
                .settings
                .locale,
        );

        Ok(Self {
            locales,
            selected,
            core,
        })
    }

    fn subscription(&self) -> Subscription<Self::Message> {
        Subscription::none()
    }

    fn update(&mut self, message: LocaleMessage) -> Command<GlobalMessage> {
        match message {
            LocaleMessage::Next => match Interview::new(Arc::clone(&self.core)) {
                Ok(locale) => {
                    let route = Routers::Interview(locale);
                    Command::perform(std::future::ready(1), |_| GlobalMessage::Route(route))
                }
                Err(e) => {
                    let route = Routers::ErrorPage(ErrorPage::from(e.to_string()));
                    Command::perform(std::future::ready(1), |_| GlobalMessage::Route(route))
                }
            },
            LocaleMessage::Selected(lang) => match self.core.lock() {
                Ok(mut core) => {
                    self.selected = Some(lang.clone());

                    rust_i18n::set_locale(&lang.symbol());
                    core.state.settings.locale = lang;
                    core.state_update().unwrap(); // TODO: remove unwrap

                    Command::none()
                }
                Err(e) => {
                    let route = Routers::ErrorPage(ErrorPage::from(e.to_string()));
                    Command::perform(std::future::ready(1), |_| GlobalMessage::Route(route))
                }
            },
        }
    }

    fn view(&self) -> Element<Self::Message> {
        let locale_pick_list = pick_list(
            self.locales.as_slice(),
            self.selected.clone(),
            LocaleMessage::Selected,
        )
        .text_size(20)
        .padding(5)
        .width(220)
        // .on_opened(LocaleMessage::Openned)
        // .on_closed(LocaleMessage::Closed)
        .style(zebra_ui::style::pick_list::PickList::OutlineLight);

        let zebra_print = zebra_ui::image::zebra_print_view();
        let title = Text::new(t!("welcome"))
            .size(34)
            .horizontal_alignment(Horizontal::Center);
        let forward_btn = Button::new(zebra_ui::image::forward_icon().height(50).width(50))
            .padding(0)
            .style(zebra_ui::style::button::Button::Transparent)
            .on_press(LocaleMessage::Next);

        let print_col = Column::new()
            .width(220)
            .height(Length::Fill)
            .push(zebra_print);
        let payload_col = Column::new()
            .width(Length::Fill)
            .height(Length::Fill)
            .align_items(iced::Alignment::Center)
            .padding(50)
            .push(title)
            .push(Space::new(0, 20))
            .push(locale_pick_list)
            .push(Space::new(0, 200))
            .push(forward_btn);
        let row = Row::new()
            .width(Length::Fill)
            .push(print_col)
            .push(payload_col);

        Container::new(row)
            .height(Length::Fill)
            .width(Length::Fill)
            .into()
    }
}
