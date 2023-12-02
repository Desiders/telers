use super::{Formatter as TextFormatter, FormatterErrorKind};

use crate::types::{
    CustomEmojiMessageEntity, MessageEntity, MessageEntityKind, PreMessageEntity,
    TextLinkMessageEntity, TextMentionMessageEntity, User,
};

use once_cell::sync::Lazy;

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
    bold_tag: &'static str,
    italic_tag: &'static str,
    underline_tag: &'static str,
    strikethrough_tag: &'static str,
    spoiler_tag: &'static str,
    emoji_tag: &'static str,
}

impl Formatter {
    /// Create a new instance of [`Formatter`]
    /// # Notes
    /// If you want to use the default tags, use `Formatter::default` instead.
    #[must_use]
    pub const fn new(
        bold_tag: &'static str,
        italic_tag: &'static str,
        underline_tag: &'static str,
        strikethrough_tag: &'static str,
        spoiler_tag: &'static str,
        emoji_tag: &'static str,
    ) -> Self {
        Self {
            bold_tag,
            italic_tag,
            underline_tag,
            strikethrough_tag,
            spoiler_tag,
            emoji_tag,
        }
    }
}

impl Default for Formatter {
    #[must_use]
    fn default() -> Self {
        Self::new(
            BOLD_TAG,
            ITALIC_TAG,
            UNDERLINE_TAG,
            STRIKETHROUGH_TAG,
            SPOILER_TAG,
            EMOJI_TAG,
        )
    }
}

impl TextFormatter for Formatter {
    fn bold<T>(&self, text: T) -> String
    where
        T: AsRef<str>,
    {
        format!(
            "<{tag}>{text}</{tag}>",
            text = text.as_ref(),
            tag = self.bold_tag
        )
    }

    fn italic<T>(&self, text: T) -> String
    where
        T: AsRef<str>,
    {
        format!(
            "<{tag}>{text}</{tag}>",
            text = text.as_ref(),
            tag = self.italic_tag
        )
    }

    fn underline<T>(&self, text: T) -> String
    where
        T: AsRef<str>,
    {
        format!(
            "<{tag}>{text}</{tag}>",
            text = text.as_ref(),
            tag = self.underline_tag
        )
    }

    fn strikethrough<T>(&self, text: T) -> String
    where
        T: AsRef<str>,
    {
        format!(
            "<{tag}>{text}</{tag}>",
            text = text.as_ref(),
            tag = self.strikethrough_tag
        )
    }

    fn spoiler<T>(&self, text: T) -> String
    where
        T: AsRef<str>,
    {
        format!(
            "<{tag}>{text}</{tag}>",
            text = text.as_ref(),
            tag = self.spoiler_tag
        )
    }

    fn text_link<T, U>(&self, text: T, url: U) -> String
    where
        T: AsRef<str>,
        U: AsRef<str>,
    {
        format!(
            "<a href=\"{url}\">{text}</a>",
            url = url.as_ref(),
            text = text.as_ref()
        )
    }

    fn text_mention<T>(&self, text: T, user_id: i64) -> String
    where
        T: AsRef<str>,
    {
        format!(
            "<a href=\"tg://user?id={user_id}\">{text}</a>",
            text = text.as_ref()
        )
    }

    fn custom_emoji<T, E>(&self, text: T, emoji_id: E) -> String
    where
        T: AsRef<str>,
        E: AsRef<str>,
    {
        format!(
            "<{tag} data-emoji-id=\"{emoji_id}\">{text}</{tag}>",
            text = text.as_ref(),
            emoji_id = emoji_id.as_ref(),
            tag = self.emoji_tag,
        )
    }

    fn code<T>(&self, text: T) -> String
    where
        T: AsRef<str>,
    {
        format!("<code>{text}</code>", text = text.as_ref())
    }

    fn pre<T>(&self, text: T) -> String
    where
        T: AsRef<str>,
    {
        format!("<pre>{text}</pre>", text = text.as_ref())
    }

    fn pre_language<T, L>(&self, text: T, language: L) -> String
    where
        T: AsRef<str>,
        L: AsRef<str>,
    {
        format!(
            "<pre><code class=\"language-{language}\">{text}</code></pre>",
            text = text.as_ref(),
            language = language.as_ref()
        )
    }

    fn quote<T>(&self, text: T) -> String
    where
        T: AsRef<str>,
    {
        text.as_ref()
            .replace('&', "&amp;")
            .replace('<', "&lt;")
            .replace('>', "&gt;")
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

pub fn text_link(text: impl AsRef<str>, url: impl AsRef<str>) -> String {
    FORMATTER.text_link(text, url)
}

pub fn text_mention(text: impl AsRef<str>, user_id: i64) -> String {
    FORMATTER.text_mention(text, user_id)
}

pub fn custom_emoji(text: impl AsRef<str>, emoji_id: impl AsRef<str>) -> String {
    FORMATTER.custom_emoji(text, emoji_id)
}

pub fn code(text: impl AsRef<str>) -> String {
    FORMATTER.code(text)
}

pub fn pre(text: impl AsRef<str>) -> String {
    FORMATTER.pre(text)
}

pub fn pre_language(text: impl AsRef<str>, language: impl AsRef<str>) -> String {
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
            "<tg-emoji data-emoji-id=\"emoji_id\">text</tg-emoji>"
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
