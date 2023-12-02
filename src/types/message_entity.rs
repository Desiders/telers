use super::User;

use serde::{Deserialize, Serialize};
use serde_with::skip_serializing_none;
use strum_macros::Display;

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

#[derive(Debug, Display, Clone, PartialEq, Eq, Hash, Deserialize, Serialize)]
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

impl Pre {
    #[must_use]
    pub const fn new() -> Self {
        Self { language: None }
    }

    #[must_use]
    pub fn new_language(language: impl Into<String>) -> Self {
        Self {
            language: Some(language.into()),
        }
    }
}

#[derive(Debug, Clone, Hash, PartialEq, Eq, Deserialize, Serialize)]
pub struct TextLink {
    pub url: String,
}

impl TextLink {
    #[must_use]
    pub fn new(url: impl Into<String>) -> Self {
        Self { url: url.into() }
    }
}

#[derive(Debug, Clone, Hash, PartialEq, Eq, Deserialize, Serialize)]
pub struct TextMention {
    pub user: User,
}

impl TextMention {
    #[must_use]
    pub fn new(user: User) -> Self {
        Self { user }
    }
}

#[derive(Debug, Clone, Hash, PartialEq, Eq, Deserialize, Serialize)]
pub struct CustomEmoji {
    pub custom_emoji_id: String,
}

impl CustomEmoji {
    #[must_use]
    pub fn new(custom_emoji_id: impl Into<String>) -> Self {
        Self {
            custom_emoji_id: custom_emoji_id.into(),
        }
    }
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
    pub fn new_pre(offset: u16, length: u16) -> Self {
        Self::new(offset, length, Kind::Pre(Pre::new()))
    }

    #[must_use]
    pub fn new_pre_language(offset: u16, length: u16, language: impl Into<String>) -> Self {
        Self::new(offset, length, Kind::Pre(Pre::new_language(language)))
    }

    #[must_use]
    pub fn new_text_link(offset: u16, length: u16, url: impl Into<String>) -> Self {
        Self::new(offset, length, Kind::TextLink(TextLink::new(url)))
    }

    #[must_use]
    pub fn new_custom_emoji(offset: u16, length: u16, custom_emoji_id: impl Into<String>) -> Self {
        Self::new(
            offset,
            length,
            Kind::CustomEmoji(CustomEmoji::new(custom_emoji_id)),
        )
    }

    #[must_use]
    pub fn new_text_mention(offset: u16, length: u16, user: User) -> Self {
        Self::new(offset, length, Kind::TextMention(TextMention::new(user)))
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
