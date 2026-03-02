use serde::{Deserialize, Serialize};
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
#[derive(Clone, Debug, Serialize, Deserialize)]
#[serde(tag = "type", rename_all = "snake_case")]
pub enum MessageEntity {
    Mention(crate::types::MessageEntityMention),
    Hashtag(crate::types::MessageEntityHashtag),
    Cashtag(crate::types::MessageEntityCashtag),
    BotCommand(crate::types::MessageEntityBotCommand),
    Url(crate::types::MessageEntityUrl),
    Email(crate::types::MessageEntityEmail),
    PhoneNumber(crate::types::MessageEntityPhoneNumber),
    Bold(crate::types::MessageEntityBold),
    Italic(crate::types::MessageEntityItalic),
    Underline(crate::types::MessageEntityUnderline),
    Strikethrough(crate::types::MessageEntityStrikethrough),
    Spoiler(crate::types::MessageEntitySpoiler),
    Blockquote(crate::types::MessageEntityBlockquote),
    ExpandableBlockquote(crate::types::MessageEntityExpandableBlockquote),
    Code(crate::types::MessageEntityCode),
    Pre(crate::types::MessageEntityPre),
    TextLink(crate::types::MessageEntityTextLink),
    TextMention(crate::types::MessageEntityTextMention),
    CustomEmoji(crate::types::MessageEntityCustomEmoji),
    DateTime(crate::types::MessageEntityDateTime),
}
impl MessageEntity {
    /// Helper method for field `custom_emoji_id`.
    ///
    /// For `custom_emoji` only, unique identifier of the custom emoji. Use getCustomEmojiStickers to get full information about the sticker
    #[must_use]
    pub fn custom_emoji_id(&self) -> Option<&str> {
        match self {
            Self::CustomEmoji(val) => Some(val.custom_emoji_id.as_ref()),
            _ => None,
        }
    }

    /// Helper method for field `date_time_format`.
    ///
    /// For `date_time` only, the string that defines the formatting of the date and time. See date-time entity formatting for more details.
    #[must_use]
    pub fn date_time_format(&self) -> Option<&str> {
        match self {
            Self::DateTime(val) => val.date_time_format.as_deref(),
            _ => None,
        }
    }

    /// Helper method for field `language`.
    ///
    /// For `pre` only, the programming language of the entity text
    #[must_use]
    pub fn language(&self) -> Option<&str> {
        match self {
            Self::Pre(val) => val.language.as_deref(),
            _ => None,
        }
    }

    /// Helper method for field `length`.
    ///
    /// Length of the entity in UTF-16 code units
    #[must_use]
    pub fn length(&self) -> i64 {
        match self {
            Self::Mention(val) => val.length,
            Self::Hashtag(val) => val.length,
            Self::Cashtag(val) => val.length,
            Self::BotCommand(val) => val.length,
            Self::Url(val) => val.length,
            Self::Email(val) => val.length,
            Self::PhoneNumber(val) => val.length,
            Self::Bold(val) => val.length,
            Self::Italic(val) => val.length,
            Self::Underline(val) => val.length,
            Self::Strikethrough(val) => val.length,
            Self::Spoiler(val) => val.length,
            Self::Blockquote(val) => val.length,
            Self::ExpandableBlockquote(val) => val.length,
            Self::Code(val) => val.length,
            Self::Pre(val) => val.length,
            Self::TextLink(val) => val.length,
            Self::TextMention(val) => val.length,
            Self::CustomEmoji(val) => val.length,
            Self::DateTime(val) => val.length,
        }
    }

    /// Helper method for field `offset`.
    ///
    /// Offset in UTF-16 code units to the start of the entity
    #[must_use]
    pub fn offset(&self) -> i64 {
        match self {
            Self::Mention(val) => val.offset,
            Self::Hashtag(val) => val.offset,
            Self::Cashtag(val) => val.offset,
            Self::BotCommand(val) => val.offset,
            Self::Url(val) => val.offset,
            Self::Email(val) => val.offset,
            Self::PhoneNumber(val) => val.offset,
            Self::Bold(val) => val.offset,
            Self::Italic(val) => val.offset,
            Self::Underline(val) => val.offset,
            Self::Strikethrough(val) => val.offset,
            Self::Spoiler(val) => val.offset,
            Self::Blockquote(val) => val.offset,
            Self::ExpandableBlockquote(val) => val.offset,
            Self::Code(val) => val.offset,
            Self::Pre(val) => val.offset,
            Self::TextLink(val) => val.offset,
            Self::TextMention(val) => val.offset,
            Self::CustomEmoji(val) => val.offset,
            Self::DateTime(val) => val.offset,
        }
    }

