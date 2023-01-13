use super::{Request, TelegramMethod};

use crate::{client::Bot, types::Update};

use serde::{Deserialize, Serialize};
use serde_with::skip_serializing_none;

#[skip_serializing_none]
#[derive(Clone, Debug, Eq, Hash, PartialEq, Serialize, Deserialize, Default)]
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

impl TelegramMethod for GetUpdates {
    type Method = GetUpdates;
    type Return = Vec<Update>;

    fn build_request(&self, _: &Bot) -> Request<Self::Method> {
        Request::new("getUpdates", self, None)
    }
}
