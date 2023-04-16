use crate::utils::text_decorations::{add_surrogates, remove_surrogates};

use super::User;

use serde::{Deserialize, Serialize};
use serde_with::skip_serializing_none;

/// This object represents one special entity in a text message. For example, hashtags, usernames, URLs, etc.
/// # Documentation
/// <https://core.telegram.org/bots/api#messageentity>
#[skip_serializing_none]
#[derive(Clone, Debug, Eq, Hash, PartialEq, Serialize, Deserialize)]
pub struct MessageEntity {
    /// Type of the entity. Currently, can be 'mention' (:code:`@username`), 'hashtag' (:code:`#hashtag`), 'cashtag' (:code:`$USD`), 'bot_command' (:code:`/start@jobs_bot`), 'url' (:code:`https://telegram.org`), 'email' (:code:`do-not-reply@telegram.org`), 'phone_number' (:code:`+1-212-555-0123`), 'bold' (**bold text**), 'italic' (*italic text*), 'underline' (underlined text), 'strikethrough' (strikethrough text), 'spoiler' (spoiler message), 'code' (monowidth string), 'pre' (monowidth block), 'text_link' (for clickable text URLs), 'text_mention' (for users [`without usernames`](https://telegram.org/blog/edit#new-mentions)), 'custom_emoji' (for inline custom emoji stickers)
    #[serde(rename = "type")]
    pub entity_type: String,
    /// Offset in UTF-16 code units to the start of the entity
    pub offset: i64,
    /// Length of the entity in UTF-16 code units
    pub length: i64,
    /// *Optional*. For 'text_link' only, URL that will be opened after user taps on the text
    pub url: Option<String>,
    /// *Optional*. For 'text_mention' only, the mentioned user
    pub user: Option<User>,
    /// *Optional*. For 'pre' only, the programming language of the entity text
    pub language: Option<String>,
    /// *Optional*. For 'custom_emoji' only, unique identifier of the custom emoji. Use [`GetCustomEmojiStickers`](crate::methods::GetCustomEmojiStickers) to get full information about the sticker
    pub custom_emoji_id: Option<String>,
}

impl MessageEntity {
    #[must_use]
    pub fn new<T: Into<String>>(entity_type: T, offset: i64, length: i64) -> Self {
        Self {
            entity_type: entity_type.into(),
            offset,
            length,
            url: None,
            user: None,
            language: None,
            custom_emoji_id: None,
        }
    }

    #[must_use]
    pub fn offset(mut self, val: i64) -> Self {
        self.offset = val;
        self
    }

    #[must_use]
    pub fn length(mut self, val: i64) -> Self {
        self.length = val;
        self
    }

    #[must_use]
    pub fn url(mut self, val: impl Into<String>) -> Self {
        self.url = Some(val.into());
        self
    }

    #[must_use]
    pub fn user(mut self, val: User) -> Self {
        self.user = Some(val);
        self
    }

    #[must_use]
    pub fn language(mut self, val: impl Into<String>) -> Self {
        self.language = Some(val.into());
        self
    }

    #[must_use]
    pub fn custom_emoji_id(mut self, val: impl Into<String>) -> Self {
        self.custom_emoji_id = Some(val.into());
        self
    }
}

impl MessageEntity {
    /// # Panics
    /// If the `self.offset` or `self.offset + self.length` is out of the range
    #[must_use]
    pub fn extract_from(&self, text: &str) -> String {
        let with_surrogates = add_surrogates(
            &text[usize::try_from(self.offset).unwrap() * 2
                ..usize::try_from(self.offset + self.length).unwrap() * 2],
        );

        remove_surrogates(&with_surrogates)
    }
}
