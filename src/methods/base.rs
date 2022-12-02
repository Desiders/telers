use crate::types::ResponseParameters;

use serde::{Deserialize, Serialize};

/// This object represents a request to Telegram API
pub struct Request<T>
where
    T: Serialize,
{
    /// Telegram API method name
    method: &'static str,
    /// Telegram API method parameters
    params: T,
    // TODO: Add a files field
    // Files to send
    // files: Option<Vec<InputFile>>,
}

impl<T> Request<T>
where
    T: Serialize,
{
    #[must_use]
    pub fn new(method: &'static str, params: T) -> Self {
        Self { method, params }
    }
}

/// This object represents a response from Telegram API
#[derive(Deserialize)]
pub struct Response<T> {
    ok: bool,
    result: Option<T>,
    description: Option<String>,
    error_code: Option<i16>,
    parameters: Option<ResponseParameters>,
}

impl<T> Response<T>
where
    T: for<'de> Deserialize<'de>,
{
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
}
