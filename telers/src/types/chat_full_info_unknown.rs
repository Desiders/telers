use serde::{Deserialize, Serialize};
use std::collections::BTreeMap;
/// This object represents a [`crate::types::ChatFullInfo`] unknown to this version of the library.
/// # Notes
/// Fields shared by all known variants are parsed as usual; everything else is kept in `extra`, so the object can be inspected and reserialized without data loss.
/// # Documentation
/// <https://core.telegram.org/bots/api#chatfullinfo>
#[derive(Clone, Debug, Serialize, Deserialize)]
pub struct ChatFullInfoUnknown {
    /// Raw `type` value of the variant unknown to this version of the library
    pub r#type: Box<str>,
    /// Unique identifier for this chat. This number may have more than 32 significant bits and some programming languages may have difficulty/silent defects in interpreting it. But it has at most 52 significant bits, so a signed 64-bit integer or double-precision float type are safe for storing this identifier.
    pub id: i64,
    /// Identifier of the accent color for the chat name and backgrounds of the chat photo, reply header, and link preview. See accent colors for more details.
    pub accent_color_id: i64,
    /// The maximum number of reactions that can be set on a message in the chat
    pub max_reaction_count: i64,
    /// Information about types of gifts that are accepted by the chat or by the corresponding user for private chats
    pub accepted_gift_types: crate::types::AcceptedGiftTypes,
    #[serde(flatten)]
    pub extra: BTreeMap<Box<str>, serde_json::Value>,
}
impl ChatFullInfoUnknown {
    /// Creates a new `ChatFullInfoUnknown`.
    ///
    /// # Arguments
    /// * `type` - Raw `type` value of the variant unknown to this version of the library
    /// * `id` - Unique identifier for this chat. This number may have more than 32 significant bits and some programming languages may have difficulty/silent defects in interpreting it. But it has at most 52 significant bits, so a signed 64-bit integer or double-precision float type are safe for storing this identifier.
    /// * `accent_color_id` - Identifier of the accent color for the chat name and backgrounds of the chat photo, reply header, and link preview. See accent colors for more details.
    /// * `max_reaction_count` - The maximum number of reactions that can be set on a message in the chat
    /// * `accepted_gift_types` - Information about types of gifts that are accepted by the chat or by the corresponding user for private chats
    #[must_use]
    pub fn new<
        T0: Into<Box<str>>,
        T1: Into<i64>,
        T2: Into<i64>,
        T3: Into<i64>,
        T4: Into<crate::types::AcceptedGiftTypes>,
    >(
        r#type: T0,
        id: T1,
        accent_color_id: T2,
        max_reaction_count: T3,
        accepted_gift_types: T4,
    ) -> Self {
        Self {
            r#type: r#type.into(),
            id: id.into(),
            accent_color_id: accent_color_id.into(),
            max_reaction_count: max_reaction_count.into(),
            accepted_gift_types: accepted_gift_types.into(),
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

    /// Identifier of the accent color for the chat name and backgrounds of the chat photo, reply header, and link preview. See accent colors for more details.
    #[must_use]
    pub fn accent_color_id<T: Into<i64>>(mut self, val: T) -> Self {
        self.accent_color_id = val.into();
        self
    }

    /// The maximum number of reactions that can be set on a message in the chat
    #[must_use]
    pub fn max_reaction_count<T: Into<i64>>(mut self, val: T) -> Self {
        self.max_reaction_count = val.into();
        self
    }

    /// Information about types of gifts that are accepted by the chat or by the corresponding user for private chats
    #[must_use]
    pub fn accepted_gift_types<T: Into<crate::types::AcceptedGiftTypes>>(mut self, val: T) -> Self {
        self.accepted_gift_types = val.into();
        self
    }
}
