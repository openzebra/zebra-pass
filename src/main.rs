// -- Copyright (c) 2023 Rina Khasanshin
// -- Email: hicarus@yandex.ru
// -- Licensed under the GNU General Public License Version 3.0 (GPL-3.0)

use std::{cell::RefCell, rc::Rc};

use chrono::{DateTime, Local};
use rand;
use serde::{
    de::{MapAccess, Visitor},
    ser::SerializeStruct,
};
use slint::{Model, SharedString, VecModel};
use zebra_pass::{
    bip39::mnemonic::{Language, Mnemonic},
    core::{
        bip39::{self, from_bip39_model},
        core::Core,
        passgen::PassGen,
    },
    errors::ZebraErrors,
};

slint::include_modules!();

impl<'de> serde::Deserialize<'de> for ElementItem {
    fn deserialize<D>(deserializer: D) -> Result<Self, D::Error>
    where
        D: serde::Deserializer<'de>,
    {
        struct ElementItemVisitor;

        impl<'de> Visitor<'de> for ElementItemVisitor {
            type Value = ElementItem;

            fn expecting(&self, formatter: &mut std::fmt::Formatter) -> std::fmt::Result {
                formatter.write_str("struct ElementItem")
            }

            fn visit_map<V>(self, mut map: V) -> Result<ElementItem, V::Error>
            where
                V: MapAccess<'de>,
            {
                let mut title: Option<String> = None;
                let mut value: Option<String> = None;
                let mut hide: Option<bool> = None;
                let mut copy: Option<bool> = None;

                while let Some(key) = map.next_key()? {
                    match key {
                        "title" => {
                            title = Some(map.next_value()?);
                        }
                        "value" => {
                            value = Some(map.next_value()?);
                        }
                        "hide" => {
                            hide = Some(map.next_value()?);
                        }
                        "copy" => {
                            copy = Some(map.next_value()?);
                        }
                        _ => {
                            // Ignore unknown fields
                            map.next_value()?;
                        }
                    }
                }

                let title = title.ok_or_else(|| serde::de::Error::missing_field("title"))?;
                let value = value.ok_or_else(|| serde::de::Error::missing_field("value"))?;
                let hide = hide.unwrap_or(false);
                let copy = copy.unwrap_or(false);

                Ok(ElementItem {
                    hide,
                    copy,
                    title: title.into(),
                    value: value.into(),
                })
            }
        }

        deserializer.deserialize_struct(
            "ElementItem",
            &["title", "value", "hide", "copy"],
            ElementItemVisitor,
        )
    }
}

impl<'de> serde::Deserialize<'de> for Element {
    fn deserialize<D>(deserializer: D) -> Result<Element, D::Error>
    where
        D: serde::Deserializer<'de>,
    {
        struct ElementVisitor;

        impl<'de> Visitor<'de> for ElementVisitor {
            type Value = Element;

            fn expecting(&self, formatter: &mut std::fmt::Formatter) -> std::fmt::Result {
                formatter.write_str("struct Element")
            }

            fn visit_map<V>(self, mut map: V) -> Result<Element, V::Error>
            where
                V: MapAccess<'de>,
            {
                let mut icon: Option<String> = None;
                let mut name: Option<String> = None;
                let mut website: Option<String> = None;
                let mut r#type: Option<i32> = None;
                let mut created: Option<String> = None;
                let mut updated: Option<String> = None;
                let mut favourite: Option<bool> = None;
                let mut fields: Option<Vec<ElementItem>> = None;
                let mut extra_fields: Option<Vec<ElementItem>> = None;

                while let Some(key) = map.next_key()? {
                    match key {
                        "icon" => {
                            icon = Some(map.next_value()?);
                        }
                        "name" => {
                            name = Some(map.next_value()?);
                        }
                        "website" => {
                            website = Some(map.next_value()?);
                        }
                        "type" => {
                            r#type = Some(map.next_value()?);
                        }
                        "created" => {
                            created = Some(map.next_value()?);
                        }
                        "updated" => {
                            updated = Some(map.next_value()?);
                        }
                        "favourite" => {
                            favourite = Some(map.next_value()?);
                        }
                        "fields" => {
                            fields = Some(map.next_value()?);
                        }
                        "extra_fields" => {
                            extra_fields = Some(map.next_value()?);
                        }
                        _ => {
                            // Ignore unknown fields
                            map.next_value()?;
                        }
                    }
                }

                let icon = icon.ok_or_else(|| serde::de::Error::missing_field("icon"))?;
                let name = name.ok_or_else(|| serde::de::Error::missing_field("name"))?;
                let website = website.ok_or_else(|| serde::de::Error::missing_field("website"))?;
                let r#type = r#type.ok_or_else(|| serde::de::Error::missing_field("type"))?;
                let created = created.ok_or_else(|| serde::de::Error::missing_field("created"))?;
                let updated = updated.ok_or_else(|| serde::de::Error::missing_field("updated"))?;
                let favourite =
                    favourite.ok_or_else(|| serde::de::Error::missing_field("favourite"))?;
                let fields = fields.ok_or_else(|| serde::de::Error::missing_field("fields"))?;
                let extra_fields =
                    extra_fields.ok_or_else(|| serde::de::Error::missing_field("extra_fields"))?;

                Ok(Element {
                    // TODO: make it works.
                    favourite,
                    icon: icon.into(),
                    name: name.into(),
                    website: website.into(),
                    r#type: r#type.into(),
                    created: created.into(),
                    updated: updated.into(),
                    fields: VecModel::from_slice(&fields),
                    extra_fields: VecModel::from_slice(&extra_fields),
                })
            }
        }

        deserializer.deserialize_struct(
            "Element",
            &[
                "icon",
                "name",
                "website",
                "type",
                "created",
                "updated",
                "favourite",
                "fields",
                "extra_fields",
            ],
            ElementVisitor,
        )
    }
}