    /// Helper method for field `unix_time`.
    ///
    /// For `date_time` only, the Unix time associated with the entity
    #[must_use]
    pub fn unix_time(&self) -> Option<i64> {
        match self {
            Self::DateTime(val) => Some(val.unix_time),
            _ => None,
        }
    }

    /// Helper method for field `url`.
    ///
    /// For `text_link` only, URL that will be opened after user taps on the text
    #[must_use]
    pub fn url(&self) -> Option<&str> {
        match self {
            Self::TextLink(val) => Some(val.url.as_ref()),
            _ => None,
        }
    }

    /// Helper method for field `user`.
    ///
    /// For `text_mention` only, the mentioned user
    #[must_use]
    pub fn user(&self) -> Option<&crate::types::User> {
        match self {
            Self::TextMention(val) => Some(val.user.as_ref()),
            _ => None,
        }
    }

    /// Helper method for nested field `added_to_attachment_menu`.
    #[must_use]
    pub fn added_to_attachment_menu(&self) -> Option<bool> {
        match self {
            Self::TextMention(val) => {
                let inner = val.user.as_ref();
                inner.added_to_attachment_menu
            }
            _ => None,
        }
    }

    /// Helper method for nested field `allows_users_to_create_topics`.
    #[must_use]
    pub fn allows_users_to_create_topics(&self) -> Option<bool> {
        match self {
            Self::TextMention(val) => {
                let inner = val.user.as_ref();
                inner.allows_users_to_create_topics
            }
            _ => None,
        }
    }

    /// Helper method for nested field `can_connect_to_business`.
    #[must_use]
    pub fn can_connect_to_business(&self) -> Option<bool> {
        match self {
            Self::TextMention(val) => {
                let inner = val.user.as_ref();
                inner.can_connect_to_business
            }
            _ => None,
        }
    }

    /// Helper method for nested field `can_join_groups`.
    #[must_use]
    pub fn can_join_groups(&self) -> Option<bool> {
        match self {
            Self::TextMention(val) => {
                let inner = val.user.as_ref();
                inner.can_join_groups
            }
            _ => None,
        }
    }

    /// Helper method for nested field `can_read_all_group_messages`.
    #[must_use]
    pub fn can_read_all_group_messages(&self) -> Option<bool> {
        match self {
            Self::TextMention(val) => {
                let inner = val.user.as_ref();
                inner.can_read_all_group_messages
            }
            _ => None,
        }
    }

    /// Helper method for nested field `first_name`.
    #[must_use]
    pub fn first_name(&self) -> Option<&str> {
        match self {
            Self::TextMention(val) => {
                let inner = val.user.as_ref();
                Some(inner.first_name.as_ref())
            }
            _ => None,
        }
    }

    /// Helper method for nested field `has_main_web_app`.
    #[must_use]
    pub fn has_main_web_app(&self) -> Option<bool> {
        match self {
            Self::TextMention(val) => {
                let inner = val.user.as_ref();
                inner.has_main_web_app
            }
            _ => None,
        }
    }

    /// Helper method for nested field `has_topics_enabled`.
    #[must_use]
    pub fn has_topics_enabled(&self) -> Option<bool> {
        match self {
            Self::TextMention(val) => {
                let inner = val.user.as_ref();
                inner.has_topics_enabled
            }
            _ => None,
        }
    }

    /// Helper method for nested field `id`.
    #[must_use]
    pub fn id(&self) -> Option<i64> {
        match self {
            Self::TextMention(val) => {
                let inner = val.user.as_ref();
                Some(inner.id)
            }
            _ => None,
        }
    }

    /// Helper method for nested field `is_bot`.
    #[must_use]
    pub fn is_bot(&self) -> Option<bool> {
        match self {
            Self::TextMention(val) => {
                let inner = val.user.as_ref();
                Some(inner.is_bot)
            }
            _ => None,
        }
    }

    /// Helper method for nested field `is_premium`.
    #[must_use]
    pub fn is_premium(&self) -> Option<bool> {
        match self {
            Self::TextMention(val) => {
                let inner = val.user.as_ref();
                inner.is_premium
            }
            _ => None,
        }
    }

    /// Helper method for nested field `language_code`.
    #[must_use]
    pub fn language_code(&self) -> Option<&str> {
        match self {
            Self::TextMention(val) => {
                let inner = val.user.as_ref();
                inner.language_code.as_deref()
            }
            _ => None,
        }
    }

