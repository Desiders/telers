use serde::{Deserialize, Serialize};
/// This object represents a rich formatted text. Currently, it can be either a String for plain text, an Array of [`crate::types::RichText`], or any of the following types:
/// - [`crate::types::RichTextBold`]
/// - [`crate::types::RichTextItalic`]
/// - [`crate::types::RichTextUnderline`]
/// - [`crate::types::RichTextStrikethrough`]
/// - [`crate::types::RichTextSpoiler`]
/// - [`crate::types::RichTextDateTime`]
/// - [`crate::types::RichTextTextMention`]
/// - [`crate::types::RichTextSubscript`]
/// - [`crate::types::RichTextSuperscript`]
/// - [`crate::types::RichTextMarked`]
/// - [`crate::types::RichTextCode`]
/// - [`crate::types::RichTextCustomEmoji`]
/// - [`crate::types::RichTextMathematicalExpression`]
/// - [`crate::types::RichTextUrl`]
/// - [`crate::types::RichTextEmailAddress`]
/// - [`crate::types::RichTextPhoneNumber`]
/// - [`crate::types::RichTextBankCardNumber`]
/// - [`crate::types::RichTextMention`]
/// - [`crate::types::RichTextHashtag`]
/// - [`crate::types::RichTextCashtag`]
/// - [`crate::types::RichTextBotCommand`]
/// - [`crate::types::RichTextAnchor`]
/// - [`crate::types::RichTextAnchorLink`]
/// - [`crate::types::RichTextReference`]
/// - [`crate::types::RichTextReferenceLink`]
/// # Documentation
/// <https://core.telegram.org/bots/api#richtext>
#[derive(Clone, Debug, Serialize, Deserialize)]
#[serde(tag = "type", rename_all = "snake_case")]
pub enum RichText {
    Bold(crate::types::RichTextBold),
    Italic(crate::types::RichTextItalic),
    Underline(crate::types::RichTextUnderline),
    Strikethrough(crate::types::RichTextStrikethrough),
    Spoiler(crate::types::RichTextSpoiler),
    DateTime(crate::types::RichTextDateTime),
    TextMention(crate::types::RichTextTextMention),
    Subscript(crate::types::RichTextSubscript),
    Superscript(crate::types::RichTextSuperscript),
    Marked(crate::types::RichTextMarked),
    Code(crate::types::RichTextCode),
    CustomEmoji(crate::types::RichTextCustomEmoji),
    MathematicalExpression(crate::types::RichTextMathematicalExpression),
    Url(crate::types::RichTextUrl),
    EmailAddress(crate::types::RichTextEmailAddress),
    PhoneNumber(crate::types::RichTextPhoneNumber),
    BankCardNumber(crate::types::RichTextBankCardNumber),
    Mention(crate::types::RichTextMention),
    Hashtag(crate::types::RichTextHashtag),
    Cashtag(crate::types::RichTextCashtag),
    BotCommand(crate::types::RichTextBotCommand),
    Anchor(crate::types::RichTextAnchor),
    AnchorLink(crate::types::RichTextAnchorLink),
    Reference(crate::types::RichTextReference),
    ReferenceLink(crate::types::RichTextReferenceLink),
    /// Content unknown to this version of the library
    #[serde(untagged)]
    Unknown(crate::types::RichTextUnknown),
    /// Plain text
    #[serde(untagged)]
    Plain(Box<str>),
    /// Multiple parts concatenated together
    #[serde(untagged)]
    Multiple(Box<[crate::types::RichText]>),
}
impl RichText {
    /// Helper method for field `alternative_text`.
    ///
    /// Alternative emoji for the custom emoji
    #[must_use]
    pub fn alternative_text(&self) -> Option<&str> {
        match self {
            Self::CustomEmoji(val) => Some(val.alternative_text.as_ref()),
            _ => None,
        }
    }

