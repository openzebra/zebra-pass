//! -- Copyright (c) 2023 Rina Khasanshin
//! -- Email: hicarus@yandex.ru
//! -- Licensed under the GNU General Public License Version 3.0 (GPL-3.0)
use std::sync::{Arc, Mutex};

use crate::pages::Page;

use super::pages;
use iced::advanced::Application;
use iced::{executor, Command, Element, Theme};
use zebra_lib::{core::Core, settings::appearance::Themes};

#[derive(Debug)]
pub enum Routers {
    Loading(pages::loader::Loader),
    Locale(pages::locale::Locale),
    Interview(pages::inverview::Interview),
    Options(pages::options::Options),
    GenPhrase(pages::gen_phrase::GenPhrase),
    Restore(pages::restore::Restore),
    PasswordSetup(pages::password_setup::PasswordSetup),
    Home(pages::home::Home),
    Generator(pages::gen::Generator),
    Settings(pages::settings::Settings),
    Lock(pages::lock::Lock),
    AddRecord(pages::add_record::AddRecordPage),
    ErrorPage(pages::error::ErrorPage),
}

pub struct Gui {
    core: Arc<Mutex<Core>>,
    route: Routers,
}

#[derive(Debug)]
pub enum GlobalMessage {
    LoadMessage(pages::loader::LoadMessage),
    LocaleMessage(pages::locale::LocaleMessage),
    InterviewMessage(pages::inverview::InterviewMessage),
    OptionsMessage(pages::options::OptionsMessage),
    GenPhraseMessage(pages::gen_phrase::GenPhraseMessage),
    RestoreMessage(pages::restore::RestoreMessage),
    PasswordSetupMessage(pages::password_setup::PasswordSetupMessage),
    HomeMessage(pages::home::HomeMessage),
    GeneratorMessage(pages::gen::GeneratorMessage),
    SettingsMessage(pages::settings::SettingsMessage),
    LockMessage(pages::lock::LockMessage),
    ErrorPageMessage(pages::error::ErrorPageMessage),
    AddRecordPageMessage(pages::add_record::AddRecordPageMessage),
    Route(Routers),
}

async fn load(_core: Arc<Mutex<Core>>) -> Result<(), ()> {
    std::thread::sleep(std::time::Duration::from_millis(100));
    // TODO: make it load when server sync added.
    Ok(())
}

impl Application for Gui {
    type Executor = executor::Default;
    type Message = GlobalMessage;
    type Flags = Core;
    type Theme = Theme;

    fn title(&self) -> String {
        "Zebrapass".into()
    }

    fn new(mut arg: Self::Flags) -> (Self, Command<Self::Message>) {
        arg.unlock("qazqaz666").unwrap();
        let core = Arc::new(Mutex::new(arg));
        let tmp = pages::home::Home::new(Arc::clone(&core)).unwrap(); // TODO: Remove unwrap
        let route = Routers::Home(tmp);
        // let loader = pages::loader::Loader::new(Arc::clone(&core)).unwrap(); // TODO: Remove unwrap
        // let route = Routers::Loading(loader);
        let core_ref = Arc::clone(&core);

        (
            Self { core, route },
            Command::perform(load(core_ref), |_| {
                GlobalMessage::LoadMessage(pages::loader::LoadMessage::Synced)
            }),
        )
    }

    fn update(&mut self, message: Self::Message) -> Command<Self::Message> {
        match message {
            GlobalMessage::LoadMessage(msg) => match &mut self.route {
                Routers::Loading(view) => view.update(msg),
                _ => Command::none(),
            },
            GlobalMessage::LocaleMessage(msg) => match &mut self.route {
                Routers::Locale(view) => view.update(msg),
                _ => Command::none(),
            },
            GlobalMessage::InterviewMessage(msg) => match &mut self.route {
                Routers::Interview(view) => view.update(msg),
                _ => Command::none(),
            },
            GlobalMessage::OptionsMessage(msg) => match &mut self.route {
                Routers::Options(view) => view.update(msg),
                _ => Command::none(),
            },
            GlobalMessage::GenPhraseMessage(msg) => match &mut self.route {
                Routers::GenPhrase(view) => view.update(msg),
                _ => Command::none(),
            },
            GlobalMessage::RestoreMessage(msg) => match &mut self.route {
                Routers::Restore(view) => view.update(msg),
                _ => Command::none(),
            },
            GlobalMessage::PasswordSetupMessage(msg) => match &mut self.route {
                Routers::PasswordSetup(view) => view.update(msg),
                _ => Command::none(),
            },
            GlobalMessage::AddRecordPageMessage(msg) => match &mut self.route {
                Routers::AddRecord(view) => view.update(msg),
                _ => Command::none(),
            },
            GlobalMessage::HomeMessage(msg) => match &mut self.route {
                Routers::Home(view) => view.update(msg),
                _ => Command::none(),
            },
            GlobalMessage::LockMessage(msg) => match &mut self.route {
                Routers::Lock(view) => view.update(msg),
                _ => Command::none(),
            },
            GlobalMessage::GeneratorMessage(msg) => match &mut self.route {
                Routers::Generator(view) => view.update(msg),
                _ => Command::none(),
            },
            GlobalMessage::SettingsMessage(msg) => match &mut self.route {
                Routers::Settings(view) => view.update(msg),
                _ => Command::none(),
            },
            GlobalMessage::ErrorPageMessage(msg) => match &mut self.route {
                Routers::ErrorPage(view) => view.update(msg),
                _ => Command::none(),
            },
            GlobalMessage::Route(route) => {
                self.route = route;
                Command::none()
            }
        }
    }

