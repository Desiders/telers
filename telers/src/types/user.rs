use serde::{Deserialize, Serialize};
/// This object represents a Telegram user or bot.
/// # Documentation
/// <https://core.telegram.org/bots/api#user>
#[derive(Clone, Debug, Serialize, Deserialize)]
pub struct User {
    /// Unique identifier for this user or bot. This number may have more than 32 significant bits and some programming languages may have difficulty/silent defects in interpreting it. But it has at most 52 significant bits, so a 64-bit integer or double-precision float type are safe for storing this identifier.
    pub id: i64,
    /// `true`, if this user is a bot
    pub is_bot: bool,
    /// User's or bot's first name
    pub first_name: Box<str>,
    /// User's or bot's last name
    #[serde(skip_serializing_if = "Option::is_none")]
    pub last_name: Option<Box<str>>,
    /// User's or bot's username
    #[serde(skip_serializing_if = "Option::is_none")]
    pub username: Option<Box<str>>,
    /// IETF language tag of the user's language
    #[serde(skip_serializing_if = "Option::is_none")]
    pub language_code: Option<Box<str>>,
    /// `true`, if this user is a Telegram Premium user
    #[serde(skip_serializing_if = "Option::is_none")]
    pub is_premium: Option<bool>,
    /// `true`, if this user added the bot to the attachment menu
    #[serde(skip_serializing_if = "Option::is_none")]
    pub added_to_attachment_menu: Option<bool>,
    /// `true`, if the bot can be invited to groups. Returned only in getMe.
    #[serde(skip_serializing_if = "Option::is_none")]
    pub can_join_groups: Option<bool>,
    /// `true`, if privacy mode is disabled for the bot. Returned only in getMe.
    #[serde(skip_serializing_if = "Option::is_none")]
    pub can_read_all_group_messages: Option<bool>,
    /// `true`, if the bot supports guest queries from chats it is not a member of. Returned only in getMe.
    #[serde(skip_serializing_if = "Option::is_none")]
    pub supports_guest_queries: Option<bool>,
    /// `true`, if the bot supports inline queries. Returned only in getMe.
    #[serde(skip_serializing_if = "Option::is_none")]
    pub supports_inline_queries: Option<bool>,
    /// `true`, if the bot can be connected to a user account to manage it. Returned only in getMe.
    #[serde(skip_serializing_if = "Option::is_none")]
    pub can_connect_to_business: Option<bool>,
    /// `true`, if the bot has a main Web App. Returned only in getMe.
    #[serde(skip_serializing_if = "Option::is_none")]
    pub has_main_web_app: Option<bool>,
    /// `true`, if the bot has forum topic mode enabled in private chats. Returned only in getMe.
    #[serde(skip_serializing_if = "Option::is_none")]
    pub has_topics_enabled: Option<bool>,
    /// `true`, if the bot allows users to create and delete topics in private chats. Returned only in getMe.
    #[serde(skip_serializing_if = "Option::is_none")]
    pub allows_users_to_create_topics: Option<bool>,
    /// `true`, if other bots can be created to be controlled by the bot. Returned only in getMe.
    #[serde(skip_serializing_if = "Option::is_none")]
    pub can_manage_bots: Option<bool>,
    /// `true`, if the bot supports join request queries and can be assigned to process them. Returned only in getMe.
    #[serde(skip_serializing_if = "Option::is_none")]
    pub supports_join_request_queries: Option<bool>,
}
impl User {
    /// Creates a new `User`.
    ///
    /// # Arguments
    /// * `id` - Unique identifier for this user or bot. This number may have more than 32 significant bits and some programming languages may have difficulty/silent defects in interpreting it. But it has at most 52 significant bits, so a 64-bit integer or double-precision float type are safe for storing this identifier.
    /// * `is_bot` - `true`, if this user is a bot
    /// * `first_name` - User's or bot's first name
    ///
    /// # Notes
    /// Use builder methods to set optional fields.
    #[must_use]
    pub fn new<T0: Into<i64>, T1: Into<bool>, T2: Into<Box<str>>>(
        id: T0,
        is_bot: T1,
        first_name: T2,
    ) -> Self {
        Self {
            id: id.into(),
            is_bot: is_bot.into(),
            first_name: first_name.into(),
            last_name: None,
            username: None,
            language_code: None,
            is_premium: None,
            added_to_attachment_menu: None,
            can_join_groups: None,
            can_read_all_group_messages: None,
            supports_guest_queries: None,
            supports_inline_queries: None,
            can_connect_to_business: None,
            has_main_web_app: None,
            has_topics_enabled: None,
            allows_users_to_create_topics: None,
            can_manage_bots: None,
            supports_join_request_queries: None,
        }
    }

