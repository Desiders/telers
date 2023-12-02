use super::{Formatter as TextFormatter, FormatterErrorKind};

use crate::types::{
    CustomEmojiMessageEntity, MessageEntity, MessageEntityKind, PreMessageEntity,
    TextLinkMessageEntity, TextMentionMessageEntity, User,
};

use once_cell::sync::Lazy;
use regex::Regex;

const QUOTE_PATTERN: &str = r"([_*\[\]()~`>#+\-=|{}.!\\])";

/// This is a legacy mode, retained for backward compatibility. To use this mode, pass `Markdown` in the `parse_mode` field.
/// # Documentation
/// <https://core.telegram.org/bots/api#markdown-style>
#[derive(Debug, Clone)]
pub struct Formatter {
    regex: Regex,
}

impl Formatter {
    /// Create a new instance of [`Formatter`]
    /// # Notes
    /// If you want to use the default regex, use [`Formatter::default()`]
    #[must_use]
    pub const fn new(regex: Regex) -> Self {
        Self { regex }
    }
}

impl Default for Formatter {
    #[must_use]
    fn default() -> Self {
        Self::new(
            Regex::new(QUOTE_PATTERN)
                .expect("Invalid quote pattern. Please report this issue to the developers."),
        )
    }
}

impl TextFormatter for Formatter {
    fn bold<T>(&self, text: T) -> String
    where
        T: AsRef<str>,
    {
        format!("*{text}*", text = text.as_ref())
    }

    fn italic<T>(&self, text: T) -> String
    where
        T: AsRef<str>,
    {
        format!("_\r{text}_\r", text = text.as_ref())
    }

    fn underline<T>(&self, text: T) -> String
    where
        T: AsRef<str>,
    {
        format!("__\r{text}__\r", text = text.as_ref())
    }

    fn strikethrough<T>(&self, text: T) -> String
    where
        T: AsRef<str>,
    {
        format!("~{text}~", text = text.as_ref())
    }

    fn spoiler<T>(&self, text: T) -> String
    where
        T: AsRef<str>,
    {
        format!("|{text}|", text = text.as_ref())
    }

    fn text_link<T, U>(&self, text: T, url: U) -> String
    where
        T: AsRef<str>,
        U: AsRef<str>,
    {
        format!("[{text}]({url})", text = text.as_ref(), url = url.as_ref())
    }

    fn text_mention<T>(&self, text: T, user_id: i64) -> String
    where
        T: AsRef<str>,
    {
        self.text_link(text, format!("tg://user?id={user_id}"))
    }

    fn custom_emoji<T, E>(&self, emoji: T, emoji_id: E) -> String
    where
        T: AsRef<str>,
        E: AsRef<str>,
    {
        self.text_link(
            emoji,
            format!("tg://emoji?id={emoji_id}", emoji_id = emoji_id.as_ref()),
        )
    }

    fn code<T>(&self, text: T) -> String
    where
        T: AsRef<str>,
    {
        format!("`{text}`", text = text.as_ref())
    }

    fn pre<T>(&self, text: T) -> String
    where
        T: AsRef<str>,
    {
        format!("```\n{text}\n```", text = text.as_ref())
    }

    fn pre_language<T, L>(&self, text: T, language: L) -> String
    where
        T: AsRef<str>,
        L: AsRef<str>,
    {
        format!(
            "```{language}\n{text}\n```",
            language = language.as_ref(),
            text = text.as_ref()
        )
    }

    fn quote<T>(&self, text: T) -> String
    where
        T: AsRef<str>,
    {
        self.regex.replacen(text.as_ref(), 0, r"\$1").to_string()
    }

    fn apply_entity<T>(&self, text: T, entity: &MessageEntity) -> Result<String, FormatterErrorKind>
    where
        T: AsRef<str>,
    {
        let text = text.as_ref();
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
        };

        Ok(format!("{previous_text}{edited_text}{next_text}"))
    }
}

pub static FORMATTER: Lazy<Formatter> = Lazy::new(Formatter::default);

pub fn bold(text: impl AsRef<str>) -> String {
    FORMATTER.bold(text)
}

pub fn italic(text: impl AsRef<str>) -> String {
    FORMATTER.italic(text)
}

pub fn underline(text: impl AsRef<str>) -> String {
    FORMATTER.underline(text)
}

pub fn strikethrough(text: impl AsRef<str>) -> String {
    FORMATTER.strikethrough(text)
}

pub fn spoiler(text: impl AsRef<str>) -> String {
    FORMATTER.spoiler(text)
}

pub fn text_link(text: impl AsRef<str>, url: &str) -> String {
    FORMATTER.text_link(text, url)
}

pub fn text_mention(text: impl AsRef<str>, user_id: i64) -> String {
    FORMATTER.text_mention(text, user_id)
}

pub fn custom_emoji(text: impl AsRef<str>, emoji_id: &str) -> String {
    FORMATTER.custom_emoji(text, emoji_id)
}

pub fn code(text: impl AsRef<str>) -> String {
    FORMATTER.code(text)
}

pub fn pre(text: impl AsRef<str>) -> String {
    FORMATTER.pre(text)
}

pub fn pre_language(text: impl AsRef<str>, language: &str) -> String {
    FORMATTER.pre_language(text, language)
}

pub fn quote(text: impl AsRef<str>) -> String {
    FORMATTER.quote(text)
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_bold() {
        let formatter = Formatter::default();
        assert_eq!(formatter.bold("text"), "*text*");
    }

    #[test]
    fn test_italic() {
        let formatter = Formatter::default();
        assert_eq!(formatter.italic("text"), "_\rtext_\r");
    }

    #[test]
    fn test_underline() {
        let formatter = Formatter::default();
        assert_eq!(formatter.underline("text"), "__\rtext__\r");
    }

    #[test]
    fn test_strikethrough() {
        let formatter = Formatter::default();
        assert_eq!(formatter.strikethrough("text"), "~text~");
    }

    #[test]
    fn test_spoiler() {
        let formatter = Formatter::default();
        assert_eq!(formatter.spoiler("text"), "|text|");
    }

    #[test]
    fn test_text_link() {
        let formatter = Formatter::default();
        assert_eq!(
            formatter.text_link("text", "http://example.com"),
            "[text](http://example.com)"
        );
    }

    #[test]
    fn test_text_mention() {
        let formatter = Formatter::default();
        assert_eq!(formatter.text_mention("text", 1), "[text](tg://user?id=1)");
    }

    #[test]
    fn test_code() {
        let formatter = Formatter::default();
        assert_eq!(formatter.code("text"), "`text`");
    }

    #[test]
    fn test_pre() {
        let formatter = Formatter::default();
        assert_eq!(formatter.pre("text"), "```\ntext\n```");
    }

    #[test]
    fn test_pre_language() {
        let formatter = Formatter::default();
        assert_eq!(
            formatter.pre_language("text", "python"),
            "```python\ntext\n```"
        );
    }

    #[test]
    fn test_quote() {
        let formatter = Formatter::default();
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
