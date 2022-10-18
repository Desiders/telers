use serde::{Deserialize, Serialize};

/// Describes the current status of a webhook.
/// <https://core.telegram.org/bots/api#webhookinfo>_
#[derive(Clone, Debug, Eq, Hash, PartialEq, Serialize, Deserialize)]
pub struct WebhookInfo {
    /// Webhook URL, may be empty if webhook is not set up
    pub url: String,
    /// :code:`True`, if a custom certificate was provided for webhook certificate checks
    pub has_custom_certificate: bool,
    /// Number of updates awaiting delivery
    pub pending_update_count: i64,
    /// *Optional*. Currently used webhook IP address
    pub ip_address: Option<String>,
    /// *Optional*. Unix time for the most recent error that happened when trying to deliver an update via webhook
    pub last_error_date: Option<i64>,
    /// *Optional*. Error message in human-readable format for the most recent error that happened when trying to deliver an update via webhook
    pub last_error_message: Option<String>,
    /// *Optional*. Unix time of the most recent error that happened when trying to synchronize available updates with Telegram datacenters
    pub last_synchronization_error_date: Option<i64>,
    /// *Optional*. The maximum allowed number of simultaneous HTTPS connections to the webhook for update delivery
    pub max_connections: Option<i64>,
    /// *Optional*. A list of update types the bot is subscribed to. Defaults to all update types except `chat_member`
    pub allowed_updates: Option<Vec<String>>,
}

impl Default for WebhookInfo {
    fn default() -> Self {
        Self {
            url: String::default(),
            has_custom_certificate: false,
            pending_update_count: 0,
            ip_address: None,
            last_error_date: None,
            last_error_message: None,
            last_synchronization_error_date: None,
            max_connections: None,
            allowed_updates: None,
        }
    }
}
