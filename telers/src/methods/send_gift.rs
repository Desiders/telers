use super::base::{Request, TelegramMethod};

use crate::{
    client::Bot,
    types::{ChatIdKind, MessageEntity},
};

use serde::Serialize;
use serde_with::skip_serializing_none;

/// Sends a gift to the given user. The gift can't be converted to Telegram Stars by the user.
/// # Documentation
/// <https://core.telegram.org/bots/api#sendgift>
/// # Returns
/// On success, `true` is returned
#[skip_serializing_none]
#[derive(Debug, Clone, Hash, PartialEq, Eq, Serialize)]
pub struct SendGift {
    /// Required if `chat_id` is not specified. Unique identifier of the target user who will receive the gift.
    pub user_id: Option<i64>,
    /// Required if `user_id` is not specified. Unique identifier for the chat or username of the channel (in the format `@channelusername`)
    pub chat_id: Option<ChatIdKind>,
    /// Identifier of the gift
    pub gift_id: String,
    /// Pass `true` to pay for the gift upgrade from the bot's balance, thereby making the upgrade free for the receiver
    pub pay_for_upgrade: Option<bool>,
    /// Text that will be shown along with the gift; 0-255 characters
    pub text: Option<String>,
    /// Mode for parsing entities in the text. See [formatting options](https://core.telegram.org/bots/api#formatting-options) for more details. Entities other than `bold`, `italic`, `underline`, `strikethrough`, `spoiler`, and `custom_emoji` are ignored.
    pub text_parse_mode: Option<String>,
    /// A JSON-serialized list of special entities that appear in the gift text. It can be specified instead of `text_parse_mode`. Entities other than `bold`, `italic`, `underline`, `strikethrough`, `spoiler`, and `custom_emoji` are ignored.
    pub text_entities: Option<Vec<MessageEntity>>,
}

impl SendGift {
    #[must_use]
    pub fn new(gift_id: impl Into<String>) -> Self {
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

    #[must_use]
    pub fn user_id(self, val: i64) -> Self {
        Self {
            user_id: Some(val),
            ..self
        }
    }

    #[must_use]
    pub fn chat_id(self, val: impl Into<ChatIdKind>) -> Self {
        Self {
            chat_id: Some(val.into()),
            ..self
        }
    }

    #[must_use]
    pub fn gift_id(self, val: impl Into<String>) -> Self {
        Self {
            gift_id: val.into(),
            ..self
        }
    }

    #[must_use]
    pub fn pay_for_upgrade(self, val: bool) -> Self {
        Self {
            pay_for_upgrade: Some(val),
            ..self
        }
    }

    #[must_use]
    pub fn text(self, val: impl Into<String>) -> Self {
        Self {
            text: Some(val.into()),
            ..self
        }
    }

    #[must_use]
    pub fn text_parse_mode(self, val: impl Into<String>) -> Self {
        Self {
            text_parse_mode: Some(val.into()),
            ..self
        }
    }

    #[must_use]
    pub fn text_entities(self, val: impl IntoIterator<Item = MessageEntity>) -> Self {
        Self {
            text_entities: Some(
                self.text_entities
                    .unwrap_or_default()
                    .into_iter()
                    .chain(val)
                    .collect(),
            ),
            ..self
        }
    }
}

impl SendGift {
    #[must_use]
    pub fn user_id_option(self, val: Option<i64>) -> Self {
        Self {
            user_id: val,
            ..self
        }
    }

    #[must_use]
    pub fn chat_id_option(self, val: Option<impl Into<ChatIdKind>>) -> Self {
        Self {
            chat_id: val.map(Into::into),
            ..self
        }
    }

    #[must_use]
    pub fn pay_for_upgrade_option(self, val: Option<bool>) -> Self {
        Self {
            pay_for_upgrade: val,
            ..self
        }
    }

    #[must_use]
    pub fn text_option(self, val: Option<impl Into<String>>) -> Self {
        Self {
            text: val.map(Into::into),
            ..self
        }
    }

    #[must_use]
    pub fn text_parse_mode_option(self, val: Option<impl Into<String>>) -> Self {
        Self {
            text_parse_mode: val.map(Into::into),
            ..self
        }
    }

    #[must_use]
    pub fn text_entities_option(
        self,
        val: Option<impl IntoIterator<Item = MessageEntity>>,
    ) -> Self {
        Self {
            text_entities: val.map(|val| {
                self.text_entities
                    .unwrap_or_default()
                    .into_iter()
                    .chain(val)
                    .collect()
            }),
            ..self
        }
    }
}

impl TelegramMethod for SendGift {
    type Method = Self;
    type Return = bool;

    fn build_request<Client>(self, _bot: &Bot<Client>) -> Request<Self::Method> {
        Request::new("sendGift", self, None)
    }
}

impl AsRef<SendGift> for SendGift {
    fn as_ref(&self) -> &Self {
        self
    }
}
