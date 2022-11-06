use serde::{Deserialize, Serialize};

/// This object represents a Telegram user or bot.
/// <https://core.telegram.org/bots/api#user>
#[derive(Default, Clone, Debug, Eq, Hash, PartialEq, Serialize, Deserialize)]
pub struct User {
    /// Unique identifier for this user or bot. This number may have more than 32 significant bits and some programming languages may have difficulty/silent defects in interpreting it. But it has at most 52 significant bits, so a 64-bit integer or double-precision float type are safe for storing this identifier.
    pub id: i64,
    /// `True`, if this user is a bot
    pub is_bot: bool,
    /// User's or bot's first name
    pub first_name: String,
    /// *Optional*. User's or bot's last name
    pub last_name: Option<String>,
    /// *Optional*. User's or bot's username
    pub username: Option<String>,
    /// *Optional*. `IETF language tag <https://en.wikipedia.org/wiki/IETF_language_tag>` of the user's language
    pub language_code: Option<String>,
    /// *Optional*. `True`, if this user is a Telegram Premium user
    pub is_premium: Option<bool>,
    /// *Optional*. `True`, if this user added the bot to the attachment menu
    pub added_to_attachment_menu: Option<bool>,
    /// *Optional*. `True`, if the bot can be invited to groups. Returned only in `aiogram_rs.methods.get_me.GetMe`.
    pub can_join_groups: Option<bool>,
    /// *Optional*. `True`, if `privacy mode <https://core.telegram.org/bots/features#privacy-modee>` is disabled for the bot. Returned only in `aiogram_rs.methods.get_me.GetMe`.
    pub can_read_all_group_messages: Option<bool>,
    /// *Optional*. `True`, if the bot supports inline queries. Returned only in `aiogram_rs.methods.get_me.GetMe`.
    pub supports_inline_queries: Option<bool>,
}

impl User {
    #[must_use]
    pub fn full_name(&self) -> String {
        if let Some(ref last_name) = self.last_name {
            format!("{} {last_name}", self.first_name)
        } else {
            self.first_name.clone()
        }
    }
}
