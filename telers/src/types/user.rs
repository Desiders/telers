use crate::FromContext;

use serde::{Deserialize, Serialize};
use serde_with::skip_serializing_none;
use std::borrow::Cow;

/// This object represents a Telegram user or bot.
/// # Documentation
/// <https://core.telegram.org/bots/api#user>
#[skip_serializing_none]
#[derive(Debug, Default, Clone, Hash, PartialEq, Eq, Deserialize, Serialize, FromContext)]
#[context(
    key = "event_user",
    description = "This object represents a Telegram user or bot. \
    This context is available only if `UserContext` middleware is used (default middleware) and user in `Update` is not empty."
)]
pub struct User {
    /// Unique identifier for this user or bot. This number may have more than 32 significant bits and some programming languages may have difficulty/silent defects in interpreting it. But it has at most 52 significant bits, so a 64-bit integer or double-precision float type are safe for storing this identifier.
    pub id: i64,
    /// `true`, if this user is a bot
    pub is_bot: bool,
    /// User's or bot's first name
    pub first_name: Box<str>,
    /// User's or bot's last name
    pub last_name: Option<Box<str>>,
    /// User's or bot's username
    pub username: Option<Box<str>>,
    /// [`IETF language tag`](https://en.wikipedia.org/wiki/IETF_language_tag) of the user's language
    pub language_code: Option<Box<str>>,
    /// `true`, if this user is a Telegram Premium user
    pub is_premium: Option<bool>,
    /// `true`, if this user added the bot to the attachment menu
    pub added_to_attachment_menu: Option<bool>,
    /// `true`, if the bot can be invited to groups. Returned only in [`GetMe`](crate::methods::GetMe).
    pub can_join_groups: Option<bool>,
    /// `true`, if [`privacy mode`](https://core.telegram.org/bots/features#privacy-modee) is disabled for the bot. Returned only in [`GetMe`](crate::methods::GetMe).
    pub can_read_all_group_messages: Option<bool>,
    /// `true`, if the bot supports inline queries. Returned only in [`GetMe`](crate::methods::GetMe).
    pub supports_inline_queries: Option<bool>,
    /// `true`, if the bot can be connected to a Telegram Business account to receive its messages. Returned only in [`GetMe`](crate::methods::GetMe).
    pub can_connect_to_business: Option<bool>,
    /// `true`, if the bot has a main Web App. Returned only in [`GetMe`](crate::methods::GetMe).
    pub has_main_web_app: Option<bool>,
}

impl User {
    #[must_use]
    pub fn full_name(&self) -> Cow<'_, str> {
        match (&self.first_name, &self.last_name) {
            (first_name, Some(last_name)) => Cow::Owned(format!("{first_name} {last_name}")),
            (first_name, None) => Cow::Borrowed(first_name),
        }
    }
}
