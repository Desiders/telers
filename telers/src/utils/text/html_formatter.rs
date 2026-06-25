use super::{Formatter as TextFormatter, FormatterErrorKind};
use crate::types::{
    MessageEntity, MessageEntityCustomEmoji, MessageEntityDateTime, MessageEntityPre,
    MessageEntityTextLink, MessageEntityTextMention,
};

use std::fmt::Display;

const BOLD_TAG: &str = "b";
const ITALIC_TAG: &str = "i";
const UNDERLINE_TAG: &str = "u";
const STRIKETHROUGH_TAG: &str = "s";
const SPOILER_TAG: &str = "tg-spoiler";
const EMOJI_TAG: &str = "tg-emoji";

/// To use this mode, pass `HTML` in the `parse_mode` field
/// # Documentation
/// <https://core.telegram.org/bots/api#html-style>
#[derive(Debug, Clone, Copy, PartialEq, Eq, PartialOrd, Ord, Hash)]
pub struct Formatter {
    bold: &'static str,
    italic: &'static str,
    underline: &'static str,
    strikethrough: &'static str,
    spoiler: &'static str,
    emoji: &'static str,
}

impl Formatter {
    /// Create a new instance of [`Formatter`] with custom tags
    /// # Notes
    /// If you want to use the default tags, use `Formatter::default` instead.
    #[inline]
    #[must_use]
    pub const fn new_with_tags(
        bold: &'static str,
        italic: &'static str,
        underline: &'static str,
        strikethrough: &'static str,
        spoiler: &'static str,
        emoji: &'static str,
    ) -> Self {
        Self {
            bold,
            italic,
            underline,
            strikethrough,
            spoiler,
            emoji,
        }
    }

    /// Create a new instance of [`Formatter`]
    #[inline]
    #[must_use]
    pub const fn new() -> Self {
        Self::new_with_tags(
            BOLD_TAG,
            ITALIC_TAG,
            UNDERLINE_TAG,
            STRIKETHROUGH_TAG,
            SPOILER_TAG,
            EMOJI_TAG,
        )
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
        format!("<{tag}>{text}</{tag}>", tag = self.bold)
    }

    fn italic<T>(&self, text: T) -> String
    where
        T: Display,
    {
        format!("<{tag}>{text}</{tag}>", tag = self.italic)
    }

    fn underline<T>(&self, text: T) -> String
    where
        T: Display,
    {
        format!("<{tag}>{text}</{tag}>", tag = self.underline)
    }

    fn strikethrough<T>(&self, text: T) -> String
    where
        T: Display,
    {
        format!("<{tag}>{text}</{tag}>", tag = self.strikethrough)
    }

    fn spoiler<T>(&self, text: T) -> String
    where
        T: Display,
    {
        format!("<{tag}>{text}</{tag}>", tag = self.spoiler)
    }

    fn blockquote<T>(&self, text: T) -> String
    where
        T: Display,
    {
        format!("<blockquote>{text}</blockquote>")
    }

    fn expandable_blockquote<T>(&self, text: T) -> String
    where
        T: Display,
    {
        format!("<blockquote expandable>{text}</blockquote>")
    }

    fn text_link<T, U>(&self, text: T, url: U) -> String
    where
        T: Display,
        U: Display,
    {
        format!("<a href=\"{url}\">{text}</a>")
    }

    fn text_mention<T>(&self, text: T, user_id: i64) -> String
    where
        T: Display,
    {
        format!("<a href=\"tg://user?id={user_id}\">{text}</a>")
    }

    fn custom_emoji<T, E>(&self, text: T, emoji_id: E) -> String
    where
        T: Display,
        E: Display,
    {
        format!(
            "<{tag} emoji-id=\"{emoji_id}\">{text}</{tag}>",
            tag = self.emoji,
        )
    }

    fn code<T>(&self, text: T) -> String
    where
        T: Display,
    {
        format!("<code>{text}</code>")
    }

    fn pre<T>(&self, text: T) -> String
    where
        T: Display,
    {
        format!("<pre>{text}</pre>")
    }

    fn pre_language<T, L>(&self, text: T, language: L) -> String
    where
        T: Display,
        L: Display,
    {
        format!("<pre><code class=\"language-{language}\">{text}</code></pre>")
    }

    fn date_time<T>(&self, text: T, unix_time: i64) -> String
    where
        T: Display,
    {
        format!("<tg-time unix=\"{unix_time}\">{text}</tg-time>")
    }

    fn date_time_with_format<T, F>(&self, text: T, unix_time: i64, date_time_format: F) -> String
    where
        T: Display,
        F: Display,
    {
        format!("<tg-time unix=\"{unix_time}\" format=\"{date_time_format}\">{text}</tg-time>")
    }

