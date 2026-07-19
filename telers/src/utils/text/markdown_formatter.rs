use super::{formatter::split_by_entity, Formatter as TextFormatter, FormatterErrorKind};
use crate::types::{
    MessageEntity, MessageEntityCustomEmoji, MessageEntityDateTime, MessageEntityPre,
    MessageEntityTextLink, MessageEntityTextMention,
};

use std::fmt::Display;

pub(crate) const ESCAPE_CHARS: [char; 19] = [
    // `\` must be escaped first-class: a literal backslash in text would otherwise be read
    // by MarkdownV2 as escaping the following character.
    '\\', '_', '*', '[', ']', '(', ')', '~', '`', '>', '#', '+', '-', '=', '|', '{', '}', '.', '!',
];

/// Characters that MarkdownV2 requires to be escaped **inside** `code` and `pre` entities
pub(crate) const CODE_ESCAPE_CHARS: [char; 2] = ['`', '\\'];

/// Characters that MarkdownV2 requires to be escaped **inside** the `(...)` of an inline link or custom emoji definition
pub(crate) const URL_ESCAPE_CHARS: [char; 2] = [')', '\\'];

fn escape(text: impl Display, chars: &[char]) -> String {
    let text = text.to_string();

    text.chars()
        .fold(String::with_capacity(text.len()), |mut string, ch| {
            if chars.contains(&ch) {
                string.push('\\');
            }
            string.push(ch);
            string
        })
}

/// # Documentation
/// <https://core.telegram.org/bots/api#markdown-style>
#[derive(Debug, Clone)]
pub struct Formatter;

impl Formatter {
    /// Create a new instance of [`Formatter`]
    #[inline]
    #[must_use]
    pub const fn new() -> Self {
        Self {}
    }
}

impl Default for Formatter {
    #[inline]
    fn default() -> Self {
        Self::new()
    }
}

impl TextFormatter for Formatter {
    fn bold<T>(&self, text: T) -> String
    where
        T: Display,
    {
        format!("*{}*", self.quote(text))
    }

    fn italic<T>(&self, text: T) -> String
    where
        T: Display,
    {
        format!("_\r{}_\r", self.quote(text))
    }

    fn underline<T>(&self, text: T) -> String
    where
        T: Display,
    {
        format!("__\r{}__\r", self.quote(text))
    }

    fn strikethrough<T>(&self, text: T) -> String
    where
        T: Display,
    {
        format!("~{}~", self.quote(text))
    }

    fn spoiler<T>(&self, text: T) -> String
    where
        T: Display,
    {
        format!("||{}||", self.quote(text))
    }

    fn blockquote<T>(&self, text: T) -> String
    where
        T: Display,
    {
        self.quote(text)
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
        format!("[{}]({})", self.quote(text), escape(url, &URL_ESCAPE_CHARS))
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
        format!("`{}`", escape(text, &CODE_ESCAPE_CHARS))
    }

    fn pre<T>(&self, text: T) -> String
    where
        T: Display,
    {
        format!("```\n{}\n```", escape(text, &CODE_ESCAPE_CHARS))
    }

    fn pre_language<T, L>(&self, text: T, language: L) -> String
    where
        T: Display,
        L: Display,
    {
        format!("```{language}\n{}\n```", escape(text, &CODE_ESCAPE_CHARS))
    }

    fn date_time<T>(&self, text: T, unix_time: i64) -> String
    where
        T: Display,
    {
        format!(
            "!{}",
            self.text_link(text, format!("tg://time?unix={unix_time}"))
        )
    }

    fn date_time_with_format<T, F>(&self, text: T, unix_time: i64, date_time_format: F) -> String
    where
        T: Display,
        F: Display,
    {
        format!(
            "!{}",
            self.text_link(
                text,
                format!("tg://time?unix={unix_time}&format={date_time_format}")
            )
        )
    }

    fn quote<T>(&self, text: T) -> String
    where
        T: Display,
    {
        escape(text, &ESCAPE_CHARS)
    }

