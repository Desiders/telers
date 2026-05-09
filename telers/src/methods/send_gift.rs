use crate::client::Bot;
use serde::Serialize;
/// Sends a gift to the given user or channel chat. The gift can't be converted to Telegram Stars by the receiver. Returns `true` on success.
/// # Documentation
/// <https://core.telegram.org/bots/api#sendgift>
/// # Returns
/// - `bool`
#[derive(Clone, Debug, Serialize)]
pub struct SendGift {
    /// Required if `chat_id` is not specified. Unique identifier of the target user who will receive the gift.
    #[serde(skip_serializing_if = "Option::is_none")]
    pub user_id: Option<i64>,
    /// Required if `user_id` is not specified. Unique identifier for the chat or username of the channel (in the format @username) that will receive the gift.
    #[serde(skip_serializing_if = "Option::is_none")]
    pub chat_id: Option<crate::types::ChatIdKind>,
    /// Identifier of the gift; limited gifts can't be sent to channel chats
    pub gift_id: Box<str>,
    /// Pass `true` to pay for the gift upgrade from the bot's balance, thereby making the upgrade free for the receiver
    #[serde(skip_serializing_if = "Option::is_none")]
    pub pay_for_upgrade: Option<bool>,
    /// Text that will be shown along with the gift; 0-128 characters
    #[serde(skip_serializing_if = "Option::is_none")]
    pub text: Option<Box<str>>,
    /// Mode for parsing entities in the text. See formatting options for more details. Entities other than `bold`, `italic`, `underline`, `strikethrough`, `spoiler`, `custom_emoji`, and `date_time` are ignored.
    #[serde(skip_serializing_if = "Option::is_none")]
    pub text_parse_mode: Option<Box<str>>,
    /// A JSON-serialized list of special entities that appear in the gift text. It can be specified instead of `text_parse_mode`. Entities other than `bold`, `italic`, `underline`, `strikethrough`, `spoiler`, `custom_emoji`, and `date_time` are ignored.
    #[serde(skip_serializing_if = "Option::is_none")]
    pub text_entities: Option<Box<[crate::types::MessageEntity]>>,
}
impl SendGift {
    /// Creates a new `SendGift`.
    ///
    /// # Arguments
    /// * `gift_id` - Identifier of the gift; limited gifts can't be sent to channel chats
    ///
    /// # Notes
    /// Use builder methods to set optional fields.
    #[must_use]
    pub fn new<T0: Into<Box<str>>>(gift_id: T0) -> Self {
        Self {
            user_id: None,
            chat_id: None,
            gift_id: gift_id.into(),
            pay_for_upgrade: None,
            text: None,
            text_parse_mode: None,
            text_entities: None,
        }
    }

    /// Required if `chat_id` is not specified. Unique identifier of the target user who will receive the gift.
    #[must_use]
    pub fn user_id<T: Into<i64>>(self, val: T) -> Self {
        let mut this = self;
        this.user_id = Some(val.into());
        this
    }

    /// Required if `chat_id` is not specified. Unique identifier of the target user who will receive the gift.
    #[must_use]
    pub fn user_id_option<T: Into<i64>>(self, val: Option<T>) -> Self {
        let mut this = self;
        this.user_id = val.map(Into::into);
        this
    }

    /// Required if `user_id` is not specified. Unique identifier for the chat or username of the channel (in the format @username) that will receive the gift.
    #[must_use]
    pub fn chat_id<T: Into<crate::types::ChatIdKind>>(self, val: T) -> Self {
        let mut this = self;
        this.chat_id = Some(val.into());
        this
    }

    /// Required if `user_id` is not specified. Unique identifier for the chat or username of the channel (in the format @username) that will receive the gift.
    #[must_use]
    pub fn chat_id_option<T: Into<crate::types::ChatIdKind>>(self, val: Option<T>) -> Self {
        let mut this = self;
        this.chat_id = val.map(Into::into);
        this
    }

    /// Identifier of the gift; limited gifts can't be sent to channel chats
    #[must_use]
    pub fn gift_id<T: Into<Box<str>>>(self, val: T) -> Self {
        let mut this = self;
        this.gift_id = val.into();
        this
    }

