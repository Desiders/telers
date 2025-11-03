use super::base::{Request, TelegramMethod};

use crate::{client::Bot, types::MessageEntity};

use serde::Serialize;
use serde_with::skip_serializing_none;

/// Gifts a Telegram Premium subscription to the given user.
/// # Documentation
/// <https://core.telegram.org/bots/api#giftpremiumsubscription>
/// # Returns
/// On success, `true` is returned
#[skip_serializing_none]
#[derive(Debug, Clone, Hash, PartialEq, Eq, Serialize)]
pub struct GiftPremiumSubscription {
    /// Unique identifier of the target user who will receive a Telegram Premium subscription
    pub user_id: i64,
    /// Number of months the Telegram Premium subscription will be active for the user; must be one of 3, 6, or 12
    pub month_count: u8,
    /// Number of Telegram Stars to pay for the Telegram Premium subscription; must be 1000 for 3 months, 1500 for 6 months, and 2500 for 12 months
    pub star_count: u16,
    /// Text that will be shown along with the service message about the subscription; 0-128 characters
    pub text: Option<String>,
    /// Mode for parsing entities in the text. See [formatting options](https://core.telegram.org/bots/api#formatting-options) for more details. Entities other than `bold`, `italic`, `underline`, `strikethrough`, `spoiler`, and `custom_emoji` are ignored.
    pub text_parse_mode: Option<String>,
    /// A JSON-serialized list of special entities that appear in the gift text. It can be specified instead of `text_parse_mode`. Entities other than `bold`, `italic`, `underline`, `strikethrough`, `spoiler`, and `custom_emoji` are ignored.
    pub text_entities: Option<Vec<MessageEntity>>,
}

impl GiftPremiumSubscription {
    #[must_use]
    pub fn new(user_id: i64, month_count: impl Into<u8>, star_count: impl Into<u16>) -> Self {
        Self {
            user_id,
            month_count: month_count.into(),
            star_count: star_count.into(),
            text: None,
            text_parse_mode: None,
            text_entities: None,
        }
    }

    #[must_use]
    pub fn user_id(self, val: i64) -> Self {
        Self {
            user_id: val,
            ..self
        }
    }

    #[must_use]
    pub fn month_count(self, val: impl Into<u8>) -> Self {
        Self {
            month_count: val.into(),
            ..self
        }
    }

    #[must_use]
    pub fn star_count(self, val: impl Into<u16>) -> Self {
        Self {
            star_count: val.into(),
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
    pub fn text_entity(self, val: MessageEntity) -> Self {
        Self {
            text_entities: Some(
                self.text_entities
                    .unwrap_or_default()
                    .into_iter()
                    .chain(Some(val))
                    .collect(),
            ),
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

impl GiftPremiumSubscription {
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

impl TelegramMethod for GiftPremiumSubscription {
    type Method = Self;
    type Return = bool;

    fn build_request<Client>(&self, _bot: &Bot<Client>) -> Request<Self::Method> {
        Request::new("giftPremiumSubscription", self, None)
    }
}

impl AsRef<GiftPremiumSubscription> for GiftPremiumSubscription {
    fn as_ref(&self) -> &Self {
        self
    }
}