    /// Helper method for field `anchor_name`.
    ///
    /// The name of the anchor. If the name is empty, then the link brings back to the top of the message.
    #[must_use]
    pub fn anchor_name(&self) -> Option<&str> {
        match self {
            Self::AnchorLink(val) => Some(val.anchor_name.as_ref()),
            _ => None,
        }
    }

    /// Helper method for field `bank_card_number`.
    ///
    /// The bank card number
    #[must_use]
    pub fn bank_card_number(&self) -> Option<&str> {
        match self {
            Self::BankCardNumber(val) => Some(val.bank_card_number.as_ref()),
            _ => None,
        }
    }

    /// Helper method for field `bot_command`.
    ///
    /// The bot command
    #[must_use]
    pub fn bot_command(&self) -> Option<&str> {
        match self {
            Self::BotCommand(val) => Some(val.bot_command.as_ref()),
            _ => None,
        }
    }

    /// Helper method for field `cashtag`.
    ///
    /// The cashtag
    #[must_use]
    pub fn cashtag(&self) -> Option<&str> {
        match self {
            Self::Cashtag(val) => Some(val.cashtag.as_ref()),
            _ => None,
        }
    }

    /// Helper method for field `custom_emoji_id`.
    ///
    /// Unique identifier of the custom emoji. Use [`crate::methods::GetCustomEmojiStickers`] to get full information about the sticker.
    #[must_use]
    pub fn custom_emoji_id(&self) -> Option<&str> {
        match self {
            Self::CustomEmoji(val) => Some(val.custom_emoji_id.as_ref()),
            _ => None,
        }
    }

    /// Helper method for field `date_time_format`.
    ///
    /// The string that defines the formatting of the date and time. See date-time entity formatting for more details.
    #[must_use]
    pub fn date_time_format(&self) -> Option<&str> {
        match self {
            Self::DateTime(val) => Some(val.date_time_format.as_ref()),
            _ => None,
        }
    }

    /// Helper method for field `email_address`.
    ///
    /// The email address
    #[must_use]
    pub fn email_address(&self) -> Option<&str> {
        match self {
            Self::EmailAddress(val) => Some(val.email_address.as_ref()),
            _ => None,
        }
    }

    /// Helper method for field `expression`.
    ///
    /// The expression in `LaTeX` format
    #[must_use]
    pub fn expression(&self) -> Option<&str> {
        match self {
            Self::MathematicalExpression(val) => Some(val.expression.as_ref()),
            _ => None,
        }
    }

    /// Helper method for field `hashtag`.
    ///
    /// The hashtag
    #[must_use]
    pub fn hashtag(&self) -> Option<&str> {
        match self {
            Self::Hashtag(val) => Some(val.hashtag.as_ref()),
            _ => None,
        }
    }

    /// Helper method for field `name`.
    ///
    /// # Variants
    /// - `RichTextAnchor`. The name of the anchor
    /// - `RichTextReference`. The name of the reference
    #[must_use]
    pub fn name(&self) -> Option<&str> {
        match self {
            Self::Anchor(val) => Some(val.name.as_ref()),
            Self::Reference(val) => Some(val.name.as_ref()),
            _ => None,
        }
    }

    /// Helper method for field `phone_number`.
    ///
    /// The phone number
    #[must_use]
    pub fn phone_number(&self) -> Option<&str> {
        match self {
            Self::PhoneNumber(val) => Some(val.phone_number.as_ref()),
            _ => None,
        }
    }

    /// Helper method for field `reference_name`.
    ///
    /// The name of the reference
    #[must_use]
    pub fn reference_name(&self) -> Option<&str> {
        match self {
            Self::ReferenceLink(val) => Some(val.reference_name.as_ref()),
            _ => None,
        }
    }

