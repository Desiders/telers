use std::fmt::Display;

use super::{Formatter as TextFormatter, FormatterErrorKind};

use crate::types::{
    CustomEmojiMessageEntity, MessageEntity, MessageEntityKind, PreMessageEntity,
    TextLinkMessageEntity, TextMentionMessageEntity, User,
};

use tracing::{event, Level};

const CHARS: [char; 18] = [
    '_', '*', '[', ']', '(', ')', '~', '`', '>', '#', '+', '-', '=', '|', '{', '}', '.', '!',
];

/// This is a legacy mode, retained for backward compatibility. To use this mode, pass `Markdown` in the `parse_mode` field.
/// # Documentation
/// <https://core.telegram.org/bots/api#markdown-style>
#[derive(Debug, Clone)]
pub struct Formatter;

impl Formatter {
    /// Create a new instance of [`Formatter`]
    #[must_use]
    pub const fn new() -> Self {
        Self {}
    }
}

impl Default for Formatter {
    fn default() -> Self {
        Self::new()
    }
}

impl TextFormatter for Formatter {
    fn bold<T>(&self, text: T) -> String
    where
        T: Display,
    {
        format!("*{text}*")
    }

    fn italic<T>(&self, text: T) -> String
    where
        T: Display,
    {
        format!("_\r{text}_\r")
    }

    fn underline<T>(&self, text: T) -> String
    where
        T: Display,
    {
        format!("__\r{text}__\r")
    }

    fn strikethrough<T>(&self, text: T) -> String
    where
        T: Display,
    {
        format!("~{text}~")
    }

    fn spoiler<T>(&self, text: T) -> String
    where
        T: Display,
    {
        format!("|{text}|")
    }

    fn blockquote<T>(&self, text: T) -> String
    where
        T: Display,
    {
        text.to_string()
            .lines()
            .map(|line| format!(">{line}"))
            .collect::<Vec<_>>()
            .join("\n")
    }

    fn expandable_blockquote<T>(&self, text: T) -> String
    where
        T: Display,
    {
        let mut text = self.blockquote(text);
        text.push_str("||");

        text
    }

    fn text_link<T, U>(&self, text: T, url: U) -> String
    where
        T: Display,
        U: Display,
    {
        format!("[{text}]({url})")
    }

    fn text_mention<T>(&self, text: T, user_id: i64) -> String
    where
        T: Display,
    {
        self.text_link(text, format!("tg://user?id={user_id}"))
    }

    fn custom_emoji<T, E>(&self, emoji: T, emoji_id: E) -> String
    where
        T: Display,
        E: Display,
    {
        format!(
            "!{}",
            self.text_link(emoji, format!("tg://emoji?id={emoji_id}"),)
        )
    }

    fn code<T>(&self, text: T) -> String
    where
        T: Display,
    {
        format!("`{text}`")
    }

    fn pre<T>(&self, text: T) -> String
    where
        T: Display,
    {
        format!("```\n{text}\n```")
    }

    fn pre_language<T, L>(&self, text: T, language: L) -> String
    where
        T: Display,
        L: Display,
    {
        format!("```{language}\n{text}\n```")
    }

    fn quote<T>(&self, text: T) -> String
    where
        T: Display,
    {
        let text = text.to_string();

        text.chars()
            .fold(String::with_capacity(text.len()), |mut string, ch| {
                if CHARS.contains(&ch) {
                    string.push('\\');
                }
                string.push(ch);
                string
            })
    }