    /// Helper method for nested field `last_name`.
    #[must_use]
    pub fn last_name(&self) -> Option<&str> {
        match self {
            Self::TextMention(val) => {
                let inner = val.user.as_ref();
                inner.last_name.as_deref()
            }
            _ => None,
        }
    }

    /// Helper method for nested field `supports_inline_queries`.
    #[must_use]
    pub fn supports_inline_queries(&self) -> Option<bool> {
        match self {
            Self::TextMention(val) => {
                let inner = val.user.as_ref();
                inner.supports_inline_queries
            }
            _ => None,
        }
    }

    /// Helper method for nested field `username`.
    #[must_use]
    pub fn username(&self) -> Option<&str> {
        match self {
            Self::TextMention(val) => {
                let inner = val.user.as_ref();
                inner.username.as_deref()
            }
            _ => None,
        }
    }
}
impl From<crate::types::MessageEntityMention> for MessageEntity {
    fn from(val: crate::types::MessageEntityMention) -> Self {
        Self::Mention(val)
    }
}
impl TryFrom<MessageEntity> for crate::types::MessageEntityMention {
    type Error = crate::errors::ConvertToTypeError;

    fn try_from(val: MessageEntity) -> Result<Self, Self::Error> {
        if let MessageEntity::Mention(inner) = val {
            Ok(inner)
        } else {
            Err(Self::Error::new(
                stringify!(MessageEntity),
                stringify!(MessageEntityMention),
            ))
        }
    }
}
impl From<crate::types::MessageEntityHashtag> for MessageEntity {
    fn from(val: crate::types::MessageEntityHashtag) -> Self {
        Self::Hashtag(val)
    }
}
impl TryFrom<MessageEntity> for crate::types::MessageEntityHashtag {
    type Error = crate::errors::ConvertToTypeError;

    fn try_from(val: MessageEntity) -> Result<Self, Self::Error> {
        if let MessageEntity::Hashtag(inner) = val {
            Ok(inner)
        } else {
            Err(Self::Error::new(
                stringify!(MessageEntity),
                stringify!(MessageEntityHashtag),
            ))
        }
    }
}
impl From<crate::types::MessageEntityCashtag> for MessageEntity {
    fn from(val: crate::types::MessageEntityCashtag) -> Self {
        Self::Cashtag(val)
    }
}
impl TryFrom<MessageEntity> for crate::types::MessageEntityCashtag {
    type Error = crate::errors::ConvertToTypeError;

    fn try_from(val: MessageEntity) -> Result<Self, Self::Error> {
        if let MessageEntity::Cashtag(inner) = val {
            Ok(inner)
        } else {
            Err(Self::Error::new(
                stringify!(MessageEntity),
                stringify!(MessageEntityCashtag),
            ))
        }
    }
}
impl From<crate::types::MessageEntityBotCommand> for MessageEntity {
    fn from(val: crate::types::MessageEntityBotCommand) -> Self {
        Self::BotCommand(val)
    }
}
impl TryFrom<MessageEntity> for crate::types::MessageEntityBotCommand {
    type Error = crate::errors::ConvertToTypeError;

    fn try_from(val: MessageEntity) -> Result<Self, Self::Error> {
        if let MessageEntity::BotCommand(inner) = val {
            Ok(inner)
        } else {
            Err(Self::Error::new(
                stringify!(MessageEntity),
                stringify!(MessageEntityBotCommand),
            ))
        }
    }
}
impl From<crate::types::MessageEntityUrl> for MessageEntity {
    fn from(val: crate::types::MessageEntityUrl) -> Self {
        Self::Url(val)
    }
}
impl TryFrom<MessageEntity> for crate::types::MessageEntityUrl {
    type Error = crate::errors::ConvertToTypeError;

    fn try_from(val: MessageEntity) -> Result<Self, Self::Error> {
        if let MessageEntity::Url(inner) = val {
            Ok(inner)
        } else {
            Err(Self::Error::new(
                stringify!(MessageEntity),
                stringify!(MessageEntityUrl),
            ))
        }
    }
}
impl From<crate::types::MessageEntityEmail> for MessageEntity {
    fn from(val: crate::types::MessageEntityEmail) -> Self {
        Self::Email(val)
    }
}
impl TryFrom<MessageEntity> for crate::types::MessageEntityEmail {
    type Error = crate::errors::ConvertToTypeError;

