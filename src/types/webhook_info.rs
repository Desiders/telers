use serde::Deserialize;

/// Describes the current status of a webhook.
/// # Documentation
/// <https://core.telegram.org/bots/api#webhookinfo>
#[derive(Debug, Clone, Hash, PartialEq, Eq, Deserialize)]
pub struct WebhookInfo {
    /// Webhook URL, may be empty if webhook is not set up
    pub url: Box<str>,
    /// `true`, if a custom certificate was provided for webhook certificate checks
    pub has_custom_certificate: bool,
    /// Number of updates awaiting delivery
    pub pending_update_count: i64,
    /// Currently used webhook IP address
    pub ip_address: Option<Box<str>>,
    /// Unix time for the most recent error that happened when trying to deliver an update via webhook
    pub last_error_date: Option<i64>,
    /// Error message in human-readable format for the most recent error that happened when trying to deliver an update via webhook
    pub last_error_message: Option<Box<str>>,
    /// Unix time of the most recent error that happened when trying to synchronize available updates with Telegram datacenters
    pub last_synchronization_error_date: Option<i64>,
    /// The maximum allowed number of simultaneous HTTPS connections to the webhook for update delivery
    pub max_connections: Option<i64>,
    /// A list of update types the bot is subscribed to. Defaults to all update types except `chat_member`
    pub allowed_updates: Option<Box<[Box<str>]>>,
}
