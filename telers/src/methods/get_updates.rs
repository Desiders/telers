use super::{Request, TelegramMethod};

use crate::{client::Bot, types::Update};

use serde::Serialize;
use serde_with::skip_serializing_none;

/// Use this method to receive incoming updates using long polling ([`wiki`](https://en.wikipedia.org/wiki/Push_technology#Long_polling)).
/// # Documentation
/// <https://core.telegram.org/bots/api#getupdates>
/// # Notes
/// - This method will not work if an outgoing webhook is set up. \
/// - In order to avoid getting duplicate updates, recalculate `offset` after each server response. \
/// # Returns
/// Array of [`Update`] objects
#[skip_serializing_none]
#[derive(Debug, Default, Clone, Hash, PartialEq, Eq, Serialize)]
pub struct GetUpdates {
    /// Identifier of the first update to be returned. Must be greater by one than the highest among the identifiers of previously received updates. By default, updates starting with the earliest unconfirmed update are returned. An update is considered confirmed as soon as [`crate::methods::get_updates::GetUpdates`] is called with an *offset* higher than its *update_id*. The negative offset can be specified to retrieve updates starting from *-offset* update from the end of the updates queue. All previous updates will forgotten.
    pub offset: Option<i64>,
    /// Limits the number of updates to be retrieved. Values between 1-100 are accepted. Defaults to 100.
    pub limit: Option<i64>,
    /// Timeout in seconds for long polling. Defaults to 0, i.e. usual short polling. Should be positive, short polling should be used for testing purposes only.
    pub timeout: Option<i64>,
    /// A JSON-serialized list of the update types you want your bot to receive. For example, specify [`message`, `edited_channel_post`, `callback_query`] to only receive updates of these types. See [`crate::types::Update`] for a complete list of available update types. Specify an empty list to receive all update types except *chat_member* (default). If not specified, the previous setting will be used.
    pub allowed_updates: Option<Vec<String>>,
}

impl GetUpdates {
    #[must_use]
    pub fn new() -> Self {
        Self::default()
    }

    #[must_use]
    pub fn offset(self, val: i64) -> Self {
        Self {
            offset: Some(val),
            ..self
        }
    }

    #[must_use]
    pub fn limit(self, val: i64) -> Self {
        Self {
            limit: Some(val),
            ..self
        }
    }

    #[must_use]
    pub fn timeout(self, val: i64) -> Self {
        Self {
            timeout: Some(val),
            ..self
        }
    }

    #[must_use]
    pub fn allowed_update(self, val: impl Into<String>) -> Self {
        Self {
            allowed_updates: Some(
                self.allowed_updates
                    .unwrap_or_default()
                    .into_iter()
                    .chain(Some(val.into()))
                    .collect(),
            ),
            ..self
        }
    }

    #[must_use]
    pub fn allowed_updates<T, I>(self, val: I) -> Self
    where
        T: Into<String>,
        I: IntoIterator<Item = T>,
    {
        Self {
            allowed_updates: Some(
                self.allowed_updates
                    .unwrap_or_default()
                    .into_iter()
                    .chain(val.into_iter().map(Into::into))
                    .collect(),
            ),
            ..self
        }
    }
}

impl GetUpdates {
    #[must_use]
    pub fn offset_option(self, val: Option<i64>) -> Self {
        Self {
            offset: val,
            ..self
        }
    }

    #[must_use]
    pub fn limit_option(self, val: Option<i64>) -> Self {
        Self { limit: val, ..self }
    }

    #[must_use]
    pub fn timeout_option(self, val: Option<i64>) -> Self {
        Self {
            timeout: val,
            ..self
        }
    }
}

impl TelegramMethod for GetUpdates {
    type Method = Self;
    type Return = Vec<Update>;

    fn build_request<Client>(&self, _bot: &Bot<Client>) -> Request<Self::Method> {
        Request::new("getUpdates", self, None)
    }
}

impl AsRef<GetUpdates> for GetUpdates {
    fn as_ref(&self) -> &Self {
        self
    }
}
