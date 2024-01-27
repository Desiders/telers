use crate::extractors::FromContext;

use serde::{Deserialize, Serialize};
use serde_with::skip_serializing_none;
use std::borrow::Cow;

/// This object represents a Telegram user or bot.
/// # Documentation
/// <https://core.telegram.org/bots/api#user>
#[skip_serializing_none]
#[derive(Debug, Default, Clone, Hash, PartialEq, Eq, Deserialize, Serialize, FromContext)]
#[context(key = "event_user")]
pub struct User {
    /// Unique identifier for this user or bot. This number may have more than 32 significant bits and some programming languages may have difficulty/silent defects in interpreting it. But it has at most 52 significant bits, so a 64-bit integer or double-precision float type are safe for storing this identifier.
    pub id: i64,
    /// `true`, if this user is a bot
    pub is_bot: bool,
    /// User's or bot's first name
    pub first_name: String,
    /// User's or bot's last name
    pub last_name: Option<String>,
    /// User's or bot's username
    pub username: Option<String>,
    /// [`IETF language tag`](https://en.wikipedia.org/wiki/IETF_language_tag) of the user's language
    pub language_code: Option<String>,
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

impl User {
    #[must_use]
    pub fn new(id: i64, is_bot: bool, first_name: impl Into<String>) -> Self {
        Self {
            id,
            is_bot,
            first_name: first_name.into(),
            last_name: None,
            username: None,
            language_code: None,
            is_premium: None,
            added_to_attachment_menu: None,
            can_join_groups: None,
            can_read_all_group_messages: None,
            supports_inline_queries: None,
        }
    }

    #[must_use]
    pub fn id(self, val: i64) -> Self {
        Self { id: val, ..self }
    }

    #[must_use]
    pub fn is_bot(self, val: bool) -> Self {
        Self {
            is_bot: val,
            ..self
        }
    }

    #[must_use]
    pub fn first_name(self, val: impl Into<String>) -> Self {
        Self {
            first_name: val.into(),
            ..self
        }
    }

    #[must_use]
    pub fn last_name(self, val: impl Into<String>) -> Self {
        Self {
            last_name: Some(val.into()),
            ..self
        }
    }

    #[must_use]
    pub fn username(self, val: impl Into<String>) -> Self {
        Self {
            username: Some(val.into()),
            ..self
        }
    }

    #[must_use]
    pub fn language_code(self, val: impl Into<String>) -> Self {
        Self {
            language_code: Some(val.into()),
            ..self
        }
    }

    #[must_use]
    pub fn is_premium(self, val: bool) -> Self {
        Self {
            is_premium: Some(val),
            ..self
        }
    }

    #[must_use]
    pub fn added_to_attachment_menu(self, val: bool) -> Self {
        Self {
            added_to_attachment_menu: Some(val),
            ..self
        }
    }

    #[must_use]
    pub fn can_join_groups(self, val: bool) -> Self {
        Self {
            can_join_groups: Some(val),
            ..self
        }
    }

    #[must_use]
    pub fn can_read_all_group_messages(self, val: bool) -> Self {
        Self {
            can_read_all_group_messages: Some(val),
            ..self
        }
    }

    #[must_use]
    pub fn supports_inline_queries(self, val: bool) -> Self {
        Self {
            supports_inline_queries: Some(val),
            ..self
        }
    }
}

impl User {
    #[must_use]
    pub fn last_name_option(self, val: Option<impl Into<String>>) -> Self {
        Self {
            last_name: val.map(Into::into),
            ..self
        }
    }

    #[must_use]
    pub fn username_option(self, val: Option<impl Into<String>>) -> Self {
        Self {
            username: val.map(Into::into),
            ..self
        }
    }

    #[must_use]
    pub fn language_code_option(self, val: Option<impl Into<String>>) -> Self {
        Self {
            language_code: val.map(Into::into),
            ..self
        }
    }

    #[must_use]
    pub fn is_premium_option(self, val: Option<bool>) -> Self {
        Self {
            is_premium: val,
            ..self
        }
    }

    #[must_use]
    pub fn added_to_attachment_menu_option(self, val: Option<bool>) -> Self {
        Self {
            added_to_attachment_menu: val,
            ..self
        }
    }

    #[must_use]
    pub fn can_join_groups_option(self, val: Option<bool>) -> Self {
        Self {
            can_join_groups: val,
            ..self
        }
    }

    #[must_use]
    pub fn can_read_all_group_messages_option(self, val: Option<bool>) -> Self {
        Self {
            can_read_all_group_messages: val,
            ..self
        }
    }

    #[must_use]
    pub fn supports_inline_queries_option(self, val: Option<bool>) -> Self {
        Self {
            supports_inline_queries: val,
            ..self
        }
    }
}
