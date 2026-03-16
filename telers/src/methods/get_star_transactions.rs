use crate::client::Bot;
use serde::Serialize;
/// Returns the bot's Telegram Star transactions in chronological order. On success, returns a [`crate::types::StarTransactions`] object.
/// # Documentation
/// <https://core.telegram.org/bots/api#getstartransactions>
/// # Returns
/// - `crate::types::StarTransactions`
#[derive(Clone, Debug, Serialize)]
pub struct GetStarTransactions {
    /// Number of transactions to skip in the response
    #[serde(skip_serializing_if = "Option::is_none")]
    pub offset: Option<i64>,
    /// The maximum number of transactions to be retrieved. Values between 1-100 are accepted. Defaults to 100.
    #[serde(skip_serializing_if = "Option::is_none")]
    pub limit: Option<u8>,
}
impl GetStarTransactions {
    /// Creates a new `GetStarTransactions`.
    ///
    /// # Notes
    /// Use builder methods to set optional fields.
    #[must_use]
    pub fn new() -> Self {
        Self {
            offset: None,
            limit: None,
        }
    }

    /// Number of transactions to skip in the response
    #[must_use]
    pub fn offset<T: Into<i64>>(self, val: T) -> Self {
        let mut this = self;
        this.offset = Some(val.into());
        this
    }

    /// Number of transactions to skip in the response
    #[must_use]
    pub fn offset_option<T: Into<i64>>(self, val: Option<T>) -> Self {
        let mut this = self;
        this.offset = val.map(Into::into);
        this
    }

    /// The maximum number of transactions to be retrieved. Values between 1-100 are accepted. Defaults to 100.
    #[must_use]
    pub fn limit<T: Into<u8>>(self, val: T) -> Self {
        let mut this = self;
        this.limit = Some(val.into());
        this
    }

    /// The maximum number of transactions to be retrieved. Values between 1-100 are accepted. Defaults to 100.
    #[must_use]
    pub fn limit_option<T: Into<u8>>(self, val: Option<T>) -> Self {
        let mut this = self;
        this.limit = val.map(Into::into);
        this
    }
}
impl Default for GetStarTransactions {
    fn default() -> Self {
        Self::new()
    }
}
impl super::TelegramMethod for GetStarTransactions {
    type Method = Self;
    type Return = crate::types::StarTransactions;

    fn build_request<Client>(self, _bot: &Bot<Client>) -> super::Request<Self::Method> {
        super::Request::new("getStarTransactions", self, None)
    }
}
