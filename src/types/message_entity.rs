use super::User;

use crate::utils::text_decorations::{add_surrogates, remove_surrogates};

use serde::{Deserialize, Serialize};
use serde_with::skip_serializing_none;

/// This object represents one special entity in a text message. For example, hashtags, usernames, URLs, etc.
/// # Documentation
/// <https://core.telegram.org/bots/api#messageentity>
#[derive(Debug, Clone, Hash, PartialEq, Eq, Deserialize, Serialize)]
pub struct MessageEntity {
    /// Offset in UTF-16 code units to the start of the entity
    pub offset: u16,
    /// Length of the entity in UTF-16 code units
    pub length: u16,

    #[serde(flatten)]
    pub kind: Kind,
}

#[derive(Debug, Clone, Hash, PartialEq, Eq, Deserialize, Serialize)]
#[serde(tag = "type", rename_all = "snake_case")]
pub enum Kind {
    Mention,
    Hashtag,
    Cashtag,
    BotCommand,
    Url,
    Email,
    PhoneNumber,
    Bold,
    Italic,
    Underline,
    Strikethrough,
    Spoiler,
    Code,
    Pre(Pre),
    TextLink(TextLink),
    TextMention(TextMention),
    CustomEmoji(CustomEmoji),
}

#[skip_serializing_none]
#[derive(Debug, Clone, Hash, PartialEq, Eq, Deserialize, Serialize)]
pub struct Pre {
    pub language: Option<String>,
}

#[derive(Debug, Clone, Hash, PartialEq, Eq, Deserialize, Serialize)]
pub struct TextLink {
    pub url: String,
}

#[derive(Debug, Clone, Hash, PartialEq, Eq, Deserialize, Serialize)]
pub struct TextMention {
    pub user: User,
}

#[derive(Debug, Clone, Hash, PartialEq, Eq, Deserialize, Serialize)]
pub struct CustomEmoji {
    pub custom_emoji_id: i64,
}

impl MessageEntity {
    #[must_use]
    pub fn new(offset: u16, length: u16, kind: impl Into<Kind>) -> Self {
        Self {
            offset,
            length,
            kind: kind.into(),
        }
    }

    #[must_use]
    pub fn new_mention(offset: u16, length: u16) -> Self {
        Self::new(offset, length, Kind::Mention)
    }

    #[must_use]
    pub fn new_hashtag(offset: u16, length: u16) -> Self {
        Self::new(offset, length, Kind::Hashtag)
    }

    #[must_use]
    pub fn new_cashtag(offset: u16, length: u16) -> Self {
        Self::new(offset, length, Kind::Cashtag)
    }

    #[must_use]
    pub fn new_bot_command(offset: u16, length: u16) -> Self {
        Self::new(offset, length, Kind::BotCommand)
    }

    #[must_use]
    pub fn new_url(offset: u16, length: u16) -> Self {
        Self::new(offset, length, Kind::Url)
    }

    #[must_use]
    pub fn new_email(offset: u16, length: u16) -> Self {
        Self::new(offset, length, Kind::Email)
    }

    #[must_use]
    pub fn new_phone_number(offset: u16, length: u16) -> Self {
        Self::new(offset, length, Kind::PhoneNumber)
    }

    #[must_use]
    pub fn new_bold(offset: u16, length: u16) -> Self {
        Self::new(offset, length, Kind::Bold)
    }

    #[must_use]
    pub fn new_italic(offset: u16, length: u16) -> Self {
        Self::new(offset, length, Kind::Italic)
    }

    #[must_use]
    pub fn new_underline(offset: u16, length: u16) -> Self {
        Self::new(offset, length, Kind::Underline)
    }

    #[must_use]
    pub fn new_strikethrough(offset: u16, length: u16) -> Self {
        Self::new(offset, length, Kind::Strikethrough)
    }

    #[must_use]
    pub fn new_spoiler(offset: u16, length: u16) -> Self {
        Self::new(offset, length, Kind::Spoiler)
    }

    #[must_use]
    pub fn new_code(offset: u16, length: u16) -> Self {
        Self::new(offset, length, Kind::Code)
    }

    #[must_use]
    pub fn offset(self, val: u16) -> Self {
        Self {
            offset: val,
            ..self
        }
    }

    #[must_use]
    pub fn length(self, val: u16) -> Self {
        Self {
            length: val,
            ..self
        }
    }
}

impl MessageEntity {
    #[must_use]
    pub fn kind(&self) -> &Kind {
        &self.kind
    }

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

impl From<Pre> for Kind {
    fn from(pre: Pre) -> Self {
        Self::Pre(pre)
    }
}

impl From<TextLink> for Kind {
    fn from(text_link: TextLink) -> Self {
        Self::TextLink(text_link)
    }
}

impl From<TextMention> for Kind {
    fn from(text_mention: TextMention) -> Self {
        Self::TextMention(text_mention)
    }
}

impl From<CustomEmoji> for Kind {
    fn from(custom_emoji: CustomEmoji) -> Self {
        Self::CustomEmoji(custom_emoji)
    }
}
