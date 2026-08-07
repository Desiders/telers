use serde::{Deserialize, Serialize};
/// Describes a Web App.
/// # Documentation
/// <https://core.telegram.org/bots/api#webappinfo>
#[derive(Clone, Debug, Serialize, Deserialize)]
pub struct WebAppInfo {
    /// An HTTPS URL of a Web App to be opened with additional data as specified in Initializing Web Apps
    pub url: Box<str>,
}
impl WebAppInfo {
    /// Creates a new `WebAppInfo`.
    ///
    /// # Arguments
    /// * `url` - An HTTPS URL of a Web App to be opened with additional data as specified in Initializing Web Apps
    #[must_use]
    pub fn new<T0: Into<Box<str>>>(url: T0) -> Self {
        Self { url: url.into() }
    }

    /// An HTTPS URL of a Web App to be opened with additional data as specified in Initializing Web Apps
    #[must_use]
    pub fn url<T: Into<Box<str>>>(mut self, val: T) -> Self {
        self.url = val.into();
        self
    }
}
