use crate::client::Bot;
use serde::Serialize;
/// Use this method to change the access settings of a managed bot. Returns `true` on success.
/// # Documentation
/// <https://core.telegram.org/bots/api#setmanagedbotaccesssettings>
/// # Returns
/// - `bool`
#[derive(Clone, Debug, Serialize)]
pub struct SetManagedBotAccessSettings {
    /// User identifier of the managed bot whose access settings will be changed
    pub user_id: i64,
    /// Pass `true`, if only selected users can access the bot. The bot's owner can always access it.
    pub is_access_restricted: bool,
    /// A JSON-serialized list of up to 10 identifiers of users who will have access to the bot in addition to its owner. Ignored if `is_access_restricted` is false.
    #[serde(skip_serializing_if = "Option::is_none")]
    pub added_user_ids: Option<Box<[i64]>>,
}
impl SetManagedBotAccessSettings {
    /// Creates a new `SetManagedBotAccessSettings`.
    ///
    /// # Arguments
    /// * `user_id` - User identifier of the managed bot whose access settings will be changed
    /// * `is_access_restricted` - Pass `true`, if only selected users can access the bot. The bot's owner can always access it.
    ///
    /// # Notes
    /// Use builder methods to set optional fields.
    #[must_use]
    pub fn new<T0: Into<i64>, T1: Into<bool>>(user_id: T0, is_access_restricted: T1) -> Self {
        Self {
            user_id: user_id.into(),
            is_access_restricted: is_access_restricted.into(),
            added_user_ids: None,
        }
    }

    /// User identifier of the managed bot whose access settings will be changed
    #[must_use]
    pub fn user_id<T: Into<i64>>(mut self, val: T) -> Self {
        self.user_id = val.into();
        self
    }

    /// Pass `true`, if only selected users can access the bot. The bot's owner can always access it.
    #[must_use]
    pub fn is_access_restricted<T: Into<bool>>(mut self, val: T) -> Self {
        self.is_access_restricted = val.into();
        self
    }

    /// A JSON-serialized list of up to 10 identifiers of users who will have access to the bot in addition to its owner. Ignored if `is_access_restricted` is false.
    ///
    /// # Notes
    /// Adds multiple elements.
    #[must_use]
    pub fn added_user_ids<TItem: Into<i64>, T: IntoIterator<Item = TItem>>(
        mut self,
        val: T,
    ) -> Self {
        self.added_user_ids = Some(
            self.added_user_ids
                .unwrap_or_default()
                .into_vec()
                .into_iter()
                .chain(val.into_iter().map(Into::into))
                .collect(),
        );
        self
    }

    /// A JSON-serialized list of up to 10 identifiers of users who will have access to the bot in addition to its owner. Ignored if `is_access_restricted` is false.
    ///
    /// # Notes
    /// Adds a single element.
    #[must_use]
    pub fn added_user_id<T: Into<i64>>(mut self, val: T) -> Self {
        self.added_user_ids = Some(
            self.added_user_ids
                .unwrap_or_default()
                .into_vec()
                .into_iter()
                .chain(Some(val.into()))
                .collect(),
        );
        self
    }

    /// A JSON-serialized list of up to 10 identifiers of users who will have access to the bot in addition to its owner. Ignored if `is_access_restricted` is false.
    ///
    /// # Notes
    /// Adds multiple elements.
    #[must_use]
    pub fn added_user_ids_option<TItem: Into<i64>, T: IntoIterator<Item = TItem>>(
        mut self,
        val: Option<T>,
    ) -> Self {
        self.added_user_ids = val.map(|v| v.into_iter().map(Into::into).collect());
        self
    }
}
impl super::TelegramMethod for SetManagedBotAccessSettings {
    type Method = Self;
    type Return = bool;

    fn build_request<Client>(self, _bot: &Bot<Client>) -> super::Request<Self::Method> {
        super::Request::new("setManagedBotAccessSettings", self, None)
    }
}
