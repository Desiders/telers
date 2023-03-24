use super::TextDecoration;

use once_cell::sync::Lazy;

const BOLD_TAG: &str = "b";
const ITALIC_TAG: &str = "i";
const UNDERLINE_TAG: &str = "u";
const STRIKETHROUGH_TAG: &str = "s";
const SPOILER_TAG: &str = "tg-spoiler";
const EMOJI_TAG: &str = "tg-emoji";

/// To use this mode, pass HTML in the `parse_mode` field. The following tags are currently supported: \
/// <b>bold</b>, <strong>bold</strong> \
/// <i>italic</i>, <em>italic</em> \
/// <u>underline</u>, <ins>underline</ins> \
/// <s>strikethrough</s>, <strike>strikethrough</strike>, <del>strikethrough</del> \
/// <span class="tg-spoiler">spoiler</span>, <tg-spoiler>spoiler</tg-spoiler> \
/// <b>bold <i>italic bold <s>italic bold strikethrough <span class="tg-spoiler">italic bold strikethrough spoiler</span></s> <u>underline italic bold</u></i> bold</b> \
/// <a href="http://www.example.com/">inline URL</a> \
/// <a href="tg://user?id=123456789">inline mention of a user</a> \
/// <code>inline fixed-width code</code>
/// <pre>pre-formatted fixed-width code block</pre>
/// <pre><code class="language-python">pre-formatted fixed-width code block written in the Python programming language</code></pre>
/// Please note:
/// - Only the tags mentioned above are currently supported.
/// - All `<`, `>` and `&` symbols that are not a part of a tag or an HTML entity must be replaced with the corresponding HTML entities (`<` with `&lt;`, `>` with `&gt;` and `&` with `&amp;`).
/// - All numerical HTML entities are supported.
/// - The API currently supports only the following named HTML entities: `&lt;`, `&gt;`, `&amp;` and `&quot;`.
/// - Use nested pre and code tags, to define programming language for pre entity.
/// - Programming language can't be specified for standalone code tags.
pub struct Decoration {
    bold_tag: &'static str,
    italic_tag: &'static str,
    underline_tag: &'static str,
    strikethrough_tag: &'static str,
    spoiler_tag: &'static str,
    emoji_tag: &'static str,
}

impl TextDecoration for Decoration {
    /// Decorate text with `bold` tag
    fn bold(&self, text: &str) -> String {
        format!("<{tag}>{text}</{tag}>", tag = self.bold_tag)
    }

    /// Decorate text with `italic` tag
    fn italic(&self, text: &str) -> String {
        format!("<{tag}>{text}</{tag}>", tag = self.italic_tag)
    }

    /// Decorate text with `code` tag
    fn code(&self, text: &str) -> String {
        format!("<code>{text}</code>")
    }

    /// Decorate text with `underline` tag
    fn underline(&self, text: &str) -> String {
        format!("<{tag}>{text}</{tag}>", tag = self.underline_tag)
    }

    /// Decorate text with `strikethrough` tag
    fn strikethrough(&self, text: &str) -> String {
        format!("<{tag}>{text}</{tag}>", tag = self.strikethrough_tag)
    }

    /// Decorate text with `spoiler` tag
    fn spoiler(&self, text: &str) -> String {
        format!("<{tag}>{text}</{tag}>", tag = self.spoiler_tag)
    }

    /// Decorate text with `pre` tag
    fn pre(&self, text: &str) -> String {
        format!("<pre>{text}</pre>")
    }

    /// Decorate text with `pre_language` tag
    fn pre_language(&self, text: &str, language: &str) -> String {
        format!("<pre><code class=\"language-{language}\">{text}</code></pre>")
    }

    /// Decorate text with `link` tag
    fn link(&self, text: &str, url: &str) -> String {
        format!("<a href=\"{url}\">{text}</a>")
    }

    /// Decorate text with `custom_emoji` tag
    fn custom_emoji(&self, text: &str, emoji_id: &str) -> String {
        format!(
            "<{tag} data-emoji-id=\"{emoji_id}\">{text}</{tag}>",
            tag = self.emoji_tag,
        )
    }

    /// Quote symbols, that can be interpreted as  HTML tags
    fn quote(&self, text: &str) -> String {
        text.replace('&', "&amp;")
            .replace('<', "&lt;")
            .replace('>', "&gt;")
    }
}

impl Decoration {
    #[must_use]
    pub fn new(
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

impl Default for Decoration {
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

pub static DECORATION: Lazy<Decoration> = Lazy::new(Decoration::default);

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_bold() {
        assert_eq!(DECORATION.bold("text"), "<b>text</b>");
    }

    #[test]
    fn test_italic() {
        assert_eq!(DECORATION.italic("text"), "<i>text</i>");
    }

    #[test]
    fn test_code() {
        assert_eq!(DECORATION.code("text"), "<code>text</code>");
    }

    #[test]
    fn test_underline() {
        assert_eq!(DECORATION.underline("text"), "<u>text</u>");
    }

    #[test]
    fn test_strikethrough() {
        assert_eq!(DECORATION.strikethrough("text"), "<s>text</s>");
    }

    #[test]
    fn test_spoiler() {
        assert_eq!(DECORATION.spoiler("text"), "<tg-spoiler>text</tg-spoiler>");
    }

    #[test]
    fn test_pre() {
        assert_eq!(DECORATION.pre("text"), "<pre>text</pre>");
    }

    #[test]
    fn test_pre_language() {
        assert_eq!(
            DECORATION.pre_language("text", "python"),
            "<pre><code class=\"language-python\">text</code></pre>"
        );
    }

    #[test]
    fn test_link() {
        assert_eq!(
            DECORATION.link("text", "http://example.com"),
            "<a href=\"http://example.com\">text</a>"
        );
    }

    #[test]
    fn test_custom_emoji() {
        assert_eq!(
            DECORATION.custom_emoji("text", "emoji_id"),
            "<tg-emoji data-emoji-id=\"emoji_id\">text</tg-emoji>"
        );
    }

    #[test]
    fn test_quote() {
        assert_eq!(DECORATION.quote("text"), "text");
        assert_eq!(DECORATION.quote("<text>"), "&lt;text&gt;");
        assert_eq!(DECORATION.quote("&text"), "&amp;text");
    }
}
