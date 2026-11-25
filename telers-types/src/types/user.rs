/// This object represents a Telegram user or bot.
/// # Documentation
/// <https://core.telegram.org/bots/api#user>
#[derive(Clone, Debug)]
#[cfg_attr(feature = "deser", derive(deser::Serialize, deser::Deserialize))]
#[cfg_attr(not(feature = "deser"), derive(serde::Serialize, serde::Deserialize))]
pub struct User {
    /// Unique identifier for this user or bot. This number may have more than 32 significant bits and some programming languages may have difficulty/silent defects in interpreting it. But it has at most 52 significant bits, so a 64-bit integer or double-precision float type is safe for storing this identifier.
    pub id: i64,
    /// `true`, if this user is a bot
    pub is_bot: bool,
    /// User's or bot's first name
    pub first_name: Box<str>,
    /// User's or bot's last name
    #[cfg_attr(not(feature = "deser"), serde(skip_serializing_if = "Option::is_none"))]
    #[cfg_attr(feature = "deser", deser(skip_serializing_if = Option::is_none))]
    pub last_name: Option<Box<str>>,
    /// User's or bot's username
    #[cfg_attr(not(feature = "deser"), serde(skip_serializing_if = "Option::is_none"))]
    #[cfg_attr(feature = "deser", deser(skip_serializing_if = Option::is_none))]
    pub username: Option<Box<str>>,
    /// IETF language tag of the user's language
    #[cfg_attr(not(feature = "deser"), serde(skip_serializing_if = "Option::is_none"))]
    #[cfg_attr(feature = "deser", deser(skip_serializing_if = Option::is_none))]
    pub language_code: Option<Box<str>>,
    /// `true`, if this user is a Telegram Premium user
    #[cfg_attr(not(feature = "deser"), serde(skip_serializing_if = "Option::is_none"))]
    #[cfg_attr(feature = "deser", deser(skip_serializing_if = Option::is_none))]
    pub is_premium: Option<bool>,
    /// `true`, if this user added the bot to the attachment menu
    #[cfg_attr(not(feature = "deser"), serde(skip_serializing_if = "Option::is_none"))]
    #[cfg_attr(feature = "deser", deser(skip_serializing_if = Option::is_none))]
    pub added_to_attachment_menu: Option<bool>,
}
impl User {
    /// Creates a new `User`.
    ///
    /// # Arguments
    /// * `id` - Unique identifier for this user or bot. This number may have more than 32 significant bits and some programming languages may have difficulty/silent defects in interpreting it. But it has at most 52 significant bits, so a 64-bit integer or double-precision float type is safe for storing this identifier.
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
        }
    }

    /// Unique identifier for this user or bot. This number may have more than 32 significant bits and some programming languages may have difficulty/silent defects in interpreting it. But it has at most 52 significant bits, so a 64-bit integer or double-precision float type is safe for storing this identifier.
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
}