    /// Helper method for field `text`.
    ///
    /// # Variants
    /// - `RichTextBold`, `RichTextItalic`, `RichTextUnderline`, `RichTextStrikethrough`, `RichTextSpoiler`, `RichTextDateTime`, `RichTextTextMention`, `RichTextSubscript`, `RichTextSuperscript`, `RichTextMarked`, `RichTextCode`, `RichTextUrl`, `RichTextEmailAddress`, `RichTextPhoneNumber`, `RichTextBankCardNumber`, `RichTextMention`, `RichTextHashtag`, `RichTextCashtag`, `RichTextBotCommand`. The text
    /// - `RichTextAnchorLink`, `RichTextReferenceLink`. The link text
    /// - `RichTextReference`. Text of the reference
    #[must_use]
    pub fn text(&self) -> Option<&crate::types::RichText> {
        match self {
            Self::Bold(val) => Some(val.text.as_ref()),
            Self::Italic(val) => Some(val.text.as_ref()),
            Self::Underline(val) => Some(val.text.as_ref()),
            Self::Strikethrough(val) => Some(val.text.as_ref()),
            Self::Spoiler(val) => Some(val.text.as_ref()),
            Self::DateTime(val) => Some(val.text.as_ref()),
            Self::TextMention(val) => Some(val.text.as_ref()),
            Self::Subscript(val) => Some(val.text.as_ref()),
            Self::Superscript(val) => Some(val.text.as_ref()),
            Self::Marked(val) => Some(val.text.as_ref()),
            Self::Code(val) => Some(val.text.as_ref()),
            Self::Url(val) => Some(val.text.as_ref()),
            Self::EmailAddress(val) => Some(val.text.as_ref()),
            Self::PhoneNumber(val) => Some(val.text.as_ref()),
            Self::BankCardNumber(val) => Some(val.text.as_ref()),
            Self::Mention(val) => Some(val.text.as_ref()),
            Self::Hashtag(val) => Some(val.text.as_ref()),
            Self::Cashtag(val) => Some(val.text.as_ref()),
            Self::BotCommand(val) => Some(val.text.as_ref()),
            Self::AnchorLink(val) => Some(val.text.as_ref()),
            Self::Reference(val) => Some(val.text.as_ref()),
            Self::ReferenceLink(val) => Some(val.text.as_ref()),
            _ => None,
        }
    }

    /// Helper method for field `unix_time`.
    ///
    /// The Unix time associated with the entity
    #[must_use]
    pub fn unix_time(&self) -> Option<i64> {
        match self {
            Self::DateTime(val) => Some(val.unix_time),
            _ => None,
        }
    }

    /// Helper method for field `url`.
    ///
    /// URL of the link
    #[must_use]
    pub fn url(&self) -> Option<&str> {
        match self {
            Self::Url(val) => Some(val.url.as_ref()),
            _ => None,
        }
    }

    /// Helper method for field `user`.
    ///
    /// The mentioned user
    #[must_use]
    pub fn user(&self) -> Option<&crate::types::User> {
        match self {
            Self::TextMention(val) => Some(val.user.as_ref()),
            _ => None,
        }
    }

