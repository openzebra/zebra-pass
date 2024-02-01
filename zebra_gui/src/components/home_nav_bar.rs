//! -- Copyright (c) 2024 Rina Khasanshin
//! -- Email: hicarus@yandex.ru
//! -- Licensed under the GNU General Public License Version 3.0 (GPL-3.0)
use iced::{widget::Space, Length};
use zebra_ui::widget::*;

pub const LINE_ALFA_CHANNEL: f32 = 0.4;

#[derive(PartialEq)]
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

    pub fn set_route(mut self, route: NavRoute) -> Self {
        self.route = route;

        self
    }

    pub fn on_home(mut self, msg: Message) -> Self {
        self.on_home = Some(msg);

        self
    }

    pub fn on_gen(mut self, msg: Message) -> Self {
        self.on_gen = Some(msg);

        self
    }

    pub fn on_settings(mut self, msg: Message) -> Self {
        self.on_settings = Some(msg);

        self
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

        let main_row = Row::new()
            .push(self.view_left_nav_bar())
            .push(vline.clone())
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
        let content_col = Column::new()
            .align_items(iced::Alignment::Center)
            .push(Space::new(Length::Fill, 60))
            .push(self.vew_home_btn())
            .push(self.vew_gen_btn())
            .push(self.vew_settings_btn());

        Container::new(content_col).width(60).height(Length::Fill)
    }

    fn vew_home_btn(&self) -> Row<'a, Message> {
        let vline = zebra_ui::components::line::Line::new()
            .width(Length::Fixed(3.0))
            .height(Length::Fixed(45.0))
            .style(if self.route == NavRoute::Home {
                zebra_ui::components::line::LineStyleSheet::Inverse
            } else {
                zebra_ui::components::line::LineStyleSheet::Transparent
            });
        let lock_btn = Button::new(zebra_ui::image::lock_icon().height(40).width(40).style(
            if self.route == NavRoute::Home {
                zebra_ui::style::svg::Svg::Inverse
            } else {
                zebra_ui::style::svg::Svg::Primary
            },
        ))
        .padding(0)
        .style(zebra_ui::style::button::Button::Transparent)
        .on_press_maybe(self.on_home.clone());

        Row::new()
            .align_items(iced::Alignment::Center)
            .push(vline)
            .push(lock_btn)
    }

    fn vew_gen_btn(&self) -> Row<'a, Message> {
        let vline = zebra_ui::components::line::Line::new()
            .width(Length::Fixed(3.0))
            .height(Length::Fixed(45.0))
            .style(if self.route == NavRoute::Gen {
                zebra_ui::components::line::LineStyleSheet::Inverse
            } else {
                zebra_ui::components::line::LineStyleSheet::Transparent
            });
        let lock_btn = Button::new(zebra_ui::image::reload_icon().height(40).width(40).style(
            if self.route == NavRoute::Gen {
                zebra_ui::style::svg::Svg::Inverse
            } else {
                zebra_ui::style::svg::Svg::Primary
            },
        ))
        .padding(0)
        .style(zebra_ui::style::button::Button::Transparent)
        .on_press_maybe(self.on_home.clone());

        Row::new()
            .align_items(iced::Alignment::Center)
            .push(vline)
            .push(lock_btn)
    }

    fn vew_settings_btn(&self) -> Row<'a, Message> {
        let vline = zebra_ui::components::line::Line::new()
            .width(Length::Fixed(3.0))
            .height(Length::Fixed(45.0))
            .style(if self.route == NavRoute::Settings {
                zebra_ui::components::line::LineStyleSheet::Inverse
            } else {
                zebra_ui::components::line::LineStyleSheet::Transparent
            });
        let lock_btn = Button::new(zebra_ui::image::gear_icon().height(30).width(30).style(
            if self.route == NavRoute::Settings {
                zebra_ui::style::svg::Svg::Inverse
            } else {
                zebra_ui::style::svg::Svg::Primary
            },
        ))
        .padding(0)
        .style(zebra_ui::style::button::Button::Transparent)
        .on_press_maybe(self.on_home.clone());

        Row::new()
            .align_items(iced::Alignment::Center)
            .push(vline)
            .push(lock_btn)
    }
}
