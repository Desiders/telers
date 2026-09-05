//! This module contains the [`CallbackData`] trait and [`CallbackDataError`] enum,
//! which are used to pack structured data into a callback data string and unpack it back.
//!
//! The easiest way to implement [`CallbackData`] for your types is to use the [`CallbackData`]
//! derive macro:
//!
//! ```rust
//! use telers::{types::CallbackQuery, CallbackData, Filter, Request};
//!
//! #[derive(CallbackData, Clone)]
//! #[callback_data(prefix = "language")]
//! struct LanguageSettings {
//!     language_code: String,
//!     enabled: bool,
//! }
//!
//! // Packing data to a string and sending it with a button
//! let callback_data = LanguageSettings {
//!     language_code: "en".into(),
//!     enabled: true,
//! }
//! .pack()
//! .unwrap();
//! assert_eq!(callback_data, "language:en:true");
//!
//! // Unpacking data from a callback query string
//! let unpacked = LanguageSettings::unpack("language:en:true").unwrap();
//! assert_eq!(unpacked.language_code, "en");
//! assert!(unpacked.enabled);
//! ```
//!
//! You can use the [`CallbackDataFilter`] to filter [`CallbackQuery`] updates and unpack the data
//! into the request context. Then you can extract it in handlers via the derived extractor
//! implementation (provided by the [`CallbackData`] derive macro):
//!
//! ```rust
//! use telers::{enums::UpdateType, event::bases::PropagateEventResult, event::telegram::Handler, event::telegram::HandlerResult, event::EventReturn, filters::callback_data::CallbackDataFilter, router::PropagateEvent, types::Update, Bot, Request, Router};
//! # use telers::{CallbackData, types::{CallbackQuery, User, UpdateCallbackQuery}};
//! # use std::sync::Arc;
//!
//! # #[derive(CallbackData, Clone)]
//! # #[callback_data(prefix = "language")]
//! # struct LanguageSettings {
//! #     language_code: String,
//! #     enabled: bool,
//! # }
//!
//! async fn handle_settings(bot: Bot, settings: LanguageSettings) -> HandlerResult {
//!     // Here you can be sure that the callback query data is `LanguageSettings`
//!     // and use it, for example, to update the settings
//!     Ok(EventReturn::Finish)
//! }
//!
//! # async fn register() {
//! let router: Router = Router::new("callback settings").on_callback_query(|observer| {
//!     observer
//!         .filter(CallbackDataFilter::<LanguageSettings>::new())
//!         .register(Handler::new(handle_settings))
//! });
//! # let mut request = Request {
//! #     bot: Bot::default(),
//! #     update: Arc::new(Update::CallbackQuery(UpdateCallbackQuery::new(0, CallbackQuery::new("id", User::new(1, true, "test"), "chat instance").data("language:en:true")))),
//! #     context: Default::default(),
//! #     extensions: Default::default(),
//! # };
//! # let mut router = router.configure_default();
//! # let response = router.propagate_event(UpdateType::CallbackQuery, request).await.unwrap();
//! # assert!(matches!(response.propagate_result, PropagateEventResult::Handled(_)));
//! # }
//! ```
//!
//! [`CallbackQuery`]: crate::types::CallbackQuery
//! [`Filter`]: crate::filters::Filter

/// Maximum length of the callback data string in bytes
/// (<https://core.telegram.org/bots/api#callbackquery>)
pub const MAX_CALLBACK_LENGTH: usize = 64;

/// Default separator used to join the prefix and values in the callback data string
pub const DEFAULT_SEPARATOR: char = ':';