impl serde::Serialize for ElementItem {
    fn serialize<S>(&self, serializer: S) -> Result<S::Ok, S::Error>
    where
        S: serde::Serializer,
    {
        let mut state = serializer.serialize_struct("ElementItem", 4)?;

        state.serialize_field("title", &self.title.to_string())?;
        state.serialize_field("value", &self.value.to_string())?;
        state.serialize_field("hide", &self.hide)?;
        state.serialize_field("copy", &self.copy)?;

        state.end()
    }
}

impl serde::Serialize for Element {
    fn serialize<S>(&self, serializer: S) -> Result<S::Ok, S::Error>
    where
        S: serde::Serializer,
    {
        let mut state = serializer.serialize_struct("Element", 9)?;
        let fields = self.fields.iter().collect::<Vec<ElementItem>>();
        let extra_fields = self.extra_fields.iter().collect::<Vec<ElementItem>>();

        state.serialize_field("name", &self.name.to_string())?;
        state.serialize_field("icon", &self.icon.to_string())?;
        state.serialize_field("website", &self.website.to_string())?;
        state.serialize_field("type", &self.r#type)?;
        state.serialize_field("created", &self.created.to_string())?;
        state.serialize_field("updated", &self.updated.to_string())?;
        state.serialize_field("favourite", &self.favourite)?;

        state.serialize_field("fields", &fields)?;
        state.serialize_field("extra_fields", &extra_fields)?;

        state.end()
    }
}

fn handler(core: Rc<RefCell<Core<Element>>>) -> Result<(), slint::PlatformError> {
    slint::init_translations!(concat!(env!("CARGO_MANIFEST_DIR"), "/locale/"));

    let core_ref_state = core.clone();
    let state = &core_ref_state.borrow().state.clone();
    let app = AppWindow::new()?;
    let main_window = Rc::new(app.as_weak().unwrap());

    if !state.borrow().payload.inited {
        main_window.set_route(Routers::LangChoose);
    } else {
        main_window.set_route(Routers::Lock);
    }

    main_window
        .global::<KeyChainLogic>()
        .on_request_random_words(|length_str| {
            let mut rng = rand::thread_rng();
            let count: usize = length_str.to_string().parse().unwrap_or(12);
            // TODO: make a error hanlder.
            let m = Mnemonic::gen(&mut rng, count, Language::English).unwrap();
            bip39::gen_bip39_words(&m, 3)
        });

    let keys_logic_ref = main_window.clone();
    let core_ref = core.clone();
    main_window
        .global::<KeyChainLogic>()
        .on_request_create_account(move || {
            let mut core = core_ref.borrow_mut();
            let sync = keys_logic_ref.global::<KeyChainLogic>().get_sync();
            let email = keys_logic_ref.global::<KeyChainLogic>().get_email();
            let password = keys_logic_ref.global::<KeyChainLogic>().get_password();
            let words_salt = keys_logic_ref.global::<KeyChainLogic>().get_words_salt();
            let words_model = keys_logic_ref.global::<KeyChainLogic>().get_random_words();
            let m = match from_bip39_model(words_model) {
                Ok(r) => r,
                Err(_) => {
                    return LogicResult {
                        error: "bip39 words are invalid".into(),
                        response: SharedString::default(),
                        success: true,
                    }
                }
            };

            match core.init_data(sync, &email, &password, &words_salt, &m) {
                Ok(_) => LogicResult {
                    error: SharedString::default(),
                    response: SharedString::default(),
                    success: true,
                },
                Err(_) => LogicResult {
                    // TODO: make more informative errors
                    error: "Cannot init data".into(),
                    response: SharedString::default(),
                    success: false,
                },
            }
        });

    let lock_page_core_ref = core.clone();
    let unlock_logic_ref = main_window.clone();
    main_window
        .global::<KeyChainLogic>()
        .on_request_unlock(move |password| {
            let mut core = lock_page_core_ref.borrow_mut();

            match core.unlock(&password.to_string()) {
                Ok(_) => {
                    let elements = VecModel::from_slice(&core.data);
                    unlock_logic_ref.global::<Logic>().set_elements(elements);
                    LogicResult {
                        error: SharedString::default(),
                        response: SharedString::default(),
                        success: true,
                    }
                }
                Err(_) => LogicResult {
                    // TODO: add more informative errors
                    error: "incorrect password".into(),
                    response: SharedString::default(),
                    success: false,
                },
            }
        });

    main_window
        .global::<GeneratorLogic>()
        .on_request_password_gen(
            move |lowercase: bool, upercase: bool, nums: bool, symbols: bool, length: i32| {
                let mut rng = rand::thread_rng();
                let pass_gen = PassGen::from(lowercase, upercase, nums, symbols);

                match pass_gen.gen(length as usize, &mut rng) {
                    Ok(password) => LogicResult {
                        error: SharedString::default(),
                        success: true,
                        response: String::from_utf8_lossy(&password).to_string().into(),
                    },
                    Err(_) => LogicResult {
                        // TODO: add more informative errors
                        error: "Invalid RNG".into(),
                        response: SharedString::default(),
                        success: false,
                    },
                }
            },
        );

    let core_elements_add_ref = core.clone();
    let logic_ref_add_new_element = Rc::clone(&main_window);
    main_window
        .global::<Logic>()
        .on_add_new_element(move |mut element| {
            let local: DateTime<Local> = Local::now();
            let formatted_date = local.format("%a %b %e %Y %H:%M:%S GMT%:z (%Z)").to_string();

            element.created = formatted_date.into();
            element.updated = element.created.clone();
            let mut core = core_elements_add_ref.borrow_mut();

            core.add_element(element).unwrap();
            logic_ref_add_new_element
                .global::<Logic>()
                .set_elements(VecModel::from_slice(&core.data));

            LogicResult {
                error: "".into(),
                response: SharedString::default(),
                success: false,
            }
        });

    app.run()
}

fn error_handler(error: ZebraErrors) -> Result<(), slint::PlatformError> {
    dbg!(error);
    // TODO: Show error window!
    Ok(())
}

fn main() -> Result<(), slint::PlatformError> {
    let core: Core<Element> = match Core::new() {
        Ok(c) => c,
        Err(e) => {
            return error_handler(e);
        }
    };

    match core.sync() {
        Ok(_) => {}
        Err(e) => {
            return error_handler(e);
        }
    }

    handler(Rc::new(RefCell::new(core)))
}

#[cfg(test)]
mod main_tests {
    use super::*;
    use slint::{ModelRc, SharedString};