    /// Helper method for field `username`.
    ///
    /// The username
    #[must_use]
    pub fn username(&self) -> Option<&str> {
        match self {
            Self::Mention(val) => Some(val.username.as_ref()),
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

    /// Helper method for nested field `can_manage_bots`.
    #[must_use]
    pub fn can_manage_bots(&self) -> Option<bool> {
        match self {
            Self::TextMention(val) => {
                let inner = val.user.as_ref();
                inner.can_manage_bots
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

    /// Helper method for nested field `supports_guest_queries`.
    #[must_use]
    pub fn supports_guest_queries(&self) -> Option<bool> {
        match self {
            Self::TextMention(val) => {
                let inner = val.user.as_ref();
                inner.supports_guest_queries
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

    /// Helper method for nested field `supports_join_request_queries`.
    #[must_use]
    pub fn supports_join_request_queries(&self) -> Option<bool> {
        match self {
            Self::TextMention(val) => {
                let inner = val.user.as_ref();
                inner.supports_join_request_queries
            }
            _ => None,
        }
    }
}
impl From<Box<str>> for RichText {
    fn from(val: Box<str>) -> Self {
        Self::Plain(val)
    }
}
impl From<String> for RichText {
    fn from(val: String) -> Self {
        Self::Plain(val.into())
    }
}
impl From<&str> for RichText {
    fn from(val: &str) -> Self {
        Self::Plain(val.into())
    }
}
impl From<Box<[RichText]>> for RichText {
    fn from(val: Box<[RichText]>) -> Self {
        Self::Multiple(val)
    }
}
impl From<Vec<RichText>> for RichText {
    fn from(val: Vec<RichText>) -> Self {
        Self::Multiple(val.into())
    }
}
impl From<crate::types::RichTextBold> for RichText {
    fn from(val: crate::types::RichTextBold) -> Self {
        Self::Bold(val)
    }
}
impl TryFrom<RichText> for crate::types::RichTextBold {
    type Error = crate::errors::ConvertToTypeError;

    fn try_from(val: RichText) -> Result<Self, Self::Error> {
        if let RichText::Bold(inner) = val {
            Ok(inner)
        } else {
            Err(Self::Error::new(
                stringify!(RichText),
                stringify!(RichTextBold),
            ))
        }
    }
}
impl From<crate::types::RichTextItalic> for RichText {
    fn from(val: crate::types::RichTextItalic) -> Self {
        Self::Italic(val)
    }
}
impl TryFrom<RichText> for crate::types::RichTextItalic {
    type Error = crate::errors::ConvertToTypeError;

    fn try_from(val: RichText) -> Result<Self, Self::Error> {
        if let RichText::Italic(inner) = val {
            Ok(inner)
        } else {
            Err(Self::Error::new(
                stringify!(RichText),
                stringify!(RichTextItalic),
            ))
        }
    }
}
impl From<crate::types::RichTextUnderline> for RichText {
    fn from(val: crate::types::RichTextUnderline) -> Self {
        Self::Underline(val)
    }
}
impl TryFrom<RichText> for crate::types::RichTextUnderline {
    type Error = crate::errors::ConvertToTypeError;

    fn try_from(val: RichText) -> Result<Self, Self::Error> {
        if let RichText::Underline(inner) = val {
            Ok(inner)
        } else {
            Err(Self::Error::new(
                stringify!(RichText),
                stringify!(RichTextUnderline),
            ))
        }
    }
}
impl From<crate::types::RichTextStrikethrough> for RichText {
    fn from(val: crate::types::RichTextStrikethrough) -> Self {
        Self::Strikethrough(val)
    }
}
impl TryFrom<RichText> for crate::types::RichTextStrikethrough {
    type Error = crate::errors::ConvertToTypeError;

    fn try_from(val: RichText) -> Result<Self, Self::Error> {
        if let RichText::Strikethrough(inner) = val {
            Ok(inner)
        } else {
            Err(Self::Error::new(
                stringify!(RichText),
                stringify!(RichTextStrikethrough),
            ))
        }
    }
}
impl From<crate::types::RichTextSpoiler> for RichText {
    fn from(val: crate::types::RichTextSpoiler) -> Self {
        Self::Spoiler(val)
    }
}
impl TryFrom<RichText> for crate::types::RichTextSpoiler {
    type Error = crate::errors::ConvertToTypeError;

    fn try_from(val: RichText) -> Result<Self, Self::Error> {
        if let RichText::Spoiler(inner) = val {
            Ok(inner)
        } else {
            Err(Self::Error::new(
                stringify!(RichText),
                stringify!(RichTextSpoiler),
            ))
        }
    }
}
impl From<crate::types::RichTextDateTime> for RichText {
    fn from(val: crate::types::RichTextDateTime) -> Self {
        Self::DateTime(val)
    }
}
impl TryFrom<RichText> for crate::types::RichTextDateTime {
    type Error = crate::errors::ConvertToTypeError;

    fn try_from(val: RichText) -> Result<Self, Self::Error> {
        if let RichText::DateTime(inner) = val {
            Ok(inner)
        } else {
            Err(Self::Error::new(
                stringify!(RichText),
                stringify!(RichTextDateTime),
            ))
        }
    }
}
impl From<crate::types::RichTextTextMention> for RichText {
    fn from(val: crate::types::RichTextTextMention) -> Self {
        Self::TextMention(val)
    }
}
impl TryFrom<RichText> for crate::types::RichTextTextMention {
    type Error = crate::errors::ConvertToTypeError;

    fn try_from(val: RichText) -> Result<Self, Self::Error> {
        if let RichText::TextMention(inner) = val {
            Ok(inner)
        } else {
            Err(Self::Error::new(
                stringify!(RichText),
                stringify!(RichTextTextMention),
            ))
        }
    }
}
impl From<crate::types::RichTextSubscript> for RichText {
    fn from(val: crate::types::RichTextSubscript) -> Self {
        Self::Subscript(val)
    }
}
impl TryFrom<RichText> for crate::types::RichTextSubscript {
    type Error = crate::errors::ConvertToTypeError;

    fn try_from(val: RichText) -> Result<Self, Self::Error> {
        if let RichText::Subscript(inner) = val {
            Ok(inner)
        } else {
            Err(Self::Error::new(
                stringify!(RichText),
                stringify!(RichTextSubscript),
            ))
        }
    }
}
impl From<crate::types::RichTextSuperscript> for RichText {
    fn from(val: crate::types::RichTextSuperscript) -> Self {
        Self::Superscript(val)
    }
}
impl TryFrom<RichText> for crate::types::RichTextSuperscript {
    type Error = crate::errors::ConvertToTypeError;

    fn try_from(val: RichText) -> Result<Self, Self::Error> {
        if let RichText::Superscript(inner) = val {
            Ok(inner)
        } else {
            Err(Self::Error::new(
                stringify!(RichText),
                stringify!(RichTextSuperscript),
            ))
        }
    }
}
impl From<crate::types::RichTextMarked> for RichText {
    fn from(val: crate::types::RichTextMarked) -> Self {
        Self::Marked(val)
    }
}
impl TryFrom<RichText> for crate::types::RichTextMarked {
    type Error = crate::errors::ConvertToTypeError;

    fn try_from(val: RichText) -> Result<Self, Self::Error> {
        if let RichText::Marked(inner) = val {
            Ok(inner)
        } else {
            Err(Self::Error::new(
                stringify!(RichText),
                stringify!(RichTextMarked),
            ))
        }
    }
}
impl From<crate::types::RichTextCode> for RichText {
    fn from(val: crate::types::RichTextCode) -> Self {
        Self::Code(val)
    }
}
impl TryFrom<RichText> for crate::types::RichTextCode {
    type Error = crate::errors::ConvertToTypeError;

    fn try_from(val: RichText) -> Result<Self, Self::Error> {
        if let RichText::Code(inner) = val {
            Ok(inner)
        } else {
            Err(Self::Error::new(
                stringify!(RichText),
                stringify!(RichTextCode),
            ))
        }
    }
}
impl From<crate::types::RichTextCustomEmoji> for RichText {
    fn from(val: crate::types::RichTextCustomEmoji) -> Self {
        Self::CustomEmoji(val)
    }
}
impl TryFrom<RichText> for crate::types::RichTextCustomEmoji {
    type Error = crate::errors::ConvertToTypeError;

    fn try_from(val: RichText) -> Result<Self, Self::Error> {
        if let RichText::CustomEmoji(inner) = val {
            Ok(inner)
        } else {
            Err(Self::Error::new(
                stringify!(RichText),
                stringify!(RichTextCustomEmoji),
            ))
        }
    }
}
impl From<crate::types::RichTextMathematicalExpression> for RichText {
    fn from(val: crate::types::RichTextMathematicalExpression) -> Self {
        Self::MathematicalExpression(val)
    }
}
impl TryFrom<RichText> for crate::types::RichTextMathematicalExpression {
    type Error = crate::errors::ConvertToTypeError;

    fn try_from(val: RichText) -> Result<Self, Self::Error> {
        if let RichText::MathematicalExpression(inner) = val {
            Ok(inner)
        } else {
            Err(Self::Error::new(
                stringify!(RichText),
                stringify!(RichTextMathematicalExpression),
            ))
        }
    }
}
impl From<crate::types::RichTextUrl> for RichText {
    fn from(val: crate::types::RichTextUrl) -> Self {
        Self::Url(val)
    }
}
impl TryFrom<RichText> for crate::types::RichTextUrl {
    type Error = crate::errors::ConvertToTypeError;

    fn try_from(val: RichText) -> Result<Self, Self::Error> {
        if let RichText::Url(inner) = val {
            Ok(inner)
        } else {
            Err(Self::Error::new(
                stringify!(RichText),
                stringify!(RichTextUrl),
            ))
        }
    }
}
impl From<crate::types::RichTextEmailAddress> for RichText {
    fn from(val: crate::types::RichTextEmailAddress) -> Self {
        Self::EmailAddress(val)
    }
}
impl TryFrom<RichText> for crate::types::RichTextEmailAddress {
    type Error = crate::errors::ConvertToTypeError;

    fn try_from(val: RichText) -> Result<Self, Self::Error> {
        if let RichText::EmailAddress(inner) = val {
            Ok(inner)
        } else {
            Err(Self::Error::new(
                stringify!(RichText),
                stringify!(RichTextEmailAddress),
            ))
        }
    }
}
impl From<crate::types::RichTextPhoneNumber> for RichText {
    fn from(val: crate::types::RichTextPhoneNumber) -> Self {
        Self::PhoneNumber(val)
    }
}
impl TryFrom<RichText> for crate::types::RichTextPhoneNumber {
    type Error = crate::errors::ConvertToTypeError;

    fn try_from(val: RichText) -> Result<Self, Self::Error> {
        if let RichText::PhoneNumber(inner) = val {
            Ok(inner)
        } else {
            Err(Self::Error::new(
                stringify!(RichText),
                stringify!(RichTextPhoneNumber),
            ))
        }
    }
}
impl From<crate::types::RichTextBankCardNumber> for RichText {
    fn from(val: crate::types::RichTextBankCardNumber) -> Self {
        Self::BankCardNumber(val)
    }
}
impl TryFrom<RichText> for crate::types::RichTextBankCardNumber {
    type Error = crate::errors::ConvertToTypeError;

    fn try_from(val: RichText) -> Result<Self, Self::Error> {
        if let RichText::BankCardNumber(inner) = val {
            Ok(inner)
        } else {
            Err(Self::Error::new(
                stringify!(RichText),
                stringify!(RichTextBankCardNumber),
            ))
        }
    }
}
impl From<crate::types::RichTextMention> for RichText {
    fn from(val: crate::types::RichTextMention) -> Self {
        Self::Mention(val)
    }
}
impl TryFrom<RichText> for crate::types::RichTextMention {
    type Error = crate::errors::ConvertToTypeError;

    fn try_from(val: RichText) -> Result<Self, Self::Error> {
        if let RichText::Mention(inner) = val {
            Ok(inner)
        } else {
            Err(Self::Error::new(
                stringify!(RichText),
                stringify!(RichTextMention),
            ))
        }
    }
}
impl From<crate::types::RichTextHashtag> for RichText {
    fn from(val: crate::types::RichTextHashtag) -> Self {
        Self::Hashtag(val)
    }
}
impl TryFrom<RichText> for crate::types::RichTextHashtag {
    type Error = crate::errors::ConvertToTypeError;

    fn try_from(val: RichText) -> Result<Self, Self::Error> {
        if let RichText::Hashtag(inner) = val {
            Ok(inner)
        } else {
            Err(Self::Error::new(
                stringify!(RichText),
                stringify!(RichTextHashtag),
            ))
        }
    }
}
impl From<crate::types::RichTextCashtag> for RichText {
    fn from(val: crate::types::RichTextCashtag) -> Self {
        Self::Cashtag(val)
    }
}
impl TryFrom<RichText> for crate::types::RichTextCashtag {
    type Error = crate::errors::ConvertToTypeError;

    fn try_from(val: RichText) -> Result<Self, Self::Error> {
        if let RichText::Cashtag(inner) = val {
            Ok(inner)
        } else {
            Err(Self::Error::new(
                stringify!(RichText),
                stringify!(RichTextCashtag),
            ))
        }
    }
}
impl From<crate::types::RichTextBotCommand> for RichText {
    fn from(val: crate::types::RichTextBotCommand) -> Self {
        Self::BotCommand(val)
    }
}
impl TryFrom<RichText> for crate::types::RichTextBotCommand {
    type Error = crate::errors::ConvertToTypeError;

    fn try_from(val: RichText) -> Result<Self, Self::Error> {
        if let RichText::BotCommand(inner) = val {
            Ok(inner)
        } else {
            Err(Self::Error::new(
                stringify!(RichText),
                stringify!(RichTextBotCommand),
            ))
        }
    }
}
impl From<crate::types::RichTextAnchor> for RichText {
    fn from(val: crate::types::RichTextAnchor) -> Self {
        Self::Anchor(val)
    }
}
impl TryFrom<RichText> for crate::types::RichTextAnchor {
    type Error = crate::errors::ConvertToTypeError;

    fn try_from(val: RichText) -> Result<Self, Self::Error> {
        if let RichText::Anchor(inner) = val {
            Ok(inner)
        } else {
            Err(Self::Error::new(
                stringify!(RichText),
                stringify!(RichTextAnchor),
            ))
        }
    }
}
impl From<crate::types::RichTextAnchorLink> for RichText {
    fn from(val: crate::types::RichTextAnchorLink) -> Self {
        Self::AnchorLink(val)
    }
}
impl TryFrom<RichText> for crate::types::RichTextAnchorLink {
    type Error = crate::errors::ConvertToTypeError;

    fn try_from(val: RichText) -> Result<Self, Self::Error> {
        if let RichText::AnchorLink(inner) = val {
            Ok(inner)
        } else {
            Err(Self::Error::new(
                stringify!(RichText),
                stringify!(RichTextAnchorLink),
            ))
        }
    }
}
impl From<crate::types::RichTextReference> for RichText {
    fn from(val: crate::types::RichTextReference) -> Self {
        Self::Reference(val)
    }
}
impl TryFrom<RichText> for crate::types::RichTextReference {
    type Error = crate::errors::ConvertToTypeError;

    fn try_from(val: RichText) -> Result<Self, Self::Error> {
        if let RichText::Reference(inner) = val {
            Ok(inner)
        } else {
            Err(Self::Error::new(
                stringify!(RichText),
                stringify!(RichTextReference),
            ))
        }
    }
}
impl From<crate::types::RichTextReferenceLink> for RichText {
    fn from(val: crate::types::RichTextReferenceLink) -> Self {
        Self::ReferenceLink(val)
    }
}
impl TryFrom<RichText> for crate::types::RichTextReferenceLink {
    type Error = crate::errors::ConvertToTypeError;

    fn try_from(val: RichText) -> Result<Self, Self::Error> {
        if let RichText::ReferenceLink(inner) = val {
            Ok(inner)
        } else {
            Err(Self::Error::new(
                stringify!(RichText),
                stringify!(RichTextReferenceLink),
            ))
        }
    }
}
impl From<crate::types::RichTextUnknown> for RichText {
    fn from(val: crate::types::RichTextUnknown) -> Self {
        Self::Unknown(val)
    }
}
impl TryFrom<RichText> for crate::types::RichTextUnknown {
    type Error = crate::errors::ConvertToTypeError;

    fn try_from(val: RichText) -> Result<Self, Self::Error> {
        if let RichText::Unknown(inner) = val {
            Ok(inner)
        } else {
            Err(Self::Error::new(
                stringify!(RichText),
                stringify!(RichTextUnknown),
            ))
        }
    }
}