/// An error that can occur while packing or unpacking [`CallbackData`].
#[derive(Debug, Clone, thiserror::Error)]
#[non_exhaustive]
pub enum CallbackDataError {
    /// The prefix or a value contains the separator character
    #[error("Value `{0}` contains the separator `{1}`")]
    SeparatorInValue(Box<str>, char),
    /// The callback data string is too long, the maximum length is [`MAX_CALLBACK_LENGTH`] bytes
    #[error("Callback data is too long: {length} bytes, but the maximum is {max} bytes")]
    TooLong {
        /// Length of the callback data string in bytes
        length: usize,
        /// Maximum length of the callback data string in bytes ([`MAX_CALLBACK_LENGTH`])
        max: usize,
    },
    /// The callback data string has a prefix that doesn't match the expected prefix
    #[error(
        "Callback data prefix mismatch: expected `{expected}`, but got `{actual}`. It looks like \
         this callback data belongs to another callback data type or was corrupted"
    )]
    PrefixMismatch {
        /// Expected prefix
        expected: Box<str>,
        /// Actual prefix of the callback data string
        actual: Box<str>,
    },
    /// The callback data string doesn't contain the expected number of values
    #[error("Callback data contains {actual} values, but {expected} are expected")]
    FieldCountMismatch {
        /// Number of expected values (the number of fields in the struct)
        expected: usize,
        /// Number of values in the callback data string
        actual: usize,
    },
    /// A field value can't be parsed from the callback data string
    #[error("Failed to parse value `{value}` of the field `{field}`")]
    InvalidValue {
        /// Name of the field
        field: &'static str,
        /// Value that can't be parsed
        value: Box<str>,
    },
}

/// Encodes a field value to a string representation and parses it back.
///
/// This trait is implemented for the primitive types which are safe to use
/// in callback data strings. Implement it for your types if you want to use them
/// as fields of [`CallbackData`] structs.
pub trait CallbackDataValue: Sized {
    /// Encodes the value to a string representation
    ///
    /// # Errors
    /// If the value can't be encoded, for example, if it contains the [`DEFAULT_SEPARATOR`]
    fn encode(&self) -> Result<String, CallbackDataError>;

    /// Parses the value from a string representation
    ///
    /// # Errors
    /// If the value can't be parsed from the string
    fn decode(value: &str, field: &'static str) -> Result<Self, CallbackDataError>;
}

/// A trait for types that can be packed to a callback data string and unpacked from it.
///
/// The easiest way to implement this trait is to use the [`CallbackData`] derive macro.
///
/// [`CallbackData`]: derive@crate::CallbackData
pub trait CallbackData: Sized {
    /// Prefix of the callback data string, which is used to identify the callback data type
    const PREFIX: &'static str;

    /// Separator used to join the prefix and values in the callback data string
    const SEPARATOR: char = DEFAULT_SEPARATOR;

    /// Packs the struct to a callback data string
    ///
    /// # Errors
    /// - If the prefix or a value contains the separator character
    /// - If the resulting string is longer than [`MAX_CALLBACK_LENGTH`] bytes
    fn pack(&self) -> Result<String, CallbackDataError>;

    /// Unpacks the callback data string to the struct
    ///
    /// # Errors
    /// - If the prefix of the callback data string doesn't match [`CallbackData::PREFIX`]
    /// - If the number of values doesn't match the number of fields
    /// - If a value can't be parsed to the field type
    fn unpack(value: &str) -> Result<Self, CallbackDataError>;
}

impl CallbackDataValue for String {
    #[inline]
    fn encode(&self) -> Result<String, CallbackDataError> {
        if self.contains(DEFAULT_SEPARATOR) {
            Err(CallbackDataError::SeparatorInValue(
                self.as_str().into(),
                DEFAULT_SEPARATOR,
            ))
        } else {
            Ok(self.clone())
        }
    }

    #[inline]
    fn decode(value: &str, _field: &'static str) -> Result<Self, CallbackDataError> {
        Ok(value.into())
    }
}

impl CallbackDataValue for Box<str> {
    #[inline]
    fn encode(&self) -> Result<String, CallbackDataError> {
        if self.contains(DEFAULT_SEPARATOR) {
            Err(CallbackDataError::SeparatorInValue(
                self.as_ref().into(),
                DEFAULT_SEPARATOR,
            ))
        } else {
            Ok(self.to_string())
        }
    }

