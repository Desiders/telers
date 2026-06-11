use serde::{Deserialize, Serialize};
/// Describes the current status of a webhook.
/// # Documentation
/// <https://core.telegram.org/bots/api#webhookinfo>
#[derive(Clone, Debug, Serialize, Deserialize)]
pub struct WebhookInfo {
    /// Webhook URL, may be empty if webhook is not set up
    pub url: Box<str>,
    /// `true`, if a custom certificate was provided for webhook certificate checks
    pub has_custom_certificate: bool,
    /// Number of updates awaiting delivery
    pub pending_update_count: i64,
    /// Currently used webhook IP address
    #[serde(skip_serializing_if = "Option::is_none")]
    pub ip_address: Option<Box<str>>,
    /// Unix time for the most recent error that happened when trying to deliver an update via webhook
    #[serde(skip_serializing_if = "Option::is_none")]
    pub last_error_date: Option<i64>,
    /// Error message in human-readable format for the most recent error that happened when trying to deliver an update via webhook
    #[serde(skip_serializing_if = "Option::is_none")]
    pub last_error_message: Option<Box<str>>,
    /// Unix time of the most recent error that happened when trying to synchronize available updates with Telegram datacenters
    #[serde(skip_serializing_if = "Option::is_none")]
    pub last_synchronization_error_date: Option<i64>,
    /// The maximum allowed number of simultaneous HTTPS connections to the webhook for update delivery
    #[serde(skip_serializing_if = "Option::is_none")]
    pub max_connections: Option<i64>,
    /// A list of update types the bot is subscribed to. Defaults to all update types except `chat_member`, `message_reaction`, and `message_reaction_count`.
    #[serde(skip_serializing_if = "Option::is_none")]
    pub allowed_updates: Option<Box<[Box<str>]>>,
}
impl WebhookInfo {
    /// Creates a new `WebhookInfo`.
    ///
    /// # Arguments
    /// * `url` - Webhook URL, may be empty if webhook is not set up
    /// * `has_custom_certificate` - `true`, if a custom certificate was provided for webhook certificate checks
    /// * `pending_update_count` - Number of updates awaiting delivery
    ///
    /// # Notes
    /// Use builder methods to set optional fields.
    #[must_use]
    pub fn new<T0: Into<Box<str>>, T1: Into<bool>, T2: Into<i64>>(
        url: T0,
        has_custom_certificate: T1,
        pending_update_count: T2,
    ) -> Self {
        Self {
            url: url.into(),
            has_custom_certificate: has_custom_certificate.into(),
            pending_update_count: pending_update_count.into(),
            ip_address: None,
            last_error_date: None,
            last_error_message: None,
            last_synchronization_error_date: None,
            max_connections: None,
            allowed_updates: None,
        }
    }

    /// Webhook URL, may be empty if webhook is not set up
    #[must_use]
    pub fn url<T: Into<Box<str>>>(mut self, val: T) -> Self {
        self.url = val.into();
        self
    }

    /// `true`, if a custom certificate was provided for webhook certificate checks
    #[must_use]
    pub fn has_custom_certificate<T: Into<bool>>(mut self, val: T) -> Self {
        self.has_custom_certificate = val.into();
        self
    }

    /// Number of updates awaiting delivery
    #[must_use]
    pub fn pending_update_count<T: Into<i64>>(mut self, val: T) -> Self {
        self.pending_update_count = val.into();
        self
    }

    /// Currently used webhook IP address
    #[must_use]
    pub fn ip_address<T: Into<Box<str>>>(mut self, val: T) -> Self {
        self.ip_address = Some(val.into());
        self
    }

    /// Currently used webhook IP address
    #[must_use]
    pub fn ip_address_option<T: Into<Box<str>>>(mut self, val: Option<T>) -> Self {
        self.ip_address = val.map(Into::into);
        self
    }

    /// Unix time for the most recent error that happened when trying to deliver an update via webhook
    #[must_use]
    pub fn last_error_date<T: Into<i64>>(mut self, val: T) -> Self {
        self.last_error_date = Some(val.into());
        self
    }

    /// Unix time for the most recent error that happened when trying to deliver an update via webhook
    #[must_use]
    pub fn last_error_date_option<T: Into<i64>>(mut self, val: Option<T>) -> Self {
        self.last_error_date = val.map(Into::into);
        self
    }

    /// Error message in human-readable format for the most recent error that happened when trying to deliver an update via webhook
    #[must_use]
    pub fn last_error_message<T: Into<Box<str>>>(mut self, val: T) -> Self {
        self.last_error_message = Some(val.into());
        self
    }

    /// Error message in human-readable format for the most recent error that happened when trying to deliver an update via webhook
    #[must_use]
    pub fn last_error_message_option<T: Into<Box<str>>>(mut self, val: Option<T>) -> Self {
        self.last_error_message = val.map(Into::into);
        self
    }

    /// Unix time of the most recent error that happened when trying to synchronize available updates with Telegram datacenters
    #[must_use]
    pub fn last_synchronization_error_date<T: Into<i64>>(mut self, val: T) -> Self {
        self.last_synchronization_error_date = Some(val.into());
        self
    }

    /// Unix time of the most recent error that happened when trying to synchronize available updates with Telegram datacenters
    #[must_use]
    pub fn last_synchronization_error_date_option<T: Into<i64>>(mut self, val: Option<T>) -> Self {
        self.last_synchronization_error_date = val.map(Into::into);
        self
    }

    /// The maximum allowed number of simultaneous HTTPS connections to the webhook for update delivery
    #[must_use]
    pub fn max_connections<T: Into<i64>>(mut self, val: T) -> Self {
        self.max_connections = Some(val.into());
        self
    }

    /// The maximum allowed number of simultaneous HTTPS connections to the webhook for update delivery
    #[must_use]
    pub fn max_connections_option<T: Into<i64>>(mut self, val: Option<T>) -> Self {
        self.max_connections = val.map(Into::into);
        self
    }

    /// A list of update types the bot is subscribed to. Defaults to all update types except `chat_member`, `message_reaction`, and `message_reaction_count`.
    ///
    /// # Notes
    /// Adds multiple elements.
    #[must_use]
    pub fn allowed_updates<T: Into<Box<[Box<str>]>>>(mut self, val: T) -> Self {
        self.allowed_updates = Some(
            self.allowed_updates
                .unwrap_or_default()
                .into_vec()
                .into_iter()
                .chain(val.into())
                .collect(),
        );
        self
    }

    /// A list of update types the bot is subscribed to. Defaults to all update types except `chat_member`, `message_reaction`, and `message_reaction_count`.
    ///
    /// # Notes
    /// Adds a single element.
    #[must_use]
    pub fn allowed_update<T: Into<Box<str>>>(mut self, val: T) -> Self {
        self.allowed_updates = Some(
            self.allowed_updates
                .unwrap_or_default()
                .into_vec()
                .into_iter()
                .chain(Some(val.into()))
                .collect(),
        );
        self
    }

    /// A list of update types the bot is subscribed to. Defaults to all update types except `chat_member`, `message_reaction`, and `message_reaction_count`.
    ///
    /// # Notes
    /// Adds a single element.
    #[must_use]
    pub fn allowed_updates_option<T: Into<Box<[Box<str>]>>>(mut self, val: Option<T>) -> Self {
        self.allowed_updates = val.map(Into::into);
        self
    }
}