    /// Unique identifier for this user or bot. This number may have more than 32 significant bits and some programming languages may have difficulty/silent defects in interpreting it. But it has at most 52 significant bits, so a 64-bit integer or double-precision float type are safe for storing this identifier.
    #[must_use]
    pub fn id<T: Into<i64>>(mut self, val: T) -> Self {
        self.id = val.into();
        self
    }

    /// `true`, if this user is a bot
    #[must_use]
    pub fn is_bot<T: Into<bool>>(mut self, val: T) -> Self {
        self.is_bot = val.into();
        self
    }

    /// User's or bot's first name
    #[must_use]
    pub fn first_name<T: Into<Box<str>>>(mut self, val: T) -> Self {
        self.first_name = val.into();
        self
    }

    /// User's or bot's last name
    #[must_use]
    pub fn last_name<T: Into<Box<str>>>(mut self, val: T) -> Self {
        self.last_name = Some(val.into());
        self
    }

    /// User's or bot's last name
    #[must_use]
    pub fn last_name_option<T: Into<Box<str>>>(mut self, val: Option<T>) -> Self {
        self.last_name = val.map(Into::into);
        self
    }

    /// User's or bot's username
    #[must_use]
    pub fn username<T: Into<Box<str>>>(mut self, val: T) -> Self {
        self.username = Some(val.into());
        self
    }

    /// User's or bot's username
    #[must_use]
    pub fn username_option<T: Into<Box<str>>>(mut self, val: Option<T>) -> Self {
        self.username = val.map(Into::into);
        self
    }

    /// IETF language tag of the user's language
    #[must_use]
    pub fn language_code<T: Into<Box<str>>>(mut self, val: T) -> Self {
        self.language_code = Some(val.into());
        self
    }

    /// IETF language tag of the user's language
    #[must_use]
    pub fn language_code_option<T: Into<Box<str>>>(mut self, val: Option<T>) -> Self {
        self.language_code = val.map(Into::into);
        self
    }

    /// `true`, if this user is a Telegram Premium user
    #[must_use]
    pub fn is_premium<T: Into<bool>>(mut self, val: T) -> Self {
        self.is_premium = Some(val.into());
        self
    }

    /// `true`, if this user is a Telegram Premium user
    #[must_use]
    pub fn is_premium_option<T: Into<bool>>(mut self, val: Option<T>) -> Self {
        self.is_premium = val.map(Into::into);
        self
    }

    /// `true`, if this user added the bot to the attachment menu
    #[must_use]
    pub fn added_to_attachment_menu<T: Into<bool>>(mut self, val: T) -> Self {
        self.added_to_attachment_menu = Some(val.into());
        self
    }

    /// `true`, if this user added the bot to the attachment menu
    #[must_use]
    pub fn added_to_attachment_menu_option<T: Into<bool>>(mut self, val: Option<T>) -> Self {
        self.added_to_attachment_menu = val.map(Into::into);
        self
    }

    /// `true`, if the bot can be invited to groups. Returned only in getMe.
    #[must_use]
    pub fn can_join_groups<T: Into<bool>>(mut self, val: T) -> Self {
        self.can_join_groups = Some(val.into());
        self
    }

    /// `true`, if the bot can be invited to groups. Returned only in getMe.
    #[must_use]
    pub fn can_join_groups_option<T: Into<bool>>(mut self, val: Option<T>) -> Self {
        self.can_join_groups = val.map(Into::into);
        self
    }

    /// `true`, if privacy mode is disabled for the bot. Returned only in getMe.
    #[must_use]
    pub fn can_read_all_group_messages<T: Into<bool>>(mut self, val: T) -> Self {
        self.can_read_all_group_messages = Some(val.into());
        self
    }

    /// `true`, if privacy mode is disabled for the bot. Returned only in getMe.
    #[must_use]
    pub fn can_read_all_group_messages_option<T: Into<bool>>(mut self, val: Option<T>) -> Self {
        self.can_read_all_group_messages = val.map(Into::into);
        self
    }