    #[inline]
    fn decode(value: &str, _field: &'static str) -> Result<Self, CallbackDataError> {
        Ok(value.into())
    }
}

macro_rules! impl_callback_data_value_display_parse {
    ($($ty:ty),* $(,)?) => {
        $(
            impl CallbackDataValue for $ty {
                #[inline]
                fn encode(&self) -> Result<String, CallbackDataError> {
                    Ok(self.to_string())
                }

                #[inline]
                fn decode(value: &str, field: &'static str) -> Result<Self, CallbackDataError> {
                    value
                        .parse()
                        .map_err(|_| CallbackDataError::InvalidValue {
                            field,
                            value: value.into(),
                        })
                }
            }
        )*
    };
}

impl_callback_data_value_display_parse!(
    i8, i16, i32, i64, i128, isize, u8, u16, u32, u64, u128, usize
);

impl CallbackDataValue for bool {
    #[inline]
    fn encode(&self) -> Result<String, CallbackDataError> {
        Ok(if *self { "true" } else { "false" }.into())
    }

    #[inline]
    fn decode(value: &str, field: &'static str) -> Result<Self, CallbackDataError> {
        match value {
            "true" => Ok(true),
            "false" => Ok(false),
            _ => Err(CallbackDataError::InvalidValue {
                field,
                value: value.into(),
            }),
        }
    }
}

impl<T: CallbackDataValue> CallbackDataValue for Option<T> {
    #[inline]
    fn encode(&self) -> Result<String, CallbackDataError> {
        match self {
            Some(value) => value.encode(),
            None => Ok(String::new()),
        }
    }