    fn apply_entity<T>(&self, text: T, entity: &MessageEntity) -> Result<String, FormatterErrorKind>
    where
        T: Display,
    {
        let text = text.to_string();

        if text.is_empty() {
            return Err(FormatterErrorKind::EmptyText);
        }

        // Entity `offset`/`length` are UTF-16 code units, so split `text` in the UTF-16 domain (see
        // `super::formatter::split_by_entity`) instead of slicing it by raw byte indices.
        let (previous_text, editable_text, next_text) = split_by_entity(&text, entity)?;

        let edited_text = match entity {
            // Auto-detected entities (their prefix `@`/`#`/`$`/`/` is already part of the
            // entity span, and Telegram re-detects them) must be returned untouched.
            MessageEntity::Mention(_)
            | MessageEntity::Hashtag(_)
            | MessageEntity::Cashtag(_)
            | MessageEntity::BotCommand(_)
            | MessageEntity::Url(_)
            | MessageEntity::Email(_)
            // Entity types unknown to the library can't be re-formatted either, so their
            // span is also kept as is.
            | MessageEntity::PhoneNumber(_)
            | MessageEntity::Unknown(_) => editable_text.to_owned(),
            MessageEntity::Bold(_) => self.bold(editable_text),
            MessageEntity::Italic(_) => self.italic(editable_text),
            MessageEntity::Underline(_) => self.underline(editable_text),
            MessageEntity::Strikethrough(_) => self.strikethrough(editable_text),
            MessageEntity::Spoiler(_) => self.spoiler(editable_text),
            MessageEntity::Blockquote(_) => self.blockquote(editable_text),
            MessageEntity::ExpandableBlockquote(_) => self.expandable_blockquote(editable_text),
            MessageEntity::Code(_) => self.code(editable_text),
            MessageEntity::Pre(MessageEntityPre {
                language, ..
            }) => match language {
                Some(language) => self.pre_language(editable_text, language),
                None => self.pre(editable_text),
            },
            MessageEntity::TextLink(MessageEntityTextLink {
                url, ..
            }) => self.text_link(editable_text, url),
            MessageEntity::TextMention(MessageEntityTextMention {
                user, ..
            }) => self.text_mention(editable_text, user.id),
            MessageEntity::CustomEmoji(MessageEntityCustomEmoji {
                custom_emoji_id, ..
            }) => self.custom_emoji(editable_text, custom_emoji_id),
            MessageEntity::DateTime(MessageEntityDateTime {
                unix_time,
                date_time_format,
                ..
            }) => match date_time_format {
                Some(date_time_format) => {
                    self.date_time_with_format(editable_text, *unix_time, date_time_format)
                }
                None => self.date_time(editable_text, *unix_time),
            },
        };

        Ok(format!("{previous_text}{edited_text}{next_text}"))
    }
}

pub const FORMATTER: Formatter = Formatter::new();

#[inline]
pub fn bold(text: impl Display) -> String {
    FORMATTER.bold(text)
}

#[inline]
pub fn italic(text: impl Display) -> String {
    FORMATTER.italic(text)
}

#[inline]
pub fn underline(text: impl Display) -> String {
    FORMATTER.underline(text)
}

#[inline]
pub fn strikethrough(text: impl Display) -> String {
    FORMATTER.strikethrough(text)
}

#[inline]
pub fn spoiler(text: impl Display) -> String {
    FORMATTER.spoiler(text)
}

#[inline]
pub fn blockquote(text: impl Display) -> String {
    FORMATTER.blockquote(text)
}

#[inline]
pub fn expandable_blockquote(text: impl Display) -> String {
    FORMATTER.expandable_blockquote(text)
}

#[inline]
pub fn text_link(text: impl Display, url: &str) -> String {
    FORMATTER.text_link(text, url)
}

#[inline]
pub fn text_mention(text: impl Display, user_id: i64) -> String {
    FORMATTER.text_mention(text, user_id)
}

#[inline]
pub fn custom_emoji(text: impl Display, emoji_id: &str) -> String {
    FORMATTER.custom_emoji(text, emoji_id)
}

#[inline]
pub fn code(text: impl Display) -> String {
    FORMATTER.code(text)
}

#[inline]
pub fn pre(text: impl Display) -> String {
    FORMATTER.pre(text)
}

#[inline]
pub fn pre_language(text: impl Display, language: &str) -> String {
    FORMATTER.pre_language(text, language)
}

#[inline]
pub fn date_time(text: impl Display, unix_time: i64) -> String {
    FORMATTER.date_time(text, unix_time)
}

