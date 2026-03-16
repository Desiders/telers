use crate::client::Bot;
use serde::Serialize;
/// Changes the username of a managed business account. Requires the `can_change_username` business bot right. Returns `true` on success.
/// # Documentation
/// <https://core.telegram.org/bots/api#setbusinessaccountusername>
/// # Returns
/// - `bool`
#[derive(Clone, Debug, Serialize)]
pub struct SetBusinessAccountUsername {
    /// Unique identifier of the business connection
    pub business_connection_id: Box<str>,
    /// The new value of the username for the business account; 0-32 characters
    #[serde(skip_serializing_if = "Option::is_none")]
    pub username: Option<Box<str>>,
}
impl SetBusinessAccountUsername {
    /// Creates a new `SetBusinessAccountUsername`.
    ///
    /// # Arguments
    /// * `business_connection_id` - Unique identifier of the business connection
    ///
    /// # Notes
    /// Use builder methods to set optional fields.
    #[must_use]
    pub fn new<T0: Into<Box<str>>>(business_connection_id: T0) -> Self {
        Self {
            business_connection_id: business_connection_id.into(),
            username: None,
        }
    }

    /// Unique identifier of the business connection
    #[must_use]
    pub fn business_connection_id<T: Into<Box<str>>>(self, val: T) -> Self {
        let mut this = self;
        this.business_connection_id = val.into();
        this
    }

    /// The new value of the username for the business account; 0-32 characters
    #[must_use]
    pub fn username<T: Into<Box<str>>>(self, val: T) -> Self {
        let mut this = self;
        this.username = Some(val.into());
        this
    }

    /// The new value of the username for the business account; 0-32 characters
    #[must_use]
    pub fn username_option<T: Into<Box<str>>>(self, val: Option<T>) -> Self {
        let mut this = self;
        this.username = val.map(Into::into);
        this
    }
}
impl super::TelegramMethod for SetBusinessAccountUsername {
    type Method = Self;
    type Return = bool;

    fn build_request<Client>(self, _bot: &Bot<Client>) -> super::Request<Self::Method> {
        super::Request::new("setBusinessAccountUsername", self, None)
    }
}
