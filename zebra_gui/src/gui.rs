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
            Routers::Loading(v) => v.subscription().map(|msg| GlobalMessage::LoadMessage(msg)),
            Routers::ErrorPage(v) => v
                .subscription()
                .map(|msg| GlobalMessage::ErrorPageMessage(msg)),
            Routers::Interview(v) => v
                .subscription()
                .map(|msg| GlobalMessage::InterviewMessage(msg)),
            Routers::Locale(v) => v
                .subscription()
                .map(|msg| GlobalMessage::LocaleMessage(msg)),
            Routers::Options(v) => v
                .subscription()
                .map(|msg| GlobalMessage::OptionsMessage(msg)),
            Routers::GenPhrase(v) => v
                .subscription()
                .map(|msg| GlobalMessage::GenPhraseMessage(msg)),
            Routers::Restore(v) => v
                .subscription()
                .map(|msg| GlobalMessage::RestoreMessage(msg)),
            Routers::PasswordSetup(v) => v
                .subscription()
                .map(|msg| GlobalMessage::PasswordSetupMessage(msg)),
            Routers::Home(v) => v.subscription().map(|msg| GlobalMessage::HomeMessage(msg)),
            Routers::Settings(v) => v
                .subscription()
                .map(|msg| GlobalMessage::SettingsMessage(msg)),
            Routers::Generator(v) => v
                .subscription()
                .map(|msg| GlobalMessage::GeneratorMessage(msg)),
            Routers::Lock(v) => v.subscription().map(|msg| GlobalMessage::LockMessage(msg)),
            Routers::AddRecord(v) => v
                .subscription()
                .map(|msg| GlobalMessage::AddRecordPageMessage(msg)),
        }])
    }

    fn view(&self) -> Element<'_, Self::Message, Self::Theme> {
        match &self.route {
            Routers::Loading(l) => l.view().map(|msg| GlobalMessage::LoadMessage(msg)),
            Routers::Locale(l) => l.view().map(|msg| GlobalMessage::LocaleMessage(msg)),
            Routers::Interview(l) => l.view().map(|msg| GlobalMessage::InterviewMessage(msg)),
            Routers::Options(l) => l.view().map(|msg| GlobalMessage::OptionsMessage(msg)),
            Routers::GenPhrase(l) => l.view().map(|msg| GlobalMessage::GenPhraseMessage(msg)),
            Routers::Restore(l) => l.view().map(|msg| GlobalMessage::RestoreMessage(msg)),
            Routers::PasswordSetup(l) => {
                l.view().map(|msg| GlobalMessage::PasswordSetupMessage(msg))
            }
            Routers::Home(l) => l.view().map(|msg| GlobalMessage::HomeMessage(msg)),
            Routers::Lock(l) => l.view().map(|msg| GlobalMessage::LockMessage(msg)),
            Routers::Generator(l) => l.view().map(|msg| GlobalMessage::GeneratorMessage(msg)),
            Routers::Settings(l) => l.view().map(|msg| GlobalMessage::SettingsMessage(msg)),
            Routers::AddRecord(l) => l.view().map(|msg| GlobalMessage::AddRecordPageMessage(msg)),
            Routers::ErrorPage(l) => l.view().map(|msg| GlobalMessage::ErrorPageMessage(msg)),
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
