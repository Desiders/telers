use crate::client::Bot;
use serde::Serialize;
/// Use this method to receive incoming updates using long polling (wiki). Returns an Array of Update objects.
/// # Documentation
/// <https://core.telegram.org/bots/api#getupdates>
/// # Returns
/// - `Box<[crate::Either<crate::types::Update,crate::types::UpdateUnparsed>]>`
#[derive(Clone, Debug, Serialize)]
pub struct GetUpdates {
    /// Identifier of the first update to be returned. Must be greater by one than the highest among the identifiers of previously received updates. By default, updates starting with the earliest unconfirmed update are returned. An update is considered confirmed as soon as [`crate::methods::GetUpdates`] is called with an offset higher than its `update_id`. The negative offset can be specified to retrieve updates starting from -offset update from the end of the updates queue. All previous updates will be forgotten.
    #[serde(skip_serializing_if = "Option::is_none")]
    pub offset: Option<i64>,
    /// Limits the number of updates to be retrieved. Values between 1-100 are accepted. Defaults to 100.
    #[serde(skip_serializing_if = "Option::is_none")]
    pub limit: Option<u8>,
    /// Timeout in seconds for long polling. Defaults to 0, i.e. usual short polling. Should be positive, short polling should be used for testing purposes only.
    #[serde(skip_serializing_if = "Option::is_none")]
    pub timeout: Option<i64>,
    /// A JSON-serialized list of the update types you want your bot to receive. For example, specify `message`, `edited_channel_post`, `callback_query` to only receive updates of these types. See Update for a complete list of available update types. Specify an empty list to receive all update types except `chat_member`, `message_reaction`, and `message_reaction_count` (default). If not specified, the previous setting will be used. Please note that this parameter doesn't affect updates created before the call to [`crate::methods::GetUpdates`], so unwanted updates may be received for a short period of time.
    #[serde(skip_serializing_if = "Option::is_none")]
    pub allowed_updates: Option<Box<[Box<str>]>>,
}
impl GetUpdates {
    /// Creates a new `GetUpdates`.
    ///
    /// # Notes
    /// Use builder methods to set optional fields.
    #[must_use]
    pub fn new() -> Self {
        Self {
            offset: None,
            limit: None,
            timeout: None,
            allowed_updates: None,
        }
    }

    /// Identifier of the first update to be returned. Must be greater by one than the highest among the identifiers of previously received updates. By default, updates starting with the earliest unconfirmed update are returned. An update is considered confirmed as soon as [`crate::methods::GetUpdates`] is called with an offset higher than its `update_id`. The negative offset can be specified to retrieve updates starting from -offset update from the end of the updates queue. All previous updates will be forgotten.
    #[must_use]
    pub fn offset<T: Into<i64>>(self, val: T) -> Self {
        let mut this = self;
        this.offset = Some(val.into());
        this
    }

    /// Identifier of the first update to be returned. Must be greater by one than the highest among the identifiers of previously received updates. By default, updates starting with the earliest unconfirmed update are returned. An update is considered confirmed as soon as [`crate::methods::GetUpdates`] is called with an offset higher than its `update_id`. The negative offset can be specified to retrieve updates starting from -offset update from the end of the updates queue. All previous updates will be forgotten.
    #[must_use]
    pub fn offset_option<T: Into<i64>>(self, val: Option<T>) -> Self {
        let mut this = self;
        this.offset = val.map(Into::into);
        this
    }

    /// Limits the number of updates to be retrieved. Values between 1-100 are accepted. Defaults to 100.
    #[must_use]
    pub fn limit<T: Into<u8>>(self, val: T) -> Self {
        let mut this = self;
        this.limit = Some(val.into());
        this
    }

    /// Limits the number of updates to be retrieved. Values between 1-100 are accepted. Defaults to 100.
    #[must_use]
    pub fn limit_option<T: Into<u8>>(self, val: Option<T>) -> Self {
        let mut this = self;
        this.limit = val.map(Into::into);
        this
    }

    /// Timeout in seconds for long polling. Defaults to 0, i.e. usual short polling. Should be positive, short polling should be used for testing purposes only.
    #[must_use]
    pub fn timeout<T: Into<i64>>(self, val: T) -> Self {
        let mut this = self;
        this.timeout = Some(val.into());
        this
    }

    /// Timeout in seconds for long polling. Defaults to 0, i.e. usual short polling. Should be positive, short polling should be used for testing purposes only.
    #[must_use]
    pub fn timeout_option<T: Into<i64>>(self, val: Option<T>) -> Self {
        let mut this = self;
        this.timeout = val.map(Into::into);
        this
    }

    /// A JSON-serialized list of the update types you want your bot to receive. For example, specify `message`, `edited_channel_post`, `callback_query` to only receive updates of these types. See Update for a complete list of available update types. Specify an empty list to receive all update types except `chat_member`, `message_reaction`, and `message_reaction_count` (default). If not specified, the previous setting will be used. Please note that this parameter doesn't affect updates created before the call to [`crate::methods::GetUpdates`], so unwanted updates may be received for a short period of time.
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

    /// A JSON-serialized list of the update types you want your bot to receive. For example, specify `message`, `edited_channel_post`, `callback_query` to only receive updates of these types. See Update for a complete list of available update types. Specify an empty list to receive all update types except `chat_member`, `message_reaction`, and `message_reaction_count` (default). If not specified, the previous setting will be used. Please note that this parameter doesn't affect updates created before the call to [`crate::methods::GetUpdates`], so unwanted updates may be received for a short period of time.
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

    /// A JSON-serialized list of the update types you want your bot to receive. For example, specify `message`, `edited_channel_post`, `callback_query` to only receive updates of these types. See Update for a complete list of available update types. Specify an empty list to receive all update types except `chat_member`, `message_reaction`, and `message_reaction_count` (default). If not specified, the previous setting will be used. Please note that this parameter doesn't affect updates created before the call to [`crate::methods::GetUpdates`], so unwanted updates may be received for a short period of time.
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
}
impl Default for GetUpdates {
    fn default() -> Self {
        Self::new()
    }
}
impl super::TelegramMethod for GetUpdates {
    type Method = Self;
    type Return = Box<[crate::Either<crate::types::Update, crate::types::UpdateUnparsed>]>;

    fn build_request<Client>(self, _bot: &Bot<Client>) -> super::Request<Self::Method> {
        super::Request::new("getUpdates", self, None)
    }
}
