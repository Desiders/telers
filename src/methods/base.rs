use crate::{client::Bot, types::ResponseParameters};

use serde::{de::DeserializeOwned, Deserialize, Serialize};
use serde_json;

/// This object represents a request to Telegram API
pub struct Request<'a, T>
where
    T: Serialize,
{
    /// Telegram API method name
    method: &'static str,
    /// Telegram API method parameters
    params: &'a T,
    // TODO: Add a files field
    // Files to send
    // files: Option<Vec<InputFile>>,
}

impl<'a, T> Request<'a, T>
where
    T: Serialize,
{
    #[must_use]
    pub fn new(method: &'static str, params: &'a T) -> Self {
        Self { method, params }
    }

    #[must_use]
    pub fn method(&self) -> &str {
        self.method
    }

    #[must_use]
    pub fn params(&self) -> &T {
        self.params
    }
}

/// This object represents a response from Telegram API. It's returned by making requests to Telegram API, for more info check [Telegram API docs](https://core.telegram.org/bots/api#making-requests)
/// # Note
/// The response contains a JSON object, which always has a Boolean field `ok` and may have an optional String field `description` with a human-readable description of the result. \
/// If `ok` equals `True`, the request was successful and the result of the query can be found in the `result` field. \
/// In case of an unsuccessful request, `ok` equals false and the error is explained in the `description`. \
/// An Integer `error_code` field is also returned, but its contents are subject to change in the future. \
/// Some errors may also have an optional field `parameters` of the type [`ResponseParameters`](https://core.telegram.org/bots/api#responseparameters), which can help to automatically handle the error.
#[derive(Deserialize)]
pub struct Response<T> {
    ok: bool,
    pub(crate) result: Option<T>,
    description: Option<String>,
    error_code: Option<i16>,
    parameters: Option<ResponseParameters>,
}

impl<T: DeserializeOwned> Response<T> {
    #[must_use]
    pub fn new(
        ok: bool,
        result: Option<T>,
        description: Option<String>,
        error_code: Option<i16>,
        parameters: Option<ResponseParameters>,
    ) -> Self {
        Self {
            ok,
            result,
            description,
            error_code,
            parameters,
        }
    }

    #[must_use]
    pub fn ok(&self) -> bool {
        self.ok
    }

    #[must_use]
    pub fn result(&self) -> Option<&T> {
        self.result.as_ref()
    }

    #[must_use]
    pub fn description(&self) -> Option<&str> {
        self.description.as_deref()
    }

    #[must_use]
    pub fn error_code(&self) -> Option<i16> {
        self.error_code
    }

    #[must_use]
    pub fn parameters(&self) -> Option<&ResponseParameters> {
        self.parameters.as_ref()
    }
}

pub trait TelegramMethod {
    /// This type represents a method to Telegram API with parameters
    type Method: Serialize;
    /// This type represents a response from Telegram API, which is returned by the method
    type Return: DeserializeOwned;

    /// This method is called when a request is sent to Telegram API.
    /// It's need for preparing a request to Telegram API.
    #[must_use]
    fn build_request(&self, bot: &Bot) -> Request<Self::Method>;

    /// This method is called when a response is received from Telegram API.
    /// It's need for parsing a response from Telegram API.
    /// # Errors
    /// - If the response cannot be parsed
    fn build_response(&self, content: &str) -> Result<Response<Self::Return>, serde_json::Error> {
        serde_json::from_str::<Response<Self::Return>>(content)
    }
}