    fn apply_entity<T>(&self, text: T, entity: &MessageEntity) -> Result<String, FormatterErrorKind>
    where
        T: Display,
    {
        let text = text.to_string();
        let text_len = text.len();

        if text_len == 0 {
            return Err(FormatterErrorKind::EmptyText);
        }

        let offset = entity.offset as usize;
        let length = entity.length as usize;

        if offset + length > text_len {
            return Err(FormatterErrorKind::IndexOutOfBounds);
        }

        let previous_text = &text[..offset];
        let editable_text = &text[offset..offset + length];
        let next_text = &text[offset + length..];

        let edited_text = match entity.kind() {
            MessageEntityKind::Mention => format!("@{editable_text}"),
            MessageEntityKind::Hashtag => format!("#{editable_text}"),
            MessageEntityKind::Cashtag => format!("${editable_text}"),
            MessageEntityKind::BotCommand => format!("/{editable_text}"),
            MessageEntityKind::Url | MessageEntityKind::Email | MessageEntityKind::PhoneNumber => {
                editable_text.to_owned()
            }
            MessageEntityKind::Bold => self.bold(editable_text),
            MessageEntityKind::Italic => self.italic(editable_text),
            MessageEntityKind::Underline => self.underline(editable_text),
            MessageEntityKind::Strikethrough => self.strikethrough(editable_text),
            MessageEntityKind::Spoiler => self.spoiler(editable_text),
            MessageEntityKind::Blockquote => self.blockquote(editable_text),
            MessageEntityKind::ExpandableBlockquote => self.expandable_blockquote(editable_text),
            MessageEntityKind::Code => self.code(editable_text),
            MessageEntityKind::Pre(PreMessageEntity { language }) => match language {
                Some(language) => self.pre_language(editable_text, language),
                None => self.pre(editable_text),
            },
            MessageEntityKind::TextLink(TextLinkMessageEntity { url }) => {
                self.text_link(editable_text, url)
            }
            MessageEntityKind::TextMention(TextMentionMessageEntity {
                user: User { id: user_id, .. },
            }) => self.text_mention(editable_text, *user_id),
            MessageEntityKind::CustomEmoji(CustomEmojiMessageEntity { custom_emoji_id }) => {
                self.custom_emoji(editable_text, custom_emoji_id)
            }
            MessageEntityKind::Unknown => {
                event!(
                    Level::WARN,
                    "Unknown entity kind: {:?}. Using the original text.",
                    entity.kind()
                );

                editable_text.to_owned()
            }
        };

        Ok(format!("{previous_text}{edited_text}{next_text}"))
    }
}

pub const FORMATTER: Formatter = Formatter::new();

pub fn bold(text: impl Display) -> String {
    FORMATTER.bold(text)
}

pub fn italic(text: impl Display) -> String {
    FORMATTER.italic(text)
}

pub fn underline(text: impl Display) -> String {
    FORMATTER.underline(text)
}

pub fn strikethrough(text: impl Display) -> String {
    FORMATTER.strikethrough(text)
}

pub fn spoiler(text: impl Display) -> String {
    FORMATTER.spoiler(text)
}

pub fn blockquote(text: impl Display) -> String {
    FORMATTER.blockquote(text)
}

pub fn expandable_blockquote(text: impl Display) -> String {
    FORMATTER.expandable_blockquote(text)
}

pub fn text_link(text: impl Display, url: &str) -> String {
    FORMATTER.text_link(text, url)
}

pub fn text_mention(text: impl Display, user_id: i64) -> String {
    FORMATTER.text_mention(text, user_id)
}

pub fn custom_emoji(text: impl Display, emoji_id: &str) -> String {
    FORMATTER.custom_emoji(text, emoji_id)
}

pub fn code(text: impl Display) -> String {
    FORMATTER.code(text)
}

pub fn pre(text: impl Display) -> String {
    FORMATTER.pre(text)
}

pub fn pre_language(text: impl Display, language: &str) -> String {
    FORMATTER.pre_language(text, language)
}

