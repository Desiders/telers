use serde::{Deserialize, Serialize};

/// Describes a [`Web App`](https://core.telegram.org/bots/webapps).
/// # Documentation
/// <https://core.telegram.org/bots/api#webappinfo>
#[derive(Debug, Default, Clone, Hash, PartialEq, Eq, Deserialize, Serialize)]
pub struct WebAppInfo {
    /// An HTTPS URL of a Web App to be opened with additional data as specified in [`Initializing Web Apps`](https://core.telegram.org/bots/webapps#initializing-mini-apps)
    pub url: Box<str>,
}

impl WebAppInfo {
    #[must_use]
    pub fn new(url: impl Into<Box<str>>) -> Self {
        Self { url: url.into() }
    }

    #[must_use]
    pub fn url(self, val: impl Into<Box<str>>) -> Self {
        Self { url: val.into() }
    }
}
