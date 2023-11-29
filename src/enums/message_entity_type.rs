use strum_macros::{AsRefStr, Display, EnumString, IntoStaticStr};

/// This enum represents all possible types of the message entity
/// # Documentation
/// <https://core.telegram.org/bots/api#messageentity>
#[derive(Debug, Display, Clone, Copy, PartialEq, Eq, Hash, EnumString, AsRefStr, IntoStaticStr)]
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
}

impl MessageEntityType {
    #[must_use]
    pub const fn all() -> [MessageEntityType; 16] {
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
            MessageEntityType::Code,
            MessageEntityType::Pre,
            MessageEntityType::TextLink,
            MessageEntityType::TextMention,
            MessageEntityType::CustomEmoji,
        ]
    }
}

impl From<MessageEntityType> for Box<str> {
    fn from(entity_type: MessageEntityType) -> Self {
        Into::<&'static str>::into(entity_type).into()
    }
}

impl<'a> PartialEq<&'a str> for MessageEntityType {
    fn eq(&self, other: &&'a str) -> bool {
        self.as_ref() == *other
    }
}