#[inline]
pub fn date_time_with_format(
    text: impl Display,
    unix_time: i64,
    date_time_format: impl Display,
) -> String {
    FORMATTER.date_time_with_format(text, unix_time, date_time_format)
}

#[inline]
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
        assert_eq!(formatter.spoiler("text"), "||text||");
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
    fn test_date_time() {
        let formatter = Formatter::default();
        assert_eq!(formatter.date_time("text", 1), "![text](tg://time?unix=1)");
        assert_eq!(
            formatter.date_time_with_format("text", 1, "test"),
            "![text](tg://time?unix=1&format=test)"
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

    #[test]
    fn test_apply_entity_keeps_auto_detected_entities_untouched() {
        use crate::types::{
            MessageEntityBotCommand, MessageEntityCashtag, MessageEntityHashtag,
            MessageEntityMention,
        };

        let formatter = Formatter;
        // Each entity span already includes its prefix char, so applying it must not add
        // a second one (no `@@user`, `##tag`, ...).
        let text = "@user #tag $CASH /cmd";
        for entity in [
            MessageEntity::Mention(MessageEntityMention::new(0, 5)),
            MessageEntity::Hashtag(MessageEntityHashtag::new(6, 4)),
            MessageEntity::Cashtag(MessageEntityCashtag::new(11, 5)),
            MessageEntity::BotCommand(MessageEntityBotCommand::new(17, 4)),
        ] {
            assert_eq!(formatter.apply_entity(text, &entity).unwrap(), text);
        }
    }

    #[test]
    fn test_code_escapes_backtick_and_backslash() {
        let formatter = Formatter;

        assert_eq!(formatter.code("a`b"), r"`a\`b`");
        assert_eq!(formatter.code(r"a\b"), r"`a\\b`");
        assert_eq!(formatter.code("a_b*c"), "`a_b*c`");
    }

    #[test]
    fn test_pre_escapes_backtick_and_backslash() {
        let formatter = Formatter;

        assert_eq!(formatter.pre("a`b"), "```\na\\`b\n```");
        assert_eq!(formatter.pre_language("a`b", "rust"), "```rust\na\\`b\n```");
        assert_eq!(formatter.pre("a_b"), "```\na_b\n```");
    }

    #[test]
    fn test_text_link_escapes_closing_paren_and_backslash_in_url() {
        let formatter = Formatter;

        assert_eq!(
            formatter.text_link("text", "http://x/a)b"),
            r"[text](http://x/a\)b)"
        );
        assert_eq!(
            formatter.text_link("text", r"http://x/a\b"),
            r"[text](http://x/a\\b)"
        );
        assert_eq!(
            formatter.text_link("a_b", "http://example.com"),
            "[a_b](http://example.com)"
        );
    }

    #[test]
    fn test_custom_emoji_and_date_time_escape_their_url() {
        let formatter = Formatter;

        assert_eq!(
            formatter.custom_emoji("x", "1)2"),
            r"![x](tg://emoji?id=1\)2)"
        );
        assert_eq!(
            formatter.date_time_with_format("x", 1, "a)b"),
            r"![x](tg://time?unix=1&format=a\)b)"
        );
    }

    #[test]
    fn apply_entity_bold_over_cyrillic_covers_whole_word() {
        use crate::types::MessageEntityBold;

        let formatter = Formatter;
        // "Привет" is 6 UTF-16 code units (and 6 chars) but 12 UTF-8 bytes. A byte-based slice would
        // bold only "При" and yield "*При*вет".
        let entity = MessageEntity::Bold(MessageEntityBold::new(0, 6));

        assert_eq!(
            formatter.apply_entity("Привет", &entity).unwrap(),
            "*Привет*"
        );
    }

    #[test]
    fn apply_entity_bold_after_emoji_uses_utf16_offsets() {
        use crate::types::MessageEntityBold;

        let formatter = Formatter;
        // "😀X": the emoji is a non-BMP scalar = 2 UTF-16 code units (4 UTF-8 bytes); "X" starts at
        // UTF-16 offset 2. A byte-based slice at offset 2 would land inside the emoji and panic.
        let entity = MessageEntity::Bold(MessageEntityBold::new(2, 1));

        assert_eq!(formatter.apply_entity("😀X", &entity).unwrap(), "😀*X*");
    }
}