    /// Pass `true` to pay for the gift upgrade from the bot's balance, thereby making the upgrade free for the receiver
    #[must_use]
    pub fn pay_for_upgrade<T: Into<bool>>(self, val: T) -> Self {
        let mut this = self;
        this.pay_for_upgrade = Some(val.into());
        this
    }

    /// Pass `true` to pay for the gift upgrade from the bot's balance, thereby making the upgrade free for the receiver
    #[must_use]
    pub fn pay_for_upgrade_option<T: Into<bool>>(self, val: Option<T>) -> Self {
        let mut this = self;
        this.pay_for_upgrade = val.map(Into::into);
        this
    }

    /// Text that will be shown along with the gift; 0-128 characters
    #[must_use]
    pub fn text<T: Into<Box<str>>>(self, val: T) -> Self {
        let mut this = self;
        this.text = Some(val.into());
        this
    }

    /// Text that will be shown along with the gift; 0-128 characters
    #[must_use]
    pub fn text_option<T: Into<Box<str>>>(self, val: Option<T>) -> Self {
        let mut this = self;
        this.text = val.map(Into::into);
        this
    }

    /// Mode for parsing entities in the text. See formatting options for more details. Entities other than `bold`, `italic`, `underline`, `strikethrough`, `spoiler`, `custom_emoji`, and `date_time` are ignored.
    #[must_use]
    pub fn text_parse_mode<T: Into<Box<str>>>(self, val: T) -> Self {
        let mut this = self;
        this.text_parse_mode = Some(val.into());
        this
    }

    /// Mode for parsing entities in the text. See formatting options for more details. Entities other than `bold`, `italic`, `underline`, `strikethrough`, `spoiler`, `custom_emoji`, and `date_time` are ignored.
    #[must_use]
    pub fn text_parse_mode_option<T: Into<Box<str>>>(self, val: Option<T>) -> Self {
        let mut this = self;
        this.text_parse_mode = val.map(Into::into);
        this
    }

    /// A JSON-serialized list of special entities that appear in the gift text. It can be specified instead of `text_parse_mode`. Entities other than `bold`, `italic`, `underline`, `strikethrough`, `spoiler`, `custom_emoji`, and `date_time` are ignored.
    ///
    /// # Notes
    /// Adds multiple elements.
    #[must_use]
    pub fn text_entities<
        TItem: Into<crate::types::MessageEntity>,
        T: IntoIterator<Item = TItem>,
    >(
        self,
        val: T,
    ) -> Self {
        let mut this = self;
        this.text_entities = Some(
            this.text_entities
                .unwrap_or_default()
                .into_vec()
                .into_iter()
                .chain(val.into_iter().map(Into::into))
                .collect(),
        );
        this
    }

    /// A JSON-serialized list of special entities that appear in the gift text. It can be specified instead of `text_parse_mode`. Entities other than `bold`, `italic`, `underline`, `strikethrough`, `spoiler`, `custom_emoji`, and `date_time` are ignored.
    ///
    /// # Notes
    /// Adds a single element.
    #[must_use]
    pub fn text_entity<T: Into<crate::types::MessageEntity>>(self, val: T) -> Self {
        let mut this = self;
        this.text_entities = Some(
            this.text_entities
                .unwrap_or_default()
                .into_vec()
                .into_iter()
                .chain(Some(val.into()))
                .collect(),
        );
        this
    }

    /// A JSON-serialized list of special entities that appear in the gift text. It can be specified instead of `text_parse_mode`. Entities other than `bold`, `italic`, `underline`, `strikethrough`, `spoiler`, `custom_emoji`, and `date_time` are ignored.
    ///
    /// # Notes
    /// Adds multiple elements.
    #[must_use]
    pub fn text_entities_option<
        TItem: Into<crate::types::MessageEntity>,
        T: IntoIterator<Item = TItem>,
    >(
        self,
        val: Option<T>,
    ) -> Self {
        let mut this = self;
        this.text_entities = val.map(|v| v.into_iter().map(Into::into).collect());
        this
    }
}
impl super::TelegramMethod for SendGift {
    type Method = Self;
    type Return = bool;

    fn build_request<Client>(self, _bot: &Bot<Client>) -> super::Request<Self::Method> {
        super::Request::new("sendGift", self, None)
    }
}