    #[inline]
    fn decode(value: &str, field: &'static str) -> Result<Self, CallbackDataError> {
        if value.is_empty() {
            Ok(None)
        } else {
            T::decode(value, field).map(Some)
        }
    }
}

/// Validates that the prefix and values don't contain the separator character,
/// joins the prefix and values with the separator,
/// and checks that the resulting string isn't longer than [`MAX_CALLBACK_LENGTH`] bytes.
///
/// This helper is used by the [`CallbackData`] implementations to pack values.
/// Prefer the [`CallbackData`] derive macro instead of implementing [`CallbackData`] manually.
///
/// # Errors
/// - If the prefix contains the separator character
/// - If a value contains the separator character
/// - If the resulting string is longer than [`MAX_CALLBACK_LENGTH`] bytes
pub fn pack_values(
    prefix: &'static str,
    separator: char,
    values: &[Result<String, CallbackDataError>],
) -> Result<String, CallbackDataError> {
    if prefix.contains(separator) {
        return Err(CallbackDataError::SeparatorInValue(
            prefix.into(),
            separator,
        ));
    }

    let mut packed = Vec::with_capacity(values.len());
    for value in values {
        let value = value.as_ref().map_err(Clone::clone)?;
        if value.contains(separator) {
            return Err(CallbackDataError::SeparatorInValue(
                value.clone().into(),
                separator,
            ));
        }
        packed.push(value.clone());
    }

    let callback_data = std::iter::once(prefix.to_string())
        .chain(packed)
        .collect::<Vec<_>>()
        .join(&separator.to_string());

    let length = callback_data.len();
    if length > MAX_CALLBACK_LENGTH {
        return Err(CallbackDataError::TooLong {
            length,
            max: MAX_CALLBACK_LENGTH,
        });
    }

    Ok(callback_data)
}

/// Splits the callback data string to the prefix and values by the separator
/// and checks that the prefix matches the expected prefix.
///
/// This helper is used by the [`CallbackData`] implementations to unpack values.
/// Prefer the [`CallbackData`] derive macro instead of implementing [`CallbackData`] manually.
///
/// # Errors
/// - If the prefix of the callback data string doesn't match the expected prefix
/// - If the number of values doesn't match the expected number of values
pub fn unpack_values<'a>(
    value: &'a str,
    prefix: &'static str,
    separator: char,
    expected_values: usize,
) -> Result<Box<[&'a str]>, CallbackDataError> {
    let mut parts = value.split(separator);

    let actual_prefix = parts.next().unwrap_or_default();
    if actual_prefix != prefix {
        return Err(CallbackDataError::PrefixMismatch {
            expected: prefix.into(),
            actual: actual_prefix.into(),
        });
    }

    let values = parts.collect::<Vec<&'a str>>();
    if values.len() != expected_values {
        return Err(CallbackDataError::FieldCountMismatch {
            expected: expected_values,
            actual: values.len(),
        });
    }

    Ok(values.into_boxed_slice())
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn pack_values_with_separator_in_value() {
        assert!(matches!(
            pack_values("prefix", DEFAULT_SEPARATOR, &["value:with:separator".to_string().encode()]),
            Err(CallbackDataError::SeparatorInValue(value, ':')) if value.as_ref() == "value:with:separator",
        ));
    }

    #[test]
    fn pack_values_with_separator_in_prefix() {
        assert!(matches!(
            pack_values("pre:fix", DEFAULT_SEPARATOR, &[]),
            Err(CallbackDataError::SeparatorInValue(value, ':')) if value.as_ref() == "pre:fix",
        ));
    }

    #[test]
    fn pack_values_too_long() {
        let values = vec!["a".repeat(MAX_CALLBACK_LENGTH).encode()];
        let expected_length = MAX_CALLBACK_LENGTH + "prefix".len() + 1;
        assert!(matches!(
            pack_values("prefix", DEFAULT_SEPARATOR, &values),
            Err(CallbackDataError::TooLong {
                length,
                max: MAX_CALLBACK_LENGTH,
            }) if length == expected_length,
        ));
    }

    #[test]
    fn unpack_values_with_prefix_mismatch() {
        assert!(matches!(
            unpack_values("wrong_prefix:value", "prefix", DEFAULT_SEPARATOR, 1),
            Err(CallbackDataError::PrefixMismatch {
                ref expected,
                ref actual,
            }) if expected.as_ref() == "prefix" && actual.as_ref() == "wrong_prefix",
        ));
    }

    #[test]
    fn unpack_values_with_field_count_mismatch() {
        assert!(matches!(
            unpack_values("prefix:value1:value2", "prefix", DEFAULT_SEPARATOR, 1),
            Err(CallbackDataError::FieldCountMismatch {
                expected: 1,
                actual: 2,
            }),
        ));
    }

    #[test]
    fn pack_and_unpack_values() {
        assert_eq!(
            pack_values(
                "prefix",
                DEFAULT_SEPARATOR,
                &["value1".to_string().encode(), "value2".to_string().encode()]
            )
            .unwrap(),
            "prefix:value1:value2",
        );

        let values = unpack_values("prefix:value1:value2", "prefix", DEFAULT_SEPARATOR, 2).unwrap();
        assert_eq!(values.as_ref(), ["value1", "value2"]);
    }

    #[test]
    fn value_encode_decode_roundtrip() {
        assert_eq!(String::from("test").encode().unwrap(), "test");
        assert_eq!(Box::<str>::from("test").encode().unwrap(), "test");
        assert_eq!(1i32.encode().unwrap(), "1");
        assert_eq!(true.encode().unwrap(), "true");
        assert_eq!(Some(1i32).encode().unwrap(), "1");
        assert_eq!(None::<i32>.encode().unwrap(), "");

        assert_eq!(i32::decode("42", "field").unwrap(), 42);
        assert!(bool::decode("true", "field").unwrap());
        assert_eq!(Option::<i32>::decode("", "field").unwrap(), None);
        assert_eq!(Option::<i32>::decode("42", "field").unwrap(), Some(42));

        assert!(i32::decode("nope", "field").is_err());
        assert!(bool::decode("1", "field").is_err());
    }
}
