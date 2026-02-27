use crate::client::Bot;
use serde::Serialize;
/// Changes the first and last name of a managed business account. Requires the `can_change_name` business bot right. Returns `true` on success.
/// # Documentation
/// <https://core.telegram.org/bots/api#setbusinessaccountname>
/// # Returns
/// - `bool`
#[derive(Clone, Debug, Serialize)]
pub struct SetBusinessAccountName {
    /// Unique identifier of the business connection
    pub business_connection_id: Box<str>,
    /// The new value of the first name for the business account; 1-64 characters
    pub first_name: Box<str>,
    /// The new value of the last name for the business account; 0-64 characters
    #[serde(skip_serializing_if = "Option::is_none")]
    pub last_name: Option<Box<str>>,
}
impl SetBusinessAccountName {
    /// Creates a new `SetBusinessAccountName`.
    ///
    /// # Arguments
    /// * `business_connection_id` - Unique identifier of the business connection
    /// * `first_name` - The new value of the first name for the business account; 1-64 characters
    ///
    /// # Notes
    /// Use builder methods to set optional fields.
    #[must_use]
    pub fn new<T0: Into<Box<str>>, T1: Into<Box<str>>>(
        business_connection_id: T0,
        first_name: T1,
    ) -> Self {
        Self {
            business_connection_id: business_connection_id.into(),
            first_name: first_name.into(),
            last_name: None,
        }
    }

    /// Unique identifier of the business connection
    #[must_use]
    pub fn business_connection_id<T: Into<Box<str>>>(self, val: T) -> Self {
        let mut this = self;
        this.business_connection_id = val.into();
        this
    }

    /// The new value of the first name for the business account; 1-64 characters
    #[must_use]
    pub fn first_name<T: Into<Box<str>>>(self, val: T) -> Self {
        let mut this = self;
        this.first_name = val.into();
        this
    }

    /// The new value of the last name for the business account; 0-64 characters
    #[must_use]
    pub fn last_name<T: Into<Box<str>>>(self, val: T) -> Self {
        let mut this = self;
        this.last_name = Some(val.into());
        this
    }

    /// The new value of the last name for the business account; 0-64 characters
    #[must_use]
    pub fn last_name_option<T: Into<Box<str>>>(self, val: Option<T>) -> Self {
        let mut this = self;
        this.last_name = val.map(Into::into);
        this
    }
}
impl super::TelegramMethod for SetBusinessAccountName {
    type Method = Self;
    type Return = bool;

    fn build_request<Client>(self, _bot: &Bot<Client>) -> super::Request<Self::Method> {
        super::Request::new("setBusinessAccountName", self, None)
    }
}
