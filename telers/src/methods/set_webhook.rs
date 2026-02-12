use super::base::{prepare_file, Request, TelegramMethod};

use crate::{client::Bot, enums::UpdateType, types::InputFile};

use serde::Serialize;
use serde_with::skip_serializing_none;

/// Use this method to specify a URL and receive incoming updates via an outgoing webhook. Whenever there is an update for the bot, we will send an HTTPS POST request to the specified URL, containing a JSON-serialized [`crate::types::Update`]. In case of an unsuccessful request (a request with response [HTTP status code](https://en.wikipedia.org/wiki/List_of_HTTP_status_codes) different from `2XY`), we will repeat the request and give up after a reasonable amount of attempts.
/// # Documentation
/// <https://core.telegram.org/bots/api#setwebhook>
/// # Returns
/// On success, `true` is returned
#[skip_serializing_none]
#[derive(Debug, Hash, PartialEq, Serialize)]
pub struct SetWebhook {
    /// HTTPS URL to send updates to. Use an empty string to remove webhook integration
    pub url: String,
    /// Upload your public key certificate so that the root certificate in use can be checked. See our [self-signed guide](https://core.telegram.org/bots/self-signed) for details.
    pub certificate: Option<InputFile>,
    /// The fixed IP address which will be used to send webhook requests instead of the IP address resolved through DNS
    pub ip_address: Option<String>,
    /// The maximum allowed number of simultaneous HTTPS connections to the webhook for update delivery, 1-100. Defaults to 40. Use lower values to limit the load on your bot's server, and higher values to increase your bot's throughput.
    pub max_connections: Option<u8>,
    /// A JSON-serialized list of the update types you want your bot to receive. For example, specify [`message`, `edited_channel_post`, `callback_query`] to only receive updates of these types. See [`crate::types::Update`] for a complete list of available update types. Specify an empty list to receive all update types except `chat_member`, `message_reaction`, and `message_reaction_count` (default). If not specified, the previous setting will be used.
    /// Please note that this parameter doesn't affect updates created before the call to the [`SetWebhook`], so unwanted updates may be received for a short period of time.
    pub allowed_updates: Option<Vec<String>>,
    /// Pass `true` to drop all pending updates
    pub drop_pending_updates: Option<bool>,
    /// A secret token to be sent in a header “X-Telegram-Bot-Api-Secret-Token” in every webhook request, 1-256 characters. Only characters `A-Z`, `a-z`, `0-9`, `_` and `-` are allowed. The header is useful to ensure that the request comes from a webhook set by you.
    pub secret_token: Option<String>,
}

impl SetWebhook {
    #[must_use]
    pub fn new(url: impl Into<String>) -> Self {
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

    #[must_use]
    pub fn url(self, val: impl Into<String>) -> Self {
        Self {
            url: val.into(),
            ..self
        }
    }

    #[must_use]
    pub fn certificate(self, val: impl Into<InputFile>) -> Self {
        Self {
            certificate: Some(val.into()),
            ..self
        }
    }

    #[must_use]
    pub fn ip_address(self, val: impl Into<String>) -> Self {
        Self {
            ip_address: Some(val.into()),
            ..self
        }
    }

    #[must_use]
    pub fn max_connections(self, val: u8) -> Self {
        Self {
            max_connections: Some(val),
            ..self
        }
    }

    #[must_use]
    pub fn allowed_update(self, val: UpdateType) -> Self {
        Self {
            allowed_updates: Some(
                self.allowed_updates
                    .unwrap_or_default()
                    .into_iter()
                    .chain(Some(val.to_string()))
                    .collect(),
            ),
            ..self
        }
    }

    #[must_use]
    pub fn allowed_updates(self, val: impl IntoIterator<Item = UpdateType>) -> Self {
        Self {
            allowed_updates: Some(
                self.allowed_updates
                    .unwrap_or_default()
                    .into_iter()
                    .chain(val.into_iter().map(|val| val.to_string()))
                    .collect(),
            ),
            ..self
        }
    }

    #[must_use]
    pub fn drop_pending_updates(self, val: bool) -> Self {
        Self {
            drop_pending_updates: Some(val),
            ..self
        }
    }

    #[must_use]
    pub fn secret_token(self, val: impl Into<String>) -> Self {
        Self {
            secret_token: Some(val.into()),
            ..self
        }
    }
}

impl SetWebhook {
    #[must_use]
    pub fn certificate_option(self, val: Option<impl Into<InputFile>>) -> Self {
        Self {
            certificate: val.map(Into::into),
            ..self
        }
    }

    #[must_use]
    pub fn ip_address_option(self, val: Option<impl Into<String>>) -> Self {
        Self {
            ip_address: val.map(Into::into),
            ..self
        }
    }

    #[must_use]
    pub fn max_connections_option(self, val: Option<u8>) -> Self {
        Self {
            max_connections: val,
            ..self
        }
    }

    #[must_use]
    pub fn allowed_update_option(self, val: Option<impl Into<String>>) -> Self {
        Self {
            allowed_updates: val.map(|val| {
                self.allowed_updates
                    .unwrap_or_default()
                    .into_iter()
                    .chain(Some(val.into()))
                    .collect()
            }),
            ..self
        }
    }

    #[must_use]
    pub fn allowed_updates_option<T, I>(self, val: Option<I>) -> Self
    where
        T: Into<String>,
        I: IntoIterator<Item = T>,
    {
        Self {
            allowed_updates: val.map(|val| {
                self.allowed_updates
                    .unwrap_or_default()
                    .into_iter()
                    .chain(val.into_iter().map(Into::into))
                    .collect()
            }),
            ..self
        }
    }

    #[must_use]
    pub fn drop_pending_updates_option(self, val: Option<bool>) -> Self {
        Self {
            drop_pending_updates: val,
            ..self
        }
    }

    #[must_use]
    pub fn secret_token_option(self, val: Option<impl Into<String>>) -> Self {
        Self {
            secret_token: val.map(Into::into),
            ..self
        }
    }
}

impl TelegramMethod for SetWebhook {
    type Method = Self;
    type Return = bool;

    fn build_request<Client>(mut self, _bot: &Bot<Client>) -> Request<Self::Method> {
        let mut files = vec![];
        if let Some(file) = &mut self.certificate {
            prepare_file(&mut files, file);
        }

        Request::new("setWebhook", self, Some(files))
    }
}

impl AsRef<SetWebhook> for SetWebhook {
    fn as_ref(&self) -> &Self {
        self
    }
}
