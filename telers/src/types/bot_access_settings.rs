use serde::{Deserialize, Serialize};
/// This object describes the access settings of a bot.
/// # Documentation
/// <https://core.telegram.org/bots/api#botaccesssettings>
#[derive(Clone, Debug, Serialize, Deserialize)]
pub struct BotAccessSettings {
    /// `true`, if only selected users can access the bot. The bot's owner can always access it.
    pub is_access_restricted: bool,
    /// The list of other users who have access to the bot if the access is restricted
    #[serde(skip_serializing_if = "Option::is_none")]
    pub added_users: Option<Box<[crate::types::User]>>,
}
impl BotAccessSettings {
    /// Creates a new `BotAccessSettings`.
    ///
    /// # Arguments
    /// * `is_access_restricted` - `true`, if only selected users can access the bot. The bot's owner can always access it.
    ///
    /// # Notes
    /// Use builder methods to set optional fields.
    #[must_use]
    pub fn new<T0: Into<bool>>(is_access_restricted: T0) -> Self {
        Self {
            is_access_restricted: is_access_restricted.into(),
            added_users: None,
        }
    }

    /// `true`, if only selected users can access the bot. The bot's owner can always access it.
    #[must_use]
    pub fn is_access_restricted<T: Into<bool>>(mut self, val: T) -> Self {
        self.is_access_restricted = val.into();
        self
    }

    /// The list of other users who have access to the bot if the access is restricted
    ///
    /// # Notes
    /// Adds multiple elements.
    #[must_use]
    pub fn added_users<T: Into<Box<[crate::types::User]>>>(mut self, val: T) -> Self {
        self.added_users = Some(
            self.added_users
                .unwrap_or_default()
                .into_vec()
                .into_iter()
                .chain(val.into())
                .collect(),
        );
        self
    }

    /// The list of other users who have access to the bot if the access is restricted
    ///
    /// # Notes
    /// Adds a single element.
    #[must_use]
    pub fn added_user<T: Into<crate::types::User>>(mut self, val: T) -> Self {
        self.added_users = Some(
            self.added_users
                .unwrap_or_default()
                .into_vec()
                .into_iter()
                .chain(Some(val.into()))
                .collect(),
        );
        self
    }

    /// The list of other users who have access to the bot if the access is restricted
    ///
    /// # Notes
    /// Adds a single element.
    #[must_use]
    pub fn added_users_option<T: Into<Box<[crate::types::User]>>>(
        mut self,
        val: Option<T>,
    ) -> Self {
        self.added_users = val.map(Into::into);
        self
    }
}