    fn subscription(&self) -> iced::Subscription<Self::Message> {
        iced::Subscription::batch([match &self.route {
            Routers::Loading(v) => v.subscription().map(GlobalMessage::LoadMessage),
            Routers::ErrorPage(v) => v.subscription().map(GlobalMessage::ErrorPageMessage),
            Routers::Interview(v) => v.subscription().map(GlobalMessage::InterviewMessage),
            Routers::Locale(v) => v.subscription().map(GlobalMessage::LocaleMessage),
            Routers::Options(v) => v.subscription().map(GlobalMessage::OptionsMessage),
            Routers::GenPhrase(v) => v.subscription().map(GlobalMessage::GenPhraseMessage),
            Routers::Restore(v) => v.subscription().map(GlobalMessage::RestoreMessage),
            Routers::PasswordSetup(v) => v.subscription().map(GlobalMessage::PasswordSetupMessage),
            Routers::Home(v) => v.subscription().map(GlobalMessage::HomeMessage),
            Routers::Settings(v) => v.subscription().map(GlobalMessage::SettingsMessage),
            Routers::Generator(v) => v.subscription().map(GlobalMessage::GeneratorMessage),
            Routers::Lock(v) => v.subscription().map(GlobalMessage::LockMessage),
            Routers::AddRecord(v) => v.subscription().map(GlobalMessage::AddRecordPageMessage),
        }])
    }

    fn view(&self) -> Element<'_, Self::Message, Self::Theme> {
        match &self.route {
            Routers::Loading(l) => l.view().map(GlobalMessage::LoadMessage),
            Routers::Locale(l) => l.view().map(GlobalMessage::LocaleMessage),
            Routers::Interview(l) => l.view().map(GlobalMessage::InterviewMessage),
            Routers::Options(l) => l.view().map(GlobalMessage::OptionsMessage),
            Routers::GenPhrase(l) => l.view().map(GlobalMessage::GenPhraseMessage),
            Routers::Restore(l) => l.view().map(GlobalMessage::RestoreMessage),
            Routers::PasswordSetup(l) => l.view().map(GlobalMessage::PasswordSetupMessage),
            Routers::Home(l) => l.view().map(GlobalMessage::HomeMessage),
            Routers::Lock(l) => l.view().map(GlobalMessage::LockMessage),
            Routers::Generator(l) => l.view().map(GlobalMessage::GeneratorMessage),
            Routers::Settings(l) => l.view().map(GlobalMessage::SettingsMessage),
            Routers::AddRecord(l) => l.view().map(GlobalMessage::AddRecordPageMessage),
            Routers::ErrorPage(l) => l.view().map(GlobalMessage::ErrorPageMessage),
        }
    }

    fn scale_factor(&self) -> f64 {
        1.0
    }

    fn theme(&self) -> Self::Theme {
        // TODO: Remove unwrap.
        match self.core.lock().unwrap().state.settings.appearance.theme {
            Themes::Dark => Theme::Custom(Arc::new(zebra_ui::theme::dark_custom_theme())),
            Themes::Light => Theme::Custom(Arc::new(zebra_ui::theme::light_custom_theme())),
            Themes::Auto => match dark_light::detect() {
                dark_light::Mode::Dark => {
                    Theme::Custom(Arc::new(zebra_ui::theme::dark_custom_theme()))
                }
                dark_light::Mode::Light => {
                    Theme::Custom(Arc::new(zebra_ui::theme::light_custom_theme()))
                }
                dark_light::Mode::Default => {
                    Theme::Custom(Arc::new(zebra_ui::theme::dark_custom_theme()))
                }
            },
        }
    }
}
