use crate::types::RichText;
use serde::{Deserialize, Serialize};
use strum_macros::{AsRefStr, Display, EnumString, IntoStaticStr};
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
pub enum RichTextType {
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
    #[strum(serialize = "date_time")]
    DateTime,
    #[strum(serialize = "text_mention")]
    TextMention,
    #[strum(serialize = "subscript")]
    Subscript,
    #[strum(serialize = "superscript")]
    Superscript,
    #[strum(serialize = "marked")]
    Marked,
    #[strum(serialize = "code")]
    Code,
    #[strum(serialize = "custom_emoji")]
    CustomEmoji,
    #[strum(serialize = "mathematical_expression")]
    MathematicalExpression,
    #[strum(serialize = "url")]
    Url,
    #[strum(serialize = "email_address")]
    EmailAddress,
    #[strum(serialize = "phone_number")]
    PhoneNumber,
    #[strum(serialize = "bank_card_number")]
    BankCardNumber,
    #[strum(serialize = "mention")]
    Mention,
    #[strum(serialize = "hashtag")]
    Hashtag,
    #[strum(serialize = "cashtag")]
    Cashtag,
    #[strum(serialize = "bot_command")]
    BotCommand,
    #[strum(serialize = "anchor")]
    Anchor,
    #[strum(serialize = "anchor_link")]
    AnchorLink,
    #[strum(serialize = "reference")]
    Reference,
    #[strum(serialize = "reference_link")]
    ReferenceLink,
}
impl RichTextType {
    #[must_use]
    pub const fn all() -> [RichTextType; 25usize] {
        [
            RichTextType::Bold,
            RichTextType::Italic,
            RichTextType::Underline,
            RichTextType::Strikethrough,
            RichTextType::Spoiler,
            RichTextType::DateTime,
            RichTextType::TextMention,
            RichTextType::Subscript,
            RichTextType::Superscript,
            RichTextType::Marked,
            RichTextType::Code,
            RichTextType::CustomEmoji,
            RichTextType::MathematicalExpression,
            RichTextType::Url,
            RichTextType::EmailAddress,
            RichTextType::PhoneNumber,
            RichTextType::BankCardNumber,
            RichTextType::Mention,
            RichTextType::Hashtag,
            RichTextType::Cashtag,
            RichTextType::BotCommand,
            RichTextType::Anchor,
            RichTextType::AnchorLink,
            RichTextType::Reference,
            RichTextType::ReferenceLink,
        ]
    }
}
impl From<RichTextType> for Box<str> {
    fn from(val: RichTextType) -> Self {
        Into::<&'static str>::into(val).into()
    }
}
impl From<RichTextType> for String {
    fn from(val: RichTextType) -> Self {
        val.as_ref().to_owned()
    }
}
impl<'a> PartialEq<&'a str> for RichTextType {
    fn eq(&self, other: &&'a str) -> bool {
        self.as_ref() == *other
    }
}
impl<'a> From<&'a RichText> for RichTextType {
    fn from(val: &'a RichText) -> Self {
        match val {
            RichText::Bold(_) => RichTextType::Bold,
            RichText::Italic(_) => RichTextType::Italic,
            RichText::Underline(_) => RichTextType::Underline,
            RichText::Strikethrough(_) => RichTextType::Strikethrough,
            RichText::Spoiler(_) => RichTextType::Spoiler,
            RichText::DateTime(_) => RichTextType::DateTime,
            RichText::TextMention(_) => RichTextType::TextMention,
            RichText::Subscript(_) => RichTextType::Subscript,
            RichText::Superscript(_) => RichTextType::Superscript,
            RichText::Marked(_) => RichTextType::Marked,
            RichText::Code(_) => RichTextType::Code,
            RichText::CustomEmoji(_) => RichTextType::CustomEmoji,
            RichText::MathematicalExpression(_) => RichTextType::MathematicalExpression,
            RichText::Url(_) => RichTextType::Url,
            RichText::EmailAddress(_) => RichTextType::EmailAddress,
            RichText::PhoneNumber(_) => RichTextType::PhoneNumber,
            RichText::BankCardNumber(_) => RichTextType::BankCardNumber,
            RichText::Mention(_) => RichTextType::Mention,
            RichText::Hashtag(_) => RichTextType::Hashtag,
            RichText::Cashtag(_) => RichTextType::Cashtag,
            RichText::BotCommand(_) => RichTextType::BotCommand,
            RichText::Anchor(_) => RichTextType::Anchor,
            RichText::AnchorLink(_) => RichTextType::AnchorLink,
            RichText::Reference(_) => RichTextType::Reference,
            RichText::ReferenceLink(_) => RichTextType::ReferenceLink,
        }
    }
}
impl From<RichText> for RichTextType {
    fn from(val: RichText) -> Self {
        RichTextType::from(&val)
    }
}
