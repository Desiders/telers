use super::base::{Request, TelegramMethod};

use crate::{client::Bot, types::PassportElementError};

use serde::Serialize;
use serde_with::skip_serializing_none;

/// Informs a user that some of the Telegram Passport elements they provided contains errors. The user will not be able to re-submit their Passport to you until the errors are fixed (the contents of the field for which you returned the error must change).
/// # Documentation
/// <https://core.telegram.org/bots/api#setpassportdataerrors>
/// # Note
/// Use this if the data submitted by the user doesn't satisfy the standards your service requires for any reason. For example, if a birthday date seems invalid, a submitted document is blurry, a scan shows evidence of tampering, etc. Supply some details in the error message to make sure the user knows how to correct the issues.
/// # Returns
/// On success, `True` is returned
#[skip_serializing_none]
#[derive(Clone, Debug, Eq, Hash, PartialEq, Serialize)]
pub struct SetPassportDataErrors {
    /// User identifier
    pub user_id: i64,
    /// A JSON-serialized array describing the errors
    pub errors: Vec<PassportElementError>,
}

impl SetPassportDataErrors {
    #[must_use]
    pub fn new<T: Into<PassportElementError>>(user_id: i64, errors: Vec<T>) -> Self {
        Self {
            user_id,
            errors: errors.into_iter().map(Into::into).collect(),
        }
    }

    #[must_use]
    pub fn user_id(mut self, val: i64) -> Self {
        self.user_id = val;
        self
    }

    #[must_use]
    pub fn errors<T: Into<PassportElementError>>(mut self, val: Vec<T>) -> Self {
        self.errors.extend(val.into_iter().map(Into::into));
        self
    }

    #[must_use]
    pub fn error<T: Into<PassportElementError>>(mut self, val: T) -> Self {
        self.errors.push(val.into());
        self
    }
}

impl TelegramMethod for SetPassportDataErrors {
    type Method = Self;
    type Return = bool;

    fn build_request<Client>(&self, _bot: &Bot<Client>) -> Request<Self::Method> {
        Request::new("setPassportDataErrors", self, None)
    }
}