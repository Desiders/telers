use super::{formatter::split_by_entity, Formatter as TextFormatter, FormatterErrorKind};
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
        format!("<{tag}>{}</{tag}>", self.quote(text), tag = self.bold)
    }

    fn italic<T>(&self, text: T) -> String
    where
        T: Display,
    {
        format!("<{tag}>{}</{tag}>", self.quote(text), tag = self.italic)
    }

    fn underline<T>(&self, text: T) -> String
    where
        T: Display,
    {
        format!("<{tag}>{}</{tag}>", self.quote(text), tag = self.underline)
    }

    fn strikethrough<T>(&self, text: T) -> String
    where
        T: Display,
    {
        format!(
            "<{tag}>{}</{tag}>",
            self.quote(text),
            tag = self.strikethrough
        )
    }

    fn spoiler<T>(&self, text: T) -> String
    where
        T: Display,
    {
        format!("<{tag}>{}</{tag}>", self.quote(text), tag = self.spoiler)
    }

    fn blockquote<T>(&self, text: T) -> String
    where
        T: Display,
    {
        format!("<blockquote>{}</blockquote>", self.quote(text))
    }

    fn expandable_blockquote<T>(&self, text: T) -> String
    where
        T: Display,
    {
        format!("<blockquote expandable>{}</blockquote>", self.quote(text))
    }

    fn text_link<T, U>(&self, text: T, url: U) -> String
    where
        T: Display,
        U: Display,
    {
        format!("<a href=\"{url}\">{}</a>", self.quote(text))
    }

    fn text_mention<T>(&self, text: T, user_id: i64) -> String
    where
        T: Display,
    {
        format!(
            "<a href=\"tg://user?id={user_id}\">{}</a>",
            self.quote(text)
        )
    }

    fn custom_emoji<T, E>(&self, text: T, emoji_id: E) -> String
    where
        T: Display,
        E: Display,
    {
        format!(
            "<{tag} emoji-id=\"{emoji_id}\">{}</{tag}>",
            self.quote(text),
            tag = self.emoji,
        )
    }

    fn code<T>(&self, text: T) -> String
    where
        T: Display,
    {
        format!("<code>{}</code>", self.quote(text))
    }

    fn pre<T>(&self, text: T) -> String
    where
        T: Display,
    {
        format!("<pre>{}</pre>", self.quote(text))
    }

    fn pre_language<T, L>(&self, text: T, language: L) -> String
    where
        T: Display,
        L: Display,
    {
        format!(
            "<pre><code class=\"language-{language}\">{}</code></pre>",
            self.quote(text)
        )
    }

    fn date_time<T>(&self, text: T, unix_time: i64) -> String
    where
        T: Display,
    {
        format!(
            "<tg-time unix=\"{unix_time}\">{}</tg-time>",
            self.quote(text)
        )
    }

    fn date_time_with_format<T, F>(&self, text: T, unix_time: i64, date_time_format: F) -> String
    where
        T: Display,
        F: Display,
    {
        format!(
            "<tg-time unix=\"{unix_time}\" format=\"{date_time_format}\">{}</tg-time>",
            self.quote(text)
        )
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

        if text.is_empty() {
            return Err(FormatterErrorKind::EmptyText);
        }

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

        Ok(format!(
            "{}{edited_text}{}",
            self.quote(previous_text),
            self.quote(next_text)
        ))
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

    #[test]
    fn test_apply_entity_keeps_auto_detected_entities_untouched() {
        use crate::types::{
            MessageEntityBotCommand, MessageEntityCashtag, MessageEntityHashtag,
            MessageEntityMention,
        };

        let formatter = Formatter::default();
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
    fn formatting_methods_escape_their_content() {
        let formatter = Formatter::default();

        // `<`, `>` and `&` inside a span are escaped so user content can't inject markup.
        assert_eq!(formatter.bold("a<b>&c"), "<b>a&lt;b&gt;&amp;c</b>");
        assert_eq!(formatter.code("a<b"), "<code>a&lt;b</code>");
        assert_eq!(
            formatter.text_link("a&b", "http://x"),
            "<a href=\"http://x\">a&amp;b</a>"
        );
    }

    #[test]
    fn test_apply_entity_escapes_surrounding_text() {
        use crate::types::MessageEntityBold;

        let formatter = Formatter::default();

        // The literal text around the entity span must be escaped too, otherwise `<`/`>`/`&`
        // in it would break the HTML markup.
        let entity = MessageEntity::Bold(MessageEntityBold::new(0, 1));
        assert_eq!(
            formatter.apply_entity("a<b>&c", &entity).unwrap(),
            "<b>a</b>&lt;b&gt;&amp;c"
        );
    }

    #[test]
    fn apply_entity_bold_over_cyrillic_covers_whole_word() {
        use crate::types::MessageEntityBold;

        let formatter = Formatter::default();
        // "Привет" is 6 UTF-16 code units (and 6 chars) but 12 UTF-8 bytes. A byte-based slice would
        // bold only "При" and yield "<b>При</b>вет".
        let entity = MessageEntity::Bold(MessageEntityBold::new(0, 6));

        assert_eq!(
            formatter.apply_entity("Привет", &entity).unwrap(),
            "<b>Привет</b>"
        );
    }

    #[test]
    fn apply_entity_bold_after_emoji_uses_utf16_offsets() {
        use crate::types::MessageEntityBold;

        let formatter = Formatter::default();
        // "😀X": the emoji is a non-BMP scalar = 2 UTF-16 code units (4 UTF-8 bytes); "X" starts at
        // UTF-16 offset 2. A byte-based slice at offset 2 would land inside the emoji and panic.
        let entity = MessageEntity::Bold(MessageEntityBold::new(2, 1));

        assert_eq!(
            formatter.apply_entity("😀X", &entity).unwrap(),
            "😀<b>X</b>"
        );
    }
}