    fn try_from(val: MessageEntity) -> Result<Self, Self::Error> {
        if let MessageEntity::Email(inner) = val {
            Ok(inner)
        } else {
            Err(Self::Error::new(
                stringify!(MessageEntity),
                stringify!(MessageEntityEmail),
            ))
        }
    }
}
impl From<crate::types::MessageEntityPhoneNumber> for MessageEntity {
    fn from(val: crate::types::MessageEntityPhoneNumber) -> Self {
        Self::PhoneNumber(val)
    }
}
impl TryFrom<MessageEntity> for crate::types::MessageEntityPhoneNumber {
    type Error = crate::errors::ConvertToTypeError;

    fn try_from(val: MessageEntity) -> Result<Self, Self::Error> {
        if let MessageEntity::PhoneNumber(inner) = val {
            Ok(inner)
        } else {
            Err(Self::Error::new(
                stringify!(MessageEntity),
                stringify!(MessageEntityPhoneNumber),
            ))
        }
    }
}
impl From<crate::types::MessageEntityBold> for MessageEntity {
    fn from(val: crate::types::MessageEntityBold) -> Self {
        Self::Bold(val)
    }
}
impl TryFrom<MessageEntity> for crate::types::MessageEntityBold {
    type Error = crate::errors::ConvertToTypeError;

    fn try_from(val: MessageEntity) -> Result<Self, Self::Error> {
        if let MessageEntity::Bold(inner) = val {
            Ok(inner)
        } else {
            Err(Self::Error::new(
                stringify!(MessageEntity),
                stringify!(MessageEntityBold),
            ))
        }
    }
}
impl From<crate::types::MessageEntityItalic> for MessageEntity {
    fn from(val: crate::types::MessageEntityItalic) -> Self {
        Self::Italic(val)
    }
}
impl TryFrom<MessageEntity> for crate::types::MessageEntityItalic {
    type Error = crate::errors::ConvertToTypeError;

    fn try_from(val: MessageEntity) -> Result<Self, Self::Error> {
        if let MessageEntity::Italic(inner) = val {
            Ok(inner)
        } else {
            Err(Self::Error::new(
                stringify!(MessageEntity),
                stringify!(MessageEntityItalic),
            ))
        }
    }
}
impl From<crate::types::MessageEntityUnderline> for MessageEntity {
    fn from(val: crate::types::MessageEntityUnderline) -> Self {
        Self::Underline(val)
    }
}
impl TryFrom<MessageEntity> for crate::types::MessageEntityUnderline {
    type Error = crate::errors::ConvertToTypeError;

    fn try_from(val: MessageEntity) -> Result<Self, Self::Error> {
        if let MessageEntity::Underline(inner) = val {
            Ok(inner)
        } else {
            Err(Self::Error::new(
                stringify!(MessageEntity),
                stringify!(MessageEntityUnderline),
            ))
        }
    }
}
impl From<crate::types::MessageEntityStrikethrough> for MessageEntity {
    fn from(val: crate::types::MessageEntityStrikethrough) -> Self {
        Self::Strikethrough(val)
    }
}
impl TryFrom<MessageEntity> for crate::types::MessageEntityStrikethrough {
    type Error = crate::errors::ConvertToTypeError;

    fn try_from(val: MessageEntity) -> Result<Self, Self::Error> {
        if let MessageEntity::Strikethrough(inner) = val {
            Ok(inner)
        } else {
            Err(Self::Error::new(
                stringify!(MessageEntity),
                stringify!(MessageEntityStrikethrough),
            ))
        }
    }
}
impl From<crate::types::MessageEntitySpoiler> for MessageEntity {
    fn from(val: crate::types::MessageEntitySpoiler) -> Self {
        Self::Spoiler(val)
    }
}
impl TryFrom<MessageEntity> for crate::types::MessageEntitySpoiler {
    type Error = crate::errors::ConvertToTypeError;

    fn try_from(val: MessageEntity) -> Result<Self, Self::Error> {
        if let MessageEntity::Spoiler(inner) = val {
            Ok(inner)
        } else {
            Err(Self::Error::new(
                stringify!(MessageEntity),
                stringify!(MessageEntitySpoiler),
            ))
        }
    }
}
impl From<crate::types::MessageEntityBlockquote> for MessageEntity {
    fn from(val: crate::types::MessageEntityBlockquote) -> Self {
        Self::Blockquote(val)
    }
}
impl TryFrom<MessageEntity> for crate::types::MessageEntityBlockquote {
    type Error = crate::errors::ConvertToTypeError;

