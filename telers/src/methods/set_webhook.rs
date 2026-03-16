use crate::client::Bot;
use serde::Serialize;
/// Use this method to specify a URL and receive incoming updates via an outgoing webhook. Whenever there is an update for the bot, we will send an HTTPS POST request to the specified URL, containing a JSON-serialized Update. In case of an unsuccessful request (a request with response HTTP status code different from 2XY), we will repeat the request and give up after a reasonable amount of attempts. Returns `true` on success.
/// If you'd like to make sure that the webhook was set by you, you can specify secret data in the parameter `secret_token`. If specified, the request will contain a header `X-Telegram-Bot-Api-Secret-Token` with the secret token as content.
/// # Documentation
/// <https://core.telegram.org/bots/api#setwebhook>
/// # Returns
/// - `bool`
#[derive(Clone, Debug, Serialize)]
pub struct SetWebhook {
    /// HTTPS URL to send updates to. Use an empty string to remove webhook integration
    pub url: Box<str>,
    /// Upload your public key certificate so that the root certificate in use can be checked. See our self-signed guide for details.
    #[serde(skip_serializing_if = "Option::is_none")]
    pub certificate: Option<crate::types::InputFile>,
    /// The fixed IP address which will be used to send webhook requests instead of the IP address resolved through DNS
    #[serde(skip_serializing_if = "Option::is_none")]
    pub ip_address: Option<Box<str>>,
    /// The maximum allowed number of simultaneous HTTPS connections to the webhook for update delivery, 1-100. Defaults to 40. Use lower values to limit the load on your bot's server, and higher values to increase your bot's throughput.
    #[serde(skip_serializing_if = "Option::is_none")]
    pub max_connections: Option<u8>,
    /// A JSON-serialized list of the update types you want your bot to receive. For example, specify `message`, `edited_channel_post`, `callback_query` to only receive updates of these types. See Update for a complete list of available update types. Specify an empty list to receive all update types except `chat_member`, `message_reaction`, and `message_reaction_count` (default). If not specified, the previous setting will be used. Please note that this parameter doesn't affect updates created before the call to the [`crate::methods::SetWebhook`], so unwanted updates may be received for a short period of time.
    #[serde(skip_serializing_if = "Option::is_none")]
    pub allowed_updates: Option<Box<[Box<str>]>>,
    /// Pass `true` to drop all pending updates
    #[serde(skip_serializing_if = "Option::is_none")]
    pub drop_pending_updates: Option<bool>,
    /// A secret token to be sent in a header `X-Telegram-Bot-Api-Secret-Token` in every webhook request, 1-256 characters. Only characters A-Z, a-z, 0-9, `_` and - are allowed. The header is useful to ensure that the request comes from a webhook set by you.
    #[serde(skip_serializing_if = "Option::is_none")]
    pub secret_token: Option<Box<str>>,
}
impl SetWebhook {
    /// Creates a new `SetWebhook`.
    ///
    /// # Arguments
    /// * `url` - HTTPS URL to send updates to. Use an empty string to remove webhook integration
    ///
    /// # Notes
    /// Use builder methods to set optional fields.
    #[must_use]
    pub fn new<T0: Into<Box<str>>>(url: T0) -> Self {
        Self {
            url: url.into(),
            certificate: None,
            ip_address: None,
            max_connections: None,
            allowed_updates: None,
            drop_pending_updates: None,
            secret_token: None,
        }
    }

    /// HTTPS URL to send updates to. Use an empty string to remove webhook integration
    #[must_use]
    pub fn url<T: Into<Box<str>>>(self, val: T) -> Self {
        let mut this = self;
        this.url = val.into();
        this
    }

    /// Upload your public key certificate so that the root certificate in use can be checked. See our self-signed guide for details.
    #[must_use]
    pub fn certificate<T: Into<crate::types::InputFile>>(self, val: T) -> Self {
        let mut this = self;
        this.certificate = Some(val.into());
        this
    }

    /// Upload your public key certificate so that the root certificate in use can be checked. See our self-signed guide for details.
    #[must_use]
    pub fn certificate_option<T: Into<crate::types::InputFile>>(self, val: Option<T>) -> Self {
        let mut this = self;
        this.certificate = val.map(Into::into);
        this
    }

    /// The fixed IP address which will be used to send webhook requests instead of the IP address resolved through DNS
    #[must_use]
    pub fn ip_address<T: Into<Box<str>>>(self, val: T) -> Self {
        let mut this = self;
        this.ip_address = Some(val.into());
        this
    }

    /// The fixed IP address which will be used to send webhook requests instead of the IP address resolved through DNS
    #[must_use]
    pub fn ip_address_option<T: Into<Box<str>>>(self, val: Option<T>) -> Self {
        let mut this = self;
        this.ip_address = val.map(Into::into);
        this
    }

    /// The maximum allowed number of simultaneous HTTPS connections to the webhook for update delivery, 1-100. Defaults to 40. Use lower values to limit the load on your bot's server, and higher values to increase your bot's throughput.
    #[must_use]
    pub fn max_connections<T: Into<u8>>(self, val: T) -> Self {
        let mut this = self;
        this.max_connections = Some(val.into());
        this
    }

