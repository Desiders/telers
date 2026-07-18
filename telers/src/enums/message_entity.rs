use crate::types::MessageEntity;
use serde::{Deserialize, Serialize};
use strum_macros::{AsRefStr, Display, EnumString, IntoStaticStr};
/// This object represents one special entity in a text message. For example, hashtags, usernames, URLs, etc.
/// Currently, it can be one of
/// - [`crate::types::MessageEntityBlockquote`]
/// - [`crate::types::MessageEntityBold`]
/// - [`crate::types::MessageEntityBotCommand`]
/// - [`crate::types::MessageEntityCashtag`]
/// - [`crate::types::MessageEntityCode`]
/// - [`crate::types::MessageEntityCustomEmoji`]
/// - [`crate::types::MessageEntityDateTime`]
/// - [`crate::types::MessageEntityEmail`]
/// - [`crate::types::MessageEntityExpandableBlockquote`]
/// - [`crate::types::MessageEntityHashtag`]
/// - [`crate::types::MessageEntityItalic`]
/// - [`crate::types::MessageEntityMention`]
/// - [`crate::types::MessageEntityPhoneNumber`]
/// - [`crate::types::MessageEntityPre`]
/// - [`crate::types::MessageEntitySpoiler`]
/// - [`crate::types::MessageEntityStrikethrough`]
/// - [`crate::types::MessageEntityTextLink`]
/// - [`crate::types::MessageEntityTextMention`]
/// - [`crate::types::MessageEntityUnderline`]
/// - [`crate::types::MessageEntityUrl`]
/// # Documentation
/// <https://core.telegram.org/bots/api#messageentity>
#[derive(
    Debug,
    Display,
    Clone,
    Copy,
    PartialEq,
    Eq,
    Hash,
    EnumString,
    AsRefStr,
    IntoStaticStr,
    Deserialize,
    Serialize,
)]
pub enum MessageEntityType {
    #[strum(serialize = "mention")]
    Mention,
    #[strum(serialize = "hashtag")]
    Hashtag,
    #[strum(serialize = "cashtag")]
    Cashtag,
    #[strum(serialize = "bot_command")]
    BotCommand,
    #[strum(serialize = "url")]
    Url,
    #[strum(serialize = "email")]
    Email,
    #[strum(serialize = "phone_number")]
    PhoneNumber,
    #[strum(serialize = "bold")]
    Bold,
    #[strum(serialize = "italic")]
    Italic,
    #[strum(serialize = "underline")]
    Underline,
    #[strum(serialize = "strikethrough")]
    Strikethrough,
    #[strum(serialize = "spoiler")]
    Spoiler,
    #[strum(serialize = "blockquote")]
    Blockquote,
    #[strum(serialize = "expandable_blockquote")]
    ExpandableBlockquote,
    #[strum(serialize = "code")]
    Code,
    #[strum(serialize = "pre")]
    Pre,
    #[strum(serialize = "text_link")]
    TextLink,
    #[strum(serialize = "text_mention")]
    TextMention,
    #[strum(serialize = "custom_emoji")]
    CustomEmoji,
    #[strum(serialize = "date_time")]
    DateTime,
    #[strum(serialize = "unknown")]
    Unknown,
}
impl MessageEntityType {
    #[must_use]
    pub const fn all() -> [MessageEntityType; 21usize] {
        [
            MessageEntityType::Mention,
            MessageEntityType::Hashtag,
            MessageEntityType::Cashtag,
            MessageEntityType::BotCommand,
            MessageEntityType::Url,
            MessageEntityType::Email,
            MessageEntityType::PhoneNumber,
            MessageEntityType::Bold,
            MessageEntityType::Italic,
            MessageEntityType::Underline,
            MessageEntityType::Strikethrough,
            MessageEntityType::Spoiler,
            MessageEntityType::Blockquote,
            MessageEntityType::ExpandableBlockquote,
            MessageEntityType::Code,
            MessageEntityType::Pre,
            MessageEntityType::TextLink,
            MessageEntityType::TextMention,
            MessageEntityType::CustomEmoji,
            MessageEntityType::DateTime,
            MessageEntityType::Unknown,
        ]
    }
}
impl From<MessageEntityType> for Box<str> {
    fn from(val: MessageEntityType) -> Self {
        Into::<&'static str>::into(val).into()
    }
}
impl From<MessageEntityType> for String {
    fn from(val: MessageEntityType) -> Self {
        val.as_ref().to_owned()
    }
}
impl<'a> PartialEq<&'a str> for MessageEntityType {
    fn eq(&self, other: &&'a str) -> bool {
        self.as_ref() == *other
    }
}
impl<'a> From<&'a MessageEntity> for MessageEntityType {
    fn from(val: &'a MessageEntity) -> Self {
        match val {
            MessageEntity::Mention(_) => MessageEntityType::Mention,
            MessageEntity::Hashtag(_) => MessageEntityType::Hashtag,
            MessageEntity::Cashtag(_) => MessageEntityType::Cashtag,
            MessageEntity::BotCommand(_) => MessageEntityType::BotCommand,
            MessageEntity::Url(_) => MessageEntityType::Url,
            MessageEntity::Email(_) => MessageEntityType::Email,
            MessageEntity::PhoneNumber(_) => MessageEntityType::PhoneNumber,
            MessageEntity::Bold(_) => MessageEntityType::Bold,
            MessageEntity::Italic(_) => MessageEntityType::Italic,
            MessageEntity::Underline(_) => MessageEntityType::Underline,
            MessageEntity::Strikethrough(_) => MessageEntityType::Strikethrough,
            MessageEntity::Spoiler(_) => MessageEntityType::Spoiler,
            MessageEntity::Blockquote(_) => MessageEntityType::Blockquote,
            MessageEntity::ExpandableBlockquote(_) => MessageEntityType::ExpandableBlockquote,
            MessageEntity::Code(_) => MessageEntityType::Code,
            MessageEntity::Pre(_) => MessageEntityType::Pre,
            MessageEntity::TextLink(_) => MessageEntityType::TextLink,
            MessageEntity::TextMention(_) => MessageEntityType::TextMention,
            MessageEntity::CustomEmoji(_) => MessageEntityType::CustomEmoji,
            MessageEntity::DateTime(_) => MessageEntityType::DateTime,
            MessageEntity::Unknown(_) => MessageEntityType::Unknown,
        }
    }
}
impl From<MessageEntity> for MessageEntityType {
    fn from(val: MessageEntity) -> Self {
        MessageEntityType::from(&val)
    }
}
