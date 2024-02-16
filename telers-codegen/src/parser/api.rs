use serde::{Deserialize, Serialize};
use std::collections::HashMap;

/// [`TelegramTypeName`] is a string that is used to identify a type, for example `Chat` or `User`.
pub type TelegramTypeName = String;
pub type FieldName = String;
pub type RawType = String;

#[derive(Debug, Deserialize, Serialize)]
pub struct Field {
    pub name: FieldName,
    pub required: bool,
    pub description: String,
    pub types: Vec<RawType>,
}

impl Field {
    /// # Panis
    /// * If the field has multiple types, but it's not a known special case.
    pub fn identify_field_type(&self) -> TypeKindInField {
        let types = self.types.as_slice();

        if multi_type_is_input_file(types) {
            return TypeKindInField::Telegram("InputFile".to_owned());
        }

        if multi_type_is_chat_id(types) {
            return TypeKindInField::Telegram("ChatId".to_owned());
        }

        if multi_type_is_reply_markup(types, &self.name) {
            return TypeKindInField::Telegram("ReplyMarkup".to_owned());
        }

        if types.len() > 1 {
            unimplemented!("Unknown case for multi types");
        }

        let r#type = types.first().unwrap();

        if is_array_of(r#type) {
            let inner_type = Field {
                name: self.name.clone(),
                required: self.required,
                description: self.description.clone(),
                types: vec![r#type.replacen("Array of ", "", 1)],
            }
            .identify_field_type();

            return TypeKindInField::Array(Box::new(inner_type));
        }

        if is_string(r#type) {
            return TypeKindInField::String;
        }

        if let Some(integer_kind) = get_if_integer(r#type, &self.description) {
            return TypeKindInField::Integer(integer_kind);
        }

        if let Some(boolean_kind) = get_if_boolean(r#type) {
            return TypeKindInField::Boolean(boolean_kind);
        }

        TypeKindInField::Telegram(r#type.to_owned())
    }
}

#[derive(Debug, Deserialize, Serialize)]
pub struct Type {
    pub name: TelegramTypeName,
    pub href: String,
    pub description: Vec<String>,
    #[serde(default)]
    pub fields: Vec<Field>,
    #[serde(default)]
    pub subtypes: Vec<TelegramTypeName>,
    #[serde(default)]
    pub subtype_of: Vec<TelegramTypeName>,
}

#[derive(Debug, Default, Deserialize, Serialize)]
pub struct Scheme {
    pub version: String,
    pub release_date: String,
    pub changelog: String,
    pub types: HashMap<TelegramTypeName, Type>,
}

impl Scheme {
    pub fn parse_from_jsom(content: &str) -> Result<Self, serde_json::Error> {
        serde_json::from_str(content)
    }

    pub fn normalize(self) -> NormalizedScheme {
        let mut normalized_types = HashMap::new();
        for (k, ty) in self.types {
            let mut normalized_fields = vec![];
            for field in ty.fields {
                normalized_fields.push(NormalizedField {
                    r#type: field.identify_field_type(),
                    name: field.name,
                    required: field.required,
                    description: field.description,
                });
            }

            let normalized_ty = NormalizedType {
                name: ty.name,
                href: ty.href,
                description: ty.description,
                fields: normalized_fields,
                subtypes: ty.subtypes,
                subtype_of: ty.subtype_of,
            };
            normalized_types.insert(k, normalized_ty);
        }

        NormalizedScheme {
            version: self.version,
            release_date: self.release_date,
            changelog: self.changelog,
            types: normalized_types,
        }
    }

    pub fn is_telegram_type(&self, raw_type: &RawType) -> bool {
        self.types.contains_key(raw_type)
    }
}

#[derive(Debug)]
pub struct NormalizedField {
    pub name: FieldName,
    pub required: bool,
    pub description: String,
    pub r#type: TypeKindInField,
}

#[derive(Debug)]
pub struct NormalizedType {
    pub name: TelegramTypeName,
    pub href: String,
    pub description: Vec<String>,
    pub fields: Vec<NormalizedField>,
    pub subtypes: Vec<TelegramTypeName>,
    pub subtype_of: Vec<TelegramTypeName>,
}

#[derive(Debug, Default)]
pub struct NormalizedScheme {
    pub version: String,
    pub release_date: String,
    pub changelog: String,
    pub types: HashMap<TelegramTypeName, NormalizedType>,
}

#[derive(Debug, PartialEq, Eq)]
pub enum IntegerKind {
    Int64,
    Int32,
    Int16,
    Float32,
}

/// # Variants
/// - `Any` - Any boolean value
/// - `True` - Only possible value is `true`
#[derive(Debug, PartialEq, Eq)]
pub enum BooleanKind {
    Any,
    True,
}

#[derive(Debug, PartialEq, Eq)]
pub enum TypeKindInField {
    String,
    Integer(IntegerKind),
    Boolean(BooleanKind),
    Telegram(TelegramTypeName),
    Array(Box<TypeKindInField>),
}

pub fn is_string(raw_type: &RawType) -> bool {
    raw_type == "String"
}

/// # Notes
/// Currently use only [`IntegerKind::Int64`] and [`IntegerKind::Float32`].
/// Need to add support for other integer types by its description.
pub fn get_if_integer(raw_type: &RawType, _description: &str) -> Option<IntegerKind> {
    match raw_type.as_str() {
        "Integer" => Some(IntegerKind::Int64),
        "Float" => Some(IntegerKind::Float32),
        _ => None,
    }
}

/// # Notes
/// Currently use only [`BooleanKind::Any`] and [`BooleanKind::True`].
/// Type like `False` is not used in the Telegram API.
pub fn get_if_boolean(raw_type: &RawType) -> Option<BooleanKind> {
    match raw_type.as_str() {
        "Boolean" => Some(BooleanKind::Any),
        "True" => Some(BooleanKind::True),
        _ => None,
    }
}

/// # Notes
/// All arrays in the Telegram API are starts with `Array of` prefix.
pub fn is_array_of(raw_type: &RawType) -> bool {
    raw_type.starts_with("Array of")
}

/// If the type is an array with `Integer` and `String` then it's just `InputFile`,
/// because all possible `String` files reresentations are wrapped in `InputFile`.
/// # Notes
/// This function is a special case for `InputFile` type.
pub fn multi_type_is_input_file(types: &[RawType]) -> bool {
    if types.len() == 1 && types.contains(&"InputFile".to_owned()) {
        return true;
    }

    if types.len() != 2 {
        return false;
    }

    types.contains(&"InputFile".to_owned()) && types.contains(&"String".to_owned())
}

/// If the type is an array with `Integer` and `String` then it's just `ChatId`.
/// # Notes
/// `ChatId` is a helper type that can be represented as `Integer` or `String`.
///
/// This function is a special case for `ChatId` type.
pub fn multi_type_is_chat_id(types: &[RawType]) -> bool {
    if types.len() != 2 {
        return false;
    }

    types.contains(&"Integer".to_owned()) && types.contains(&"String".to_owned())
}

/// If the type is an array with `InlineKeyboardMarkup` and `ReplyKeyboardMarkup`, etc., then it's just `ReplyMarkup`.
/// If it's array with one type, then it's not `ReplyMarkup`.
/// # Notes
/// This function is a special case for `ReplyMarkup` type.
/// # Warnings
/// Here not checks that types are markup types, because if `name` is `reply_markup` then it's a markup type: single or multi.
pub fn multi_type_is_reply_markup(types: &[RawType], name: &str) -> bool {
    if types.len() == 1 {
        return false;
    }

    name == "reply_markup"
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_parse_json_to_scheme() {
        let content = r#"
            {
                "version": "1.0",
                "release_date": "2021-01-01",
                "changelog": "Initial release",
                "types": {
                    "Type1": {
                        "name": "Type1",
                        "href": "https://example.com",
                        "description": ["Type1 description"],
                        "fields": [
                            {
                                "name": "field1",
                                "required": true,
                                "description": "Field1 description",
                                "types": ["String"]
                            }
                        ]
                    }
                }
            }
        "#;

        let scheme = Scheme::parse_from_jsom(content).unwrap();

        assert_eq!(scheme.version, "1.0");
        assert_eq!(scheme.release_date, "2021-01-01");
        assert_eq!(scheme.changelog, "Initial release");
        assert_eq!(scheme.types.len(), 1);
    }

    #[test]
    fn test_is_telegram_type() {
        let content = r#"
            {
                "version": "1.0",
                "release_date": "2021-01-01",
                "changelog": "Initial release",
                "types": {
                    "Type1": {
                        "name": "Type1",
                        "href": "https://example.com",
                        "description": ["Type1 description"],
                        "fields": [
                            {
                                "name": "field1",
                                "required": true,
                                "description": "Field1 description",
                                "types": ["String"]
                            }
                        ]
                    }
                }
            }
        "#;

        let scheme = Scheme::parse_from_jsom(content).unwrap();

        assert!(scheme.is_telegram_type(&"Type1".to_owned()));
        assert!(!scheme.is_telegram_type(&"Type2".to_owned()));
    }

    #[test]
    fn test_is_string() {
        assert!(is_string(&"String".to_owned()));
        assert!(!is_string(&"Integer".to_owned()));
    }

    #[test]
    fn test_get_if_integer() {
        assert_eq!(
            get_if_integer(&"Integer".to_owned(), ""),
            Some(IntegerKind::Int64)
        );
        assert_eq!(
            get_if_integer(&"Float".to_owned(), ""),
            Some(IntegerKind::Float32)
        );
        assert_eq!(get_if_integer(&"String".to_owned(), ""), None);
    }

    #[test]
    fn test_get_if_boolean() {
        assert_eq!(
            get_if_boolean(&"Boolean".to_owned()),
            Some(BooleanKind::Any)
        );
        assert_eq!(get_if_boolean(&"True".to_owned()), Some(BooleanKind::True));
        assert_eq!(get_if_boolean(&"String".to_owned()), None);
    }

    #[test]
    fn test_is_array_of() {
        assert!(is_array_of(&"Array of String".to_owned()));
        assert!(is_array_of(&"Array of Array of String".to_owned()));
        assert!(!is_array_of(&"String".to_owned()));
    }

    #[test]
    fn test_multi_type_is_input_file() {
        assert!(multi_type_is_input_file(&["InputFile".to_owned()]));
        assert!(multi_type_is_input_file(&[
            "InputFile".to_owned(),
            "String".to_owned(),
        ]));
        assert!(multi_type_is_input_file(&[
            "String".to_owned(),
            "InputFile".to_owned(),
        ]));
        assert!(!multi_type_is_input_file(&["String".to_owned()]));
        assert!(!multi_type_is_input_file(&[
            "String".to_owned(),
            "Integer".to_owned(),
        ]));
    }

    #[test]
    fn test_multi_type_is_chat_id() {
        assert!(!multi_type_is_chat_id(&["Integer".to_owned()]));
        assert!(!multi_type_is_chat_id(&["String".to_owned()]));
        assert!(multi_type_is_chat_id(&[
            "String".to_owned(),
            "Integer".to_owned(),
        ]));
        assert!(multi_type_is_chat_id(&[
            "Integer".to_owned(),
            "String".to_owned(),
        ]));
        assert!(!multi_type_is_chat_id(&["InputFile".to_owned()]));
        assert!(!multi_type_is_chat_id(&[
            "InputFile".to_owned(),
            "String".to_owned(),
        ]));
    }

    #[test]
    fn test_multi_type_is_reply_markup() {
        assert!(!multi_type_is_reply_markup(
            &["String".to_owned()],
            "reply_markup"
        ));
        assert!(!multi_type_is_reply_markup(
            &["InlineKeyboardMarkup".to_owned()],
            "reply_markup"
        ));
        assert!(multi_type_is_reply_markup(
            &["Markup1".to_owned(), "Markup2".to_owned()],
            "reply_markup"
        ));
        assert!(multi_type_is_reply_markup(
            &[
                "InlineKeyboardMarkup".to_owned(),
                "ReplyKeyboardMarkup".to_owned(),
            ],
            "reply_markup"
        ));
        assert!(multi_type_is_reply_markup(
            &[
                "ReplyKeyboardMarkup".to_owned(),
                "InlineKeyboardMarkup".to_owned(),
            ],
            "reply_markup"
        ));
    }

    #[test]
    fn test_identify_field_type() {
        let content = r#"
            {
                "version": "1.0",
                "release_date": "2021-01-01",
                "changelog": "Initial release",
                "types": {
                    "Type1": {
                        "name": "Type1",
                        "href": "https://example.com",
                        "description": ["Type1 description"],
                        "fields": [
                            {
                                "name": "field1",
                                "required": true,
                                "description": "Field1 description",
                                "types": ["String"]
                            },
                            {
                                "name": "field2",
                                "required": true,
                                "description": "Field2 description",
                                "types": ["Integer"]
                            },
                            {
                                "name": "field3",
                                "required": true,
                                "description": "Field3 description",
                                "types": ["Boolean"]
                            },
                            {
                                "name": "field4",
                                "required": true,
                                "description": "Field4 description",
                                "types": ["Float"]
                            },
                            {
                                "name": "field5",
                                "required": true,
                                "description": "Field5 description",
                                "types": ["Array of String"]
                            },
                            {
                                "name": "field6",
                                "required": true,
                                "description": "Field6 description",
                                "types": ["Array of Array of String"]
                            },
                            {
                                "name": "field7",
                                "required": true,
                                "description": "Field7 description",
                                "types": ["Array of Integer"]
                            },
                            {
                                "name": "field8",
                                "required": true,
                                "description": "Field8 description",
                                "types": ["Array of Float"]
                            },
                            {
                                "name": "field9",
                                "required": true,
                                "description": "Field9 description",
                                "types": ["True"]
                            },
                            {
                                "name": "field10",
                                "required": true,
                                "description": "Field10 description",
                                "types": ["Type1"]
                            }
                        ]
                    },
                    "Type2": {
                        "name": "Type2",
                        "href": "https://example.com",
                        "description": ["Type2 description"],
                        "fields": [
                            {
                                "name": "field1",
                                "required": true,
                                "description": "Field1 description",
                                "types": ["InputFile", "String"]
                            },
                            {
                                "name": "field2",
                                "required": true,
                                "description": "Field2 description",
                                "types": ["InputFile"]
                            },
                            {
                                "name": "reply_markup",
                                "required": true,
                                "description": "Field description",
                                "types": ["InlineKeyboardMarkup", "ReplyKeyboardMarkup"]
                            },
                            {
                                "name": "reply_markup",
                                "required": true,
                                "description": "Field description",
                                "types": ["ReplyKeyboardMarkup"]
                            },
                            {
                                "name": "chat_id",
                                "required": true,
                                "description": "Field description",
                                "types": ["Integer", "String"]
                            },
                            {
                                "name": "chat_id",
                                "required": true,
                                "description": "Field description",
                                "types": ["String", "Integer"]
                            },
                            {
                                "name": "chat_id",
                                "required": true,
                                "description": "Field description",
                                "types": ["Integer"]
                            }
                        ]
                    }
                }
            }
        "#;

        let scheme = Scheme::parse_from_jsom(content).unwrap();

        let fields = scheme.types.get("Type1").unwrap().fields.as_slice();

        assert_eq!(
            fields.get(0).unwrap().identify_field_type(),
            TypeKindInField::String
        );
        assert_eq!(
            fields.get(1).unwrap().identify_field_type(),
            TypeKindInField::Integer(IntegerKind::Int64)
        );
        assert_eq!(
            fields.get(2).unwrap().identify_field_type(),
            TypeKindInField::Boolean(BooleanKind::Any)
        );
        assert_eq!(
            fields.get(3).unwrap().identify_field_type(),
            TypeKindInField::Integer(IntegerKind::Float32)
        );
        assert_eq!(
            fields.get(4).unwrap().identify_field_type(),
            TypeKindInField::Array(Box::new(TypeKindInField::String))
        );
        assert_eq!(
            fields.get(5).unwrap().identify_field_type(),
            TypeKindInField::Array(Box::new(TypeKindInField::Array(Box::new(
                TypeKindInField::String
            ))))
        );
        assert_eq!(
            fields.get(6).unwrap().identify_field_type(),
            TypeKindInField::Array(Box::new(TypeKindInField::Integer(IntegerKind::Int64))),
        );
        assert_eq!(
            fields.get(7).unwrap().identify_field_type(),
            TypeKindInField::Array(Box::new(TypeKindInField::Integer(IntegerKind::Float32))),
        );
        assert_eq!(
            fields.get(8).unwrap().identify_field_type(),
            TypeKindInField::Boolean(BooleanKind::True),
        );
        assert_eq!(
            fields.get(9).unwrap().identify_field_type(),
            TypeKindInField::Telegram("Type1".to_owned()),
        );

        let fields = scheme.types.get("Type2").unwrap().fields.as_slice();

        assert_eq!(
            fields.get(0).unwrap().identify_field_type(),
            TypeKindInField::Telegram("InputFile".to_owned()),
        );
        assert_eq!(
            fields.get(1).unwrap().identify_field_type(),
            TypeKindInField::Telegram("InputFile".to_owned()),
        );
        assert_eq!(
            fields.get(2).unwrap().identify_field_type(),
            TypeKindInField::Telegram("ReplyMarkup".to_owned())
        );
        assert_eq!(
            fields.get(3).unwrap().identify_field_type(),
            TypeKindInField::Telegram("ReplyKeyboardMarkup".to_owned()),
        );
        assert_eq!(
            fields.get(4).unwrap().identify_field_type(),
            TypeKindInField::Telegram("ChatId".to_owned()),
        );
        assert_eq!(
            fields.get(5).unwrap().identify_field_type(),
            TypeKindInField::Telegram("ChatId".to_owned()),
        );
        assert_eq!(
            fields.get(6).unwrap().identify_field_type(),
            TypeKindInField::Integer(IntegerKind::Int64),
        );
    }
}
