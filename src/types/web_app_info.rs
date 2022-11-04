use serde::{Deserialize, Serialize};

/// Describes a `Web App <https://core.telegram.org/bots/webapps>`.
/// <https://core.telegram.org/bots/api#webappinfo>
#[derive(Default, Clone, Debug, Eq, Hash, PartialEq, Serialize, Deserialize)]
pub struct WebAppInfo {
    /// An HTTPS URL of a Web App to be opened with additional data as specified in `Initializing Web Apps <https://core.telegram.org/bots/webapps#initializing-web-apps>`
    pub url: String,
}
