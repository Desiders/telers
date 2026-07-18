use serde::{Deserialize, Serialize};
use std::collections::BTreeMap;
/// This object represents a Chat unknown to this version of the library.
/// # Notes
/// Fields shared by all known variants are parsed as usual; everything else is kept in `extra`, so the object can be inspected and reserialized without data loss.
/// # Documentation
/// <https://core.telegram.org/bots/api#chat>
#[derive(Clone, Debug, Serialize, Deserialize)]
pub struct ChatUnknown {
    /// Raw `type` value of the variant unknown to this version of the library
    pub r#type: Box<str>,
    /// Unique identifier for this chat. This number may have more than 32 significant bits and some programming languages may have difficulty/silent defects in interpreting it. But it has at most 52 significant bits, so a signed 64-bit integer or double-precision float type are safe for storing this identifier.
    pub id: i64,
    /// `true`, if the chat is the direct messages chat of a channel
    #[serde(skip_serializing_if = "Option::is_none")]
    pub is_direct_messages: Option<bool>,
    #[serde(flatten)]
    pub extra: BTreeMap<Box<str>, serde_json::Value>,
}
impl ChatUnknown {
    /// Creates a new `ChatUnknown`.
    ///
    /// # Arguments
    /// * `type` - Raw `type` value of the variant unknown to this version of the library
    /// * `id` - Unique identifier for this chat. This number may have more than 32 significant bits and some programming languages may have difficulty/silent defects in interpreting it. But it has at most 52 significant bits, so a signed 64-bit integer or double-precision float type are safe for storing this identifier.
    ///
    /// # Notes
    /// Use builder methods to set optional fields.
    #[must_use]
    pub fn new<T0: Into<Box<str>>, T1: Into<i64>>(r#type: T0, id: T1) -> Self {
        Self {
            r#type: r#type.into(),
            id: id.into(),
            is_direct_messages: None,
            extra: BTreeMap::new(),
        }
    }

    /// Raw `type` value of the variant unknown to this version of the library
    #[must_use]
    pub fn r#type<T: Into<Box<str>>>(mut self, val: T) -> Self {
        self.r#type = val.into();
        self
    }

    /// Unique identifier for this chat. This number may have more than 32 significant bits and some programming languages may have difficulty/silent defects in interpreting it. But it has at most 52 significant bits, so a signed 64-bit integer or double-precision float type are safe for storing this identifier.
    #[must_use]
    pub fn id<T: Into<i64>>(mut self, val: T) -> Self {
        self.id = val.into();
        self
    }

    /// `true`, if the chat is the direct messages chat of a channel
    #[must_use]
    pub fn is_direct_messages<T: Into<bool>>(mut self, val: T) -> Self {
        self.is_direct_messages = Some(val.into());
        self
    }

    /// `true`, if the chat is the direct messages chat of a channel
    #[must_use]
    pub fn is_direct_messages_option<T: Into<bool>>(mut self, val: Option<T>) -> Self {
        self.is_direct_messages = val.map(Into::into);
        self
    }
}
