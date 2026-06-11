use crate::client::Bot;
use serde::Serialize;
/// Returns the gifts owned by a chat. Returns [`crate::types::OwnedGifts`] on success.
/// # Documentation
/// <https://core.telegram.org/bots/api#getchatgifts>
/// # Returns
/// - `crate::types::OwnedGifts`
#[derive(Clone, Debug, Serialize)]
pub struct GetChatGifts {
    /// Unique identifier for the target chat or username of the target channel in the format @username
    pub chat_id: crate::types::ChatIdKind,
    /// Pass `true` to exclude gifts that aren't saved to the chat's profile page. Always `true`, unless the bot has the `can_post_messages` administrator right in the channel.
    #[serde(skip_serializing_if = "Option::is_none")]
    pub exclude_unsaved: Option<bool>,
    /// Pass `true` to exclude gifts that are saved to the chat's profile page. Always `false`, unless the bot has the `can_post_messages` administrator right in the channel.
    #[serde(skip_serializing_if = "Option::is_none")]
    pub exclude_saved: Option<bool>,
    /// Pass `true` to exclude gifts that can be purchased an unlimited number of times
    #[serde(skip_serializing_if = "Option::is_none")]
    pub exclude_unlimited: Option<bool>,
    /// Pass `true` to exclude gifts that can be purchased a limited number of times and can be upgraded to unique
    #[serde(skip_serializing_if = "Option::is_none")]
    pub exclude_limited_upgradable: Option<bool>,
    /// Pass `true` to exclude gifts that can be purchased a limited number of times and can't be upgraded to unique
    #[serde(skip_serializing_if = "Option::is_none")]
    pub exclude_limited_non_upgradable: Option<bool>,
    /// Pass `true` to exclude gifts that were assigned from the TON blockchain and can't be resold or transferred in Telegram
    #[serde(skip_serializing_if = "Option::is_none")]
    pub exclude_from_blockchain: Option<bool>,
    /// Pass `true` to exclude unique gifts
    #[serde(skip_serializing_if = "Option::is_none")]
    pub exclude_unique: Option<bool>,
    /// Pass `true` to sort results by gift price instead of send date. Sorting is applied before pagination.
    #[serde(skip_serializing_if = "Option::is_none")]
    pub sort_by_price: Option<bool>,
    /// Offset of the first entry to return as received from the previous request; use an empty string to get the first chunk of results
    #[serde(skip_serializing_if = "Option::is_none")]
    pub offset: Option<Box<str>>,
    /// The maximum number of gifts to be returned; 1-100. Defaults to 100.
    #[serde(skip_serializing_if = "Option::is_none")]
    pub limit: Option<u8>,
}
impl GetChatGifts {
    /// Creates a new `GetChatGifts`.
    ///
    /// # Arguments
    /// * `chat_id` - Unique identifier for the target chat or username of the target channel in the format @username
    ///
    /// # Notes
    /// Use builder methods to set optional fields.
    #[must_use]
    pub fn new<T0: Into<crate::types::ChatIdKind>>(chat_id: T0) -> Self {
        Self {
            chat_id: chat_id.into(),
            exclude_unsaved: None,
            exclude_saved: None,
            exclude_unlimited: None,
            exclude_limited_upgradable: None,
            exclude_limited_non_upgradable: None,
            exclude_from_blockchain: None,
            exclude_unique: None,
            sort_by_price: None,
            offset: None,
            limit: None,
        }
    }

    /// Unique identifier for the target chat or username of the target channel in the format @username
    #[must_use]
    pub fn chat_id<T: Into<crate::types::ChatIdKind>>(mut self, val: T) -> Self {
        self.chat_id = val.into();
        self
    }

    /// Pass `true` to exclude gifts that aren't saved to the chat's profile page. Always `true`, unless the bot has the `can_post_messages` administrator right in the channel.
    #[must_use]
    pub fn exclude_unsaved<T: Into<bool>>(mut self, val: T) -> Self {
        self.exclude_unsaved = Some(val.into());
        self
    }

    /// Pass `true` to exclude gifts that aren't saved to the chat's profile page. Always `true`, unless the bot has the `can_post_messages` administrator right in the channel.
    #[must_use]
    pub fn exclude_unsaved_option<T: Into<bool>>(mut self, val: Option<T>) -> Self {
        self.exclude_unsaved = val.map(Into::into);
        self
    }

    /// Pass `true` to exclude gifts that are saved to the chat's profile page. Always `false`, unless the bot has the `can_post_messages` administrator right in the channel.
    #[must_use]
    pub fn exclude_saved<T: Into<bool>>(mut self, val: T) -> Self {
        self.exclude_saved = Some(val.into());
        self
    }

    /// Pass `true` to exclude gifts that are saved to the chat's profile page. Always `false`, unless the bot has the `can_post_messages` administrator right in the channel.
    #[must_use]
    pub fn exclude_saved_option<T: Into<bool>>(mut self, val: Option<T>) -> Self {
        self.exclude_saved = val.map(Into::into);
        self
    }

