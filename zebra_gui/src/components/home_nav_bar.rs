//! -- Copyright (c) 2024 Rina Khasanshin
//! -- Email: hicarus@yandex.ru
//! -- Licensed under the GNU General Public License Version 3.0 (GPL-3.0)
use iced::Length;
use zebra_ui::widget::*;

const LINE_ALFA_CHANNEL: f32 = 0.4;

pub enum NavRoute {
    Home,
    Gen,
    Settings,
}

pub struct NavBar<Message: Clone> {
    on_home: Option<Message>,
    on_gen: Option<Message>,
    on_settings: Option<Message>,
    route: NavRoute,
}

impl<'a, Message: Clone + 'a> NavBar<Message> {
    pub fn new() -> Self {
        Self {
            route: NavRoute::Home,
            on_home: None,
            on_gen: None,
            on_settings: None,
        }
    }

    pub fn view(&self, content: Container<'a, Message>) -> Container<'a, Message> {
        let header = self.view_header();

        let vline = zebra_ui::components::line::Line::new()
            .width(Length::Fixed(1.0))
            .height(Length::Fill)
            .alfa(LINE_ALFA_CHANNEL)
            .style(zebra_ui::components::line::LineStyleSheet::Secondary);
        let hline = zebra_ui::components::line::Line::new()
            .height(Length::Fixed(1.0))
            .width(Length::Fill)
            .alfa(LINE_ALFA_CHANNEL)
            .style(zebra_ui::components::line::LineStyleSheet::Secondary);

        let left_search_col = Column::new().height(Length::Fill).width(200);
        let main_row = Row::new()
            .push(self.view_left_nav_bar())
            .push(vline.clone())
            .push(left_search_col)
            .push(vline)
            .push(content);
        let main_col = Column::new().push(header).push(hline).push(main_row);

        Container::new(main_col)
            .height(Length::Fill)
            .width(Length::Fill)
    }

    fn view_header(&self) -> Container<'a, Message> {
        let zebra_logo = Container::new(zebra_ui::image::zebra_logo_view()).width(125);
        let header_row = Row::new().push(zebra_logo);

        Container::new(header_row).width(Length::Fill).height(60)
    }

    fn view_left_nav_bar(&self) -> Container<'a, Message> {
        let content_col = Column::new();

        Container::new(content_col).width(60).height(Length::Fill)
    }
}