pub fn quote(text: impl Display) -> String {
    FORMATTER.quote(text)
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_bold() {
        let formatter = Formatter;
        assert_eq!(formatter.bold("text"), "*text*");
    }

    #[test]
    fn test_italic() {
        let formatter = Formatter;
        assert_eq!(formatter.italic("text"), "_\rtext_\r");
    }

    #[test]
    fn test_underline() {
        let formatter = Formatter;
        assert_eq!(formatter.underline("text"), "__\rtext__\r");
    }

    #[test]
    fn test_strikethrough() {
        let formatter = Formatter;
        assert_eq!(formatter.strikethrough("text"), "~text~");
    }

    #[test]
    fn test_spoiler() {
        let formatter = Formatter;
        assert_eq!(formatter.spoiler("text"), "|text|");
    }

    #[test]
    fn test_blockquote() {
        let formatter = Formatter;
        assert_eq!(formatter.blockquote("text"), ">text");
        assert_eq!(formatter.blockquote("text\ntext"), ">text\n>text");
    }

    #[test]
    fn expandable_blockquote() {
        let formatter = Formatter;
        assert_eq!(formatter.expandable_blockquote("text"), ">text||");
        assert_eq!(
            formatter.expandable_blockquote("text\ntext"),
            ">text\n>text||"
        );
    }

    #[test]
    fn test_text_link() {
        let formatter = Formatter;
        assert_eq!(
            formatter.text_link("text", "http://example.com"),
            "[text](http://example.com)"
        );
    }

    #[test]
    fn test_text_mention() {
        let formatter = Formatter;
        assert_eq!(formatter.text_mention("text", 1), "[text](tg://user?id=1)");
    }

    #[test]
    fn test_custom_emoji() {
        let formatter = Formatter;
        assert_eq!(
            formatter.custom_emoji("text", "1"),
            "![text](tg://emoji?id=1)"
        );
    }

    #[test]
    fn test_code() {
        let formatter = Formatter;
        assert_eq!(formatter.code("text"), "`text`");
    }

    #[test]
    fn test_pre() {
        let formatter = Formatter;
        assert_eq!(formatter.pre("text"), "```\ntext\n```");
    }

    #[test]
    fn test_pre_language() {
        let formatter = Formatter;
        assert_eq!(
            formatter.pre_language("text", "python"),
            "```python\ntext\n```"
        );
    }

    #[test]
    fn test_quote() {
        let formatter = Formatter;
        assert_eq!(formatter.quote("test"), "test");
        assert_eq!(formatter.quote("[test]"), r"\[test\]");
        assert_eq!(formatter.quote("test ` test"), r"test \` test");
        assert_eq!(formatter.quote("test * test"), r"test \* test");
        assert_eq!(formatter.quote("test _ test"), r"test \_ test");
        assert_eq!(formatter.quote("test ~ test"), r"test \~ test");
        assert_eq!(formatter.quote("test | test"), r"test \| test");
        assert_eq!(formatter.quote("test > test"), r"test \> test");
        assert_eq!(formatter.quote("test # test"), r"test \# test");
        assert_eq!(formatter.quote("test + test"), r"test \+ test");
        assert_eq!(formatter.quote("test - test"), r"test \- test");
        assert_eq!(formatter.quote("test = test"), r"test \= test");
        assert_eq!(formatter.quote("test . test"), r"test \. test");
        assert_eq!(formatter.quote("test ! test"), r"test \! test");
        assert_eq!(formatter.quote("test [ test"), r"test \[ test");
        assert_eq!(formatter.quote("test ] test"), r"test \] test");
        assert_eq!(formatter.quote("test ( test"), r"test \( test");
        assert_eq!(formatter.quote("test ) test"), r"test \) test");
        assert_eq!(formatter.quote("test { test"), r"test \{ test");
        assert_eq!(formatter.quote("test } test"), r"test \} test");

        // Test for all symbols (yes, I'm paranoid)
        assert_eq!(
            formatter.quote("test ` * _ ~ | > # + - = . ! [ ] ( ) { } test"),
            r"test \` \* \_ \~ \| \> \# \+ \- \= \. \! \[ \] \( \) \{ \} test"
        );
    }
}