    fn try_from(val: MessageEntity) -> Result<Self, Self::Error> {
        if let MessageEntity::Blockquote(inner) = val {
            Ok(inner)
        } else {
            Err(Self::Error::new(
                stringify!(MessageEntity),
                stringify!(MessageEntityBlockquote),
            ))
        }
    }
}
impl From<crate::types::MessageEntityExpandableBlockquote> for MessageEntity {
    fn from(val: crate::types::MessageEntityExpandableBlockquote) -> Self {
        Self::ExpandableBlockquote(val)
    }
}
impl TryFrom<MessageEntity> for crate::types::MessageEntityExpandableBlockquote {
    type Error = crate::errors::ConvertToTypeError;

    fn try_from(val: MessageEntity) -> Result<Self, Self::Error> {
        if let MessageEntity::ExpandableBlockquote(inner) = val {
            Ok(inner)
        } else {
            Err(Self::Error::new(
                stringify!(MessageEntity),
                stringify!(MessageEntityExpandableBlockquote),
            ))
        }
    }
}
impl From<crate::types::MessageEntityCode> for MessageEntity {
    fn from(val: crate::types::MessageEntityCode) -> Self {
        Self::Code(val)
    }
}
impl TryFrom<MessageEntity> for crate::types::MessageEntityCode {
    type Error = crate::errors::ConvertToTypeError;

    fn try_from(val: MessageEntity) -> Result<Self, Self::Error> {
        if let MessageEntity::Code(inner) = val {
            Ok(inner)
        } else {
            Err(Self::Error::new(
                stringify!(MessageEntity),
                stringify!(MessageEntityCode),
            ))
        }
    }
}
impl From<crate::types::MessageEntityPre> for MessageEntity {
    fn from(val: crate::types::MessageEntityPre) -> Self {
        Self::Pre(val)
    }
}
impl TryFrom<MessageEntity> for crate::types::MessageEntityPre {
    type Error = crate::errors::ConvertToTypeError;

    fn try_from(val: MessageEntity) -> Result<Self, Self::Error> {
        if let MessageEntity::Pre(inner) = val {
            Ok(inner)
        } else {
            Err(Self::Error::new(
                stringify!(MessageEntity),
                stringify!(MessageEntityPre),
            ))
        }
    }
}
impl From<crate::types::MessageEntityTextLink> for MessageEntity {
    fn from(val: crate::types::MessageEntityTextLink) -> Self {
        Self::TextLink(val)
    }
}
impl TryFrom<MessageEntity> for crate::types::MessageEntityTextLink {
    type Error = crate::errors::ConvertToTypeError;

    fn try_from(val: MessageEntity) -> Result<Self, Self::Error> {
        if let MessageEntity::TextLink(inner) = val {
            Ok(inner)
        } else {
            Err(Self::Error::new(
                stringify!(MessageEntity),
                stringify!(MessageEntityTextLink),
            ))
        }
    }
}
impl From<crate::types::MessageEntityTextMention> for MessageEntity {
    fn from(val: crate::types::MessageEntityTextMention) -> Self {
        Self::TextMention(val)
    }
}
impl TryFrom<MessageEntity> for crate::types::MessageEntityTextMention {
    type Error = crate::errors::ConvertToTypeError;

    fn try_from(val: MessageEntity) -> Result<Self, Self::Error> {
        if let MessageEntity::TextMention(inner) = val {
            Ok(inner)
        } else {
            Err(Self::Error::new(
                stringify!(MessageEntity),
                stringify!(MessageEntityTextMention),
            ))
        }
    }
}
impl From<crate::types::MessageEntityCustomEmoji> for MessageEntity {
    fn from(val: crate::types::MessageEntityCustomEmoji) -> Self {
        Self::CustomEmoji(val)
    }
}
impl TryFrom<MessageEntity> for crate::types::MessageEntityCustomEmoji {
    type Error = crate::errors::ConvertToTypeError;

    fn try_from(val: MessageEntity) -> Result<Self, Self::Error> {
        if let MessageEntity::CustomEmoji(inner) = val {
            Ok(inner)
        } else {
            Err(Self::Error::new(
                stringify!(MessageEntity),
                stringify!(MessageEntityCustomEmoji),
            ))
        }
    }
}
impl From<crate::types::MessageEntityDateTime> for MessageEntity {
    fn from(val: crate::types::MessageEntityDateTime) -> Self {
        Self::DateTime(val)
    }
}
impl TryFrom<MessageEntity> for crate::types::MessageEntityDateTime {
    type Error = crate::errors::ConvertToTypeError;

    fn try_from(val: MessageEntity) -> Result<Self, Self::Error> {
        if let MessageEntity::DateTime(inner) = val {
            Ok(inner)
        } else {
            Err(Self::Error::new(
                stringify!(MessageEntity),
                stringify!(MessageEntityDateTime),
            ))
        }
    }
}
