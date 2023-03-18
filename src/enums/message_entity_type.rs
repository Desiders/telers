use std::fmt::{self, Debug};

pub enum MessageEntityType {
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
    Code,
    Pre,
    TextLink,
    TextMention,
    CustomEmoji,
}

impl Debug for MessageEntityType {
    fn fmt(&self, f: &mut fmt::Formatter<'_>) -> fmt::Result {
        write!(f, "{}", self.as_str())
    }
}

impl MessageEntityType {
    pub const fn as_str(&self) -> &'static str {
        match self {
            MessageEntityType::Mention => "mention",
            MessageEntityType::Hashtag => "hashtag",
            MessageEntityType::Cashtag => "cashtag",
            MessageEntityType::BotCommand => "bot_command",
            MessageEntityType::Url => "url",
            MessageEntityType::Email => "email",
            MessageEntityType::PhoneNumber => "phone_number",
            MessageEntityType::Bold => "bold",
            MessageEntityType::Italic => "italic",
            MessageEntityType::Underline => "underline",
            MessageEntityType::Strikethrough => "strikethrough",
            MessageEntityType::Code => "code",
            MessageEntityType::Pre => "pre",
            MessageEntityType::TextLink => "text_link",
            MessageEntityType::TextMention => "text_mention",
            MessageEntityType::CustomEmoji => "custom_emoji",
        }
    }

    pub const fn all() -> &'static [MessageEntityType; 16] {
        &[
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

impl From<MessageEntityType> for String {
    fn from(entity_type: MessageEntityType) -> Self {
        entity_type.as_str().to_string()
    }
}

impl<'a> From<&'a MessageEntityType> for String {
    fn from(entity_type: &'a MessageEntityType) -> Self {
        entity_type.as_str().to_string()
    }
}