    /// Pass `true` to exclude gifts that can be purchased an unlimited number of times
    #[must_use]
    pub fn exclude_unlimited<T: Into<bool>>(mut self, val: T) -> Self {
        self.exclude_unlimited = Some(val.into());
        self
    }

    /// Pass `true` to exclude gifts that can be purchased an unlimited number of times
    #[must_use]
    pub fn exclude_unlimited_option<T: Into<bool>>(mut self, val: Option<T>) -> Self {
        self.exclude_unlimited = val.map(Into::into);
        self
    }

    /// Pass `true` to exclude gifts that can be purchased a limited number of times and can be upgraded to unique
    #[must_use]
    pub fn exclude_limited_upgradable<T: Into<bool>>(mut self, val: T) -> Self {
        self.exclude_limited_upgradable = Some(val.into());
        self
    }

    /// Pass `true` to exclude gifts that can be purchased a limited number of times and can be upgraded to unique
    #[must_use]
    pub fn exclude_limited_upgradable_option<T: Into<bool>>(mut self, val: Option<T>) -> Self {
        self.exclude_limited_upgradable = val.map(Into::into);
        self
    }

    /// Pass `true` to exclude gifts that can be purchased a limited number of times and can't be upgraded to unique
    #[must_use]
    pub fn exclude_limited_non_upgradable<T: Into<bool>>(mut self, val: T) -> Self {
        self.exclude_limited_non_upgradable = Some(val.into());
        self
    }

    /// Pass `true` to exclude gifts that can be purchased a limited number of times and can't be upgraded to unique
    #[must_use]
    pub fn exclude_limited_non_upgradable_option<T: Into<bool>>(mut self, val: Option<T>) -> Self {
        self.exclude_limited_non_upgradable = val.map(Into::into);
        self
    }

    /// Pass `true` to exclude gifts that were assigned from the TON blockchain and can't be resold or transferred in Telegram
    #[must_use]
    pub fn exclude_from_blockchain<T: Into<bool>>(mut self, val: T) -> Self {
        self.exclude_from_blockchain = Some(val.into());
        self
    }

    /// Pass `true` to exclude gifts that were assigned from the TON blockchain and can't be resold or transferred in Telegram
    #[must_use]
    pub fn exclude_from_blockchain_option<T: Into<bool>>(mut self, val: Option<T>) -> Self {
        self.exclude_from_blockchain = val.map(Into::into);
        self
    }

    /// Pass `true` to exclude unique gifts
    #[must_use]
    pub fn exclude_unique<T: Into<bool>>(mut self, val: T) -> Self {
        self.exclude_unique = Some(val.into());
        self
    }

    /// Pass `true` to exclude unique gifts
    #[must_use]
    pub fn exclude_unique_option<T: Into<bool>>(mut self, val: Option<T>) -> Self {
        self.exclude_unique = val.map(Into::into);
        self
    }

    /// Pass `true` to sort results by gift price instead of send date. Sorting is applied before pagination.
    #[must_use]
    pub fn sort_by_price<T: Into<bool>>(mut self, val: T) -> Self {
        self.sort_by_price = Some(val.into());
        self
    }

    /// Pass `true` to sort results by gift price instead of send date. Sorting is applied before pagination.
    #[must_use]
    pub fn sort_by_price_option<T: Into<bool>>(mut self, val: Option<T>) -> Self {
        self.sort_by_price = val.map(Into::into);
        self
    }

    /// Offset of the first entry to return as received from the previous request; use an empty string to get the first chunk of results
    #[must_use]
    pub fn offset<T: Into<Box<str>>>(mut self, val: T) -> Self {
        self.offset = Some(val.into());
        self
    }

    /// Offset of the first entry to return as received from the previous request; use an empty string to get the first chunk of results
    #[must_use]
    pub fn offset_option<T: Into<Box<str>>>(mut self, val: Option<T>) -> Self {
        self.offset = val.map(Into::into);
        self
    }

    /// The maximum number of gifts to be returned; 1-100. Defaults to 100.
    #[must_use]
    pub fn limit<T: Into<u8>>(mut self, val: T) -> Self {
        self.limit = Some(val.into());
        self
    }

    /// The maximum number of gifts to be returned; 1-100. Defaults to 100.
    #[must_use]
    pub fn limit_option<T: Into<u8>>(mut self, val: Option<T>) -> Self {
        self.limit = val.map(Into::into);
        self
    }
}
impl super::TelegramMethod for GetChatGifts {
    type Method = Self;
    type Return = crate::types::OwnedGifts;

    fn build_request<Client>(self, _bot: &Bot<Client>) -> super::Request<Self::Method> {
        super::Request::new("getChatGifts", self, None)
    }
}