    /// `true`, if the bot supports guest queries from chats it is not a member of. Returned only in getMe.
    #[must_use]
    pub fn supports_guest_queries<T: Into<bool>>(mut self, val: T) -> Self {
        self.supports_guest_queries = Some(val.into());
        self
    }

    /// `true`, if the bot supports guest queries from chats it is not a member of. Returned only in getMe.
    #[must_use]
    pub fn supports_guest_queries_option<T: Into<bool>>(mut self, val: Option<T>) -> Self {
        self.supports_guest_queries = val.map(Into::into);
        self
    }

    /// `true`, if the bot supports inline queries. Returned only in getMe.
    #[must_use]
    pub fn supports_inline_queries<T: Into<bool>>(mut self, val: T) -> Self {
        self.supports_inline_queries = Some(val.into());
        self
    }

    /// `true`, if the bot supports inline queries. Returned only in getMe.
    #[must_use]
    pub fn supports_inline_queries_option<T: Into<bool>>(mut self, val: Option<T>) -> Self {
        self.supports_inline_queries = val.map(Into::into);
        self
    }

    /// `true`, if the bot can be connected to a user account to manage it. Returned only in getMe.
    #[must_use]
    pub fn can_connect_to_business<T: Into<bool>>(mut self, val: T) -> Self {
        self.can_connect_to_business = Some(val.into());
        self
    }

    /// `true`, if the bot can be connected to a user account to manage it. Returned only in getMe.
    #[must_use]
    pub fn can_connect_to_business_option<T: Into<bool>>(mut self, val: Option<T>) -> Self {
        self.can_connect_to_business = val.map(Into::into);
        self
    }

    /// `true`, if the bot has a main Web App. Returned only in getMe.
    #[must_use]
    pub fn has_main_web_app<T: Into<bool>>(mut self, val: T) -> Self {
        self.has_main_web_app = Some(val.into());
        self
    }

    /// `true`, if the bot has a main Web App. Returned only in getMe.
    #[must_use]
    pub fn has_main_web_app_option<T: Into<bool>>(mut self, val: Option<T>) -> Self {
        self.has_main_web_app = val.map(Into::into);
        self
    }

    /// `true`, if the bot has forum topic mode enabled in private chats. Returned only in getMe.
    #[must_use]
    pub fn has_topics_enabled<T: Into<bool>>(mut self, val: T) -> Self {
        self.has_topics_enabled = Some(val.into());
        self
    }

    /// `true`, if the bot has forum topic mode enabled in private chats. Returned only in getMe.
    #[must_use]
    pub fn has_topics_enabled_option<T: Into<bool>>(mut self, val: Option<T>) -> Self {
        self.has_topics_enabled = val.map(Into::into);
        self
    }

    /// `true`, if the bot allows users to create and delete topics in private chats. Returned only in getMe.
    #[must_use]
    pub fn allows_users_to_create_topics<T: Into<bool>>(mut self, val: T) -> Self {
        self.allows_users_to_create_topics = Some(val.into());
        self
    }

    /// `true`, if the bot allows users to create and delete topics in private chats. Returned only in getMe.
    #[must_use]
    pub fn allows_users_to_create_topics_option<T: Into<bool>>(mut self, val: Option<T>) -> Self {
        self.allows_users_to_create_topics = val.map(Into::into);
        self
    }

    /// `true`, if other bots can be created to be controlled by the bot. Returned only in getMe.
    #[must_use]
    pub fn can_manage_bots<T: Into<bool>>(mut self, val: T) -> Self {
        self.can_manage_bots = Some(val.into());
        self
    }

    /// `true`, if other bots can be created to be controlled by the bot. Returned only in getMe.
    #[must_use]
    pub fn can_manage_bots_option<T: Into<bool>>(mut self, val: Option<T>) -> Self {
        self.can_manage_bots = val.map(Into::into);
        self
    }

    /// `true`, if the bot supports join request queries and can be assigned to process them. Returned only in getMe.
    #[must_use]
    pub fn supports_join_request_queries<T: Into<bool>>(mut self, val: T) -> Self {
        self.supports_join_request_queries = Some(val.into());
        self
    }

    /// `true`, if the bot supports join request queries and can be assigned to process them. Returned only in getMe.
    #[must_use]
    pub fn supports_join_request_queries_option<T: Into<bool>>(mut self, val: Option<T>) -> Self {
        self.supports_join_request_queries = val.map(Into::into);
        self
    }
}