    #[test]
    fn test_element_item_deserialize() {
        let test_item_element = ElementItem {
            title: "title".into(),
            value: "some value".into(),
            hide: false,
            copy: true,
        };
        let json_payload = serde_json::to_string(&test_item_element).unwrap();
        let data: ElementItem = serde_json::from_str(&json_payload).unwrap();

        assert_eq!(test_item_element.title, data.title);
        assert_eq!(test_item_element.value, data.value);
        assert_eq!(test_item_element.hide, data.hide);
        assert_eq!(test_item_element.copy, data.copy);
    }

    #[test]
    fn test_element_deserialize() {
        let test_element = Element {
            name: SharedString::from("name"),
            website: SharedString::from("domain.com"),
            icon: SharedString::default(),
            r#type: 0,
            created: SharedString::from("time"),
            updated: SharedString::from("time"),
            favourite: true,
            fields: ModelRc::default(),
            extra_fields: ModelRc::default(),
        };
        let json_payload = serde_json::to_string(&test_element).unwrap();
        let data: Element = serde_json::from_str(&json_payload).unwrap();

        assert_eq!(test_element.name, data.name);
        assert_eq!(test_element.website, data.website);
        assert_eq!(test_element.icon, data.icon);
        assert_eq!(test_element.r#type, data.r#type);
        assert_eq!(test_element.created, data.created);
        assert_eq!(test_element.updated, data.updated);
        assert_eq!(test_element.favourite, data.favourite);
        assert_eq!(
            test_element.fields.iter().collect::<Vec<ElementItem>>(),
            data.fields.iter().collect::<Vec<ElementItem>>(),
        );
        assert_eq!(
            test_element
                .extra_fields
                .iter()
                .collect::<Vec<ElementItem>>(),
            data.extra_fields.iter().collect::<Vec<ElementItem>>(),
        );
    }

    #[test]
    fn test_main() {
        let core: Core<Element> = match Core::from("test_main", "test_main_app", "test_main_corp") {
            Ok(c) => c,
            Err(e) => {
                panic!("{:?}", e);
            }
        };

        match core.sync() {
            Ok(_) => {}
            Err(e) => {
                panic!("{:?}", e);
            }
        }
    }
}