    fn quote<T>(&self, text: T) -> String
    where
        T: Display,
    {
        let text = text.to_string();

        text.chars()
            .fold(String::with_capacity(text.len()), |mut string, ch| {
                match ch {
                    '&' => string.push_str("&amp;"),
                    '<' => string.push_str("&lt;"),
                    '>' => string.push_str("&gt;"),
                    _ => string.push(ch),
                }
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

        let offset = usize::try_from(entity.offset()).unwrap();
        let length = usize::try_from(entity.length()).unwrap();

        if offset + length > text_len {
            return Err(FormatterErrorKind::IndexOutOfBounds);
        }

        let previous_text = &text[..offset];
        let editable_text = &text[offset..offset + length];
        let next_text = &text[offset + length..];

        let edited_text = match entity {
            MessageEntity::Mention(_) => format!("@{editable_text}"),
            MessageEntity::Hashtag(_) => format!("#{editable_text}"),
            MessageEntity::Cashtag(_) => format!("${editable_text}"),
            MessageEntity::BotCommand(_) => format!("/{editable_text}"),
            MessageEntity::Url(_) | MessageEntity::Email(_) | MessageEntity::PhoneNumber(_) => {
                editable_text.to_owned()
            }
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
pub fn text_link(text: impl Display, url: impl Display) -> String {
    FORMATTER.text_link(text, url)
}

#[inline]
pub fn text_mention(text: impl Display, user_id: i64) -> String {
    FORMATTER.text_mention(text, user_id)
}

#[inline]
pub fn custom_emoji(text: impl Display, emoji_id: impl Display) -> String {
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
pub fn pre_language(text: impl Display, language: impl Display) -> String {
    FORMATTER.pre_language(text, language)
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
        let formatter = Formatter::default();
        assert_eq!(formatter.bold("text"), "<b>text</b>");
    }

    #[test]
    fn test_italic() {
        let formatter = Formatter::default();
        assert_eq!(formatter.italic("text"), "<i>text</i>");
    }

    #[test]
    fn test_underline() {
        let formatter = Formatter::default();
        assert_eq!(formatter.underline("text"), "<u>text</u>");
    }

    #[test]
    fn test_strikethrough() {
        let formatter = Formatter::default();
        assert_eq!(formatter.strikethrough("text"), "<s>text</s>");
    }

    #[test]
    fn test_spoiler() {
        let formatter = Formatter::default();
        assert_eq!(formatter.spoiler("text"), "<tg-spoiler>text</tg-spoiler>");
    }

    #[test]
    fn test_blockquote() {
        let formatter = Formatter::default();
        assert_eq!(
            formatter.blockquote("text"),
            "<blockquote>text</blockquote>"
        );
    }

    #[test]
    fn test_expandable_blockquote() {
        let formatter = Formatter::default();
        assert_eq!(
            formatter.expandable_blockquote("text"),
            "<blockquote expandable>text</blockquote>"
        );
    }

    #[test]
    fn test_text_link() {
        let formatter = Formatter::default();
        assert_eq!(
            formatter.text_link("text", "http://example.com"),
            "<a href=\"http://example.com\">text</a>"
        );
    }

    #[test]
    fn test_text_mention() {
        let formatter = Formatter::default();
        assert_eq!(
            formatter.text_mention("text", 1),
            "<a href=\"tg://user?id=1\">text</a>"
        );
    }

    #[test]
    fn test_code() {
        let formatter = Formatter::default();
        assert_eq!(formatter.code("text"), "<code>text</code>");
    }

    #[test]
    fn test_pre() {
        let formatter = Formatter::default();
        assert_eq!(formatter.pre("text"), "<pre>text</pre>");
    }

    #[test]
    fn test_pre_language() {
        let formatter = Formatter::default();
        assert_eq!(
            formatter.pre_language("text", "python"),
            "<pre><code class=\"language-python\">text</code></pre>"
        );
    }

    #[test]
    fn test_custom_emoji() {
        let formatter = Formatter::default();
        assert_eq!(
            formatter.custom_emoji("text", "emoji_id"),
            "<tg-emoji emoji-id=\"emoji_id\">text</tg-emoji>"
        );
    }

    #[test]
    fn test_date_time() {
        let formatter = Formatter::default();
        assert_eq!(
            formatter.date_time("text", 1),
            "<tg-time unix=\"1\">text</tg-time>"
        );
        assert_eq!(
            formatter.date_time_with_format("text", 1, "test"),
            "<tg-time unix=\"1\" format=\"test\">text</tg-time>"
        );
    }

    #[test]
    fn test_quote() {
        let formatter = Formatter::default();
        assert_eq!(formatter.quote("text"), "text");
        assert_eq!(formatter.quote("<text>"), "&lt;text&gt;");
        assert_eq!(formatter.quote("&text"), "&amp;text");
    }
}
