use crate::client::Bot;
use serde::Serialize;
/// Informs a user that some of the Telegram Passport elements they provided contains errors. The user will not be able to re-submit their Passport to you until the errors are fixed (the contents of the field for which you returned the error must change). Returns `true` on success.
/// Use this if the data submitted by the user doesn't satisfy the standards your service requires for any reason. For example, if a birthday date seems invalid, a submitted document is blurry, a scan shows evidence of tampering, etc. Supply some details in the error message to make sure the user knows how to correct the issues.
/// # Documentation
/// <https://core.telegram.org/bots/api#setpassportdataerrors>
/// # Returns
/// - `bool`
#[derive(Clone, Debug, Serialize)]
pub struct SetPassportDataErrors {
    /// User identifier
    pub user_id: i64,
    /// A JSON-serialized array describing the errors
    pub errors: Box<[crate::types::PassportElementError]>,
}
impl SetPassportDataErrors {
    /// Creates a new `SetPassportDataErrors`.
    ///
    /// # Arguments
    /// * `user_id` - User identifier
    /// * `errors` - A JSON-serialized array describing the errors
    #[must_use]
    pub fn new<
        T0: Into<i64>,
        T1Item: Into<crate::types::PassportElementError>,
        T1: IntoIterator<Item = T1Item>,
    >(
        user_id: T0,
        errors: T1,
    ) -> Self {
        Self {
            user_id: user_id.into(),
            errors: errors.into_iter().map(Into::into).collect(),
        }
    }

    /// User identifier
    #[must_use]
    pub fn user_id<T: Into<i64>>(mut self, val: T) -> Self {
        self.user_id = val.into();
        self
    }

    /// A JSON-serialized array describing the errors
    ///
    /// # Notes
    /// Adds multiple elements.
    #[must_use]
    pub fn errors<
        TItem: Into<crate::types::PassportElementError>,
        T: IntoIterator<Item = TItem>,
    >(
        mut self,
        val: T,
    ) -> Self {
        self.errors = self
            .errors
            .into_vec()
            .into_iter()
            .chain(val.into_iter().map(Into::into))
            .collect();
        self
    }

    /// A JSON-serialized array describing the errors
    ///
    /// # Notes
    /// Adds a single element.
    #[must_use]
    pub fn error<T: Into<crate::types::PassportElementError>>(mut self, val: T) -> Self {
        self.errors = self
            .errors
            .into_vec()
            .into_iter()
            .chain(Some(val.into()))
            .collect();
        self
    }
}
impl super::TelegramMethod for SetPassportDataErrors {
    type Method = Self;
    type Return = bool;

    fn build_request<Client>(self, _bot: &Bot<Client>) -> super::Request<Self::Method> {
        super::Request::new("setPassportDataErrors", self, None)
    }
}
