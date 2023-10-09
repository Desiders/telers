use serde::{Deserialize, Serialize};

/// Describes a [`Web App`](https://core.telegram.org/bots/webapps).
/// # Documentation
/// <https://core.telegram.org/bots/api#webappinfo>
#[derive(Default, Clone, Debug, Eq, Hash, PartialEq, Serialize, Deserialize)]
pub struct WebAppInfo {
    /// An HTTPS URL of a Web App to be opened with additional data as specified in [`Initializing Web Apps`](https://core.telegram.org/bots/webapps#initializing-mini-apps)
    pub url: String,
}

impl WebAppInfo {
    #[must_use]
    pub fn new(url: impl Into<String>) -> Self {
        Self { url: url.into() }
    }

    #[must_use]
    pub fn url(self, val: impl Into<String>) -> Self {
        Self { url: val.into() }
    }
}