    /// The maximum allowed number of simultaneous HTTPS connections to the webhook for update delivery, 1-100. Defaults to 40. Use lower values to limit the load on your bot's server, and higher values to increase your bot's throughput.
    #[must_use]
    pub fn max_connections_option<T: Into<u8>>(self, val: Option<T>) -> Self {
        let mut this = self;
        this.max_connections = val.map(Into::into);
        this
    }

    /// A JSON-serialized list of the update types you want your bot to receive. For example, specify `message`, `edited_channel_post`, `callback_query` to only receive updates of these types. See Update for a complete list of available update types. Specify an empty list to receive all update types except `chat_member`, `message_reaction`, and `message_reaction_count` (default). If not specified, the previous setting will be used. Please note that this parameter doesn't affect updates created before the call to the [`crate::methods::SetWebhook`], so unwanted updates may be received for a short period of time.
    ///
    /// # Notes
    /// Adds multiple elements.
    #[must_use]
    pub fn allowed_updates<TItem: Into<Box<str>>, T: IntoIterator<Item = TItem>>(
        self,
        val: T,
    ) -> Self {
        let mut this = self;
        this.allowed_updates = Some(
            this.allowed_updates
                .unwrap_or_default()
                .into_vec()
                .into_iter()
                .chain(val.into_iter().map(Into::into))
                .collect(),
        );
        this
    }

    /// A JSON-serialized list of the update types you want your bot to receive. For example, specify `message`, `edited_channel_post`, `callback_query` to only receive updates of these types. See Update for a complete list of available update types. Specify an empty list to receive all update types except `chat_member`, `message_reaction`, and `message_reaction_count` (default). If not specified, the previous setting will be used. Please note that this parameter doesn't affect updates created before the call to the [`crate::methods::SetWebhook`], so unwanted updates may be received for a short period of time.
    ///
    /// # Notes
    /// Adds a single element.
    #[must_use]
    pub fn allowed_update<T: Into<Box<str>>>(self, val: T) -> Self {
        let mut this = self;
        this.allowed_updates = Some(
            this.allowed_updates
                .unwrap_or_default()
                .into_vec()
                .into_iter()
                .chain(Some(val.into()))
                .collect(),
        );
        this
    }

    /// A JSON-serialized list of the update types you want your bot to receive. For example, specify `message`, `edited_channel_post`, `callback_query` to only receive updates of these types. See Update for a complete list of available update types. Specify an empty list to receive all update types except `chat_member`, `message_reaction`, and `message_reaction_count` (default). If not specified, the previous setting will be used. Please note that this parameter doesn't affect updates created before the call to the [`crate::methods::SetWebhook`], so unwanted updates may be received for a short period of time.
    ///
    /// # Notes
    /// Adds multiple elements.
    #[must_use]
    pub fn allowed_updates_option<TItem: Into<Box<str>>, T: IntoIterator<Item = TItem>>(
        self,
        val: Option<T>,
    ) -> Self {
        let mut this = self;
        this.allowed_updates = val.map(|v| v.into_iter().map(Into::into).collect());
        this
    }

    /// Pass `true` to drop all pending updates
    #[must_use]
    pub fn drop_pending_updates<T: Into<bool>>(self, val: T) -> Self {
        let mut this = self;
        this.drop_pending_updates = Some(val.into());
        this
    }

    /// Pass `true` to drop all pending updates
    #[must_use]
    pub fn drop_pending_updates_option<T: Into<bool>>(self, val: Option<T>) -> Self {
        let mut this = self;
        this.drop_pending_updates = val.map(Into::into);
        this
    }

    /// A secret token to be sent in a header `X-Telegram-Bot-Api-Secret-Token` in every webhook request, 1-256 characters. Only characters A-Z, a-z, 0-9, `_` and - are allowed. The header is useful to ensure that the request comes from a webhook set by you.
    #[must_use]
    pub fn secret_token<T: Into<Box<str>>>(self, val: T) -> Self {
        let mut this = self;
        this.secret_token = Some(val.into());
        this
    }

    /// A secret token to be sent in a header `X-Telegram-Bot-Api-Secret-Token` in every webhook request, 1-256 characters. Only characters A-Z, a-z, 0-9, `_` and - are allowed. The header is useful to ensure that the request comes from a webhook set by you.
    #[must_use]
    pub fn secret_token_option<T: Into<Box<str>>>(self, val: Option<T>) -> Self {
        let mut this = self;
        this.secret_token = val.map(Into::into);
        this
    }
}
impl super::TelegramMethod for SetWebhook {
    type Method = Self;
    type Return = bool;

    fn build_request<Client>(mut self, _bot: &Bot<Client>) -> super::Request<Self::Method> {
        let mut files = vec![];
        if let Some(file) = &mut self.certificate {
            super::prepare_file(&mut files, file);
        }
        super::Request::new("setWebhook", self, Some(files))
    }
}
