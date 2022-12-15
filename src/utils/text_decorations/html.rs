use super::TextDecoration;

use once_cell::sync::Lazy;

const BOLD_TAG: &str = "b";
const ITALIC_TAG: &str = "i";
const UNDERLINE_TAG: &str = "u";
const STRIKETHROUGH_TAG: &str = "s";
const SPOILER_TAG: &str = "tg-spoiler";
const EMOJI_TAG: &str = "tg-emoji";

#[allow(clippy::module_name_repetitions)]
pub struct HtmlDecoration<'a> {
    bold_tag: &'a str,
    italic_tag: &'a str,
    underline_tag: &'a str,
    strikethrough_tag: &'a str,
    spoiler_tag: &'a str,
    emoji_tag: &'a str,
}

impl<'a> TextDecoration for HtmlDecoration<'a> {
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
        format!("<{tag}>{text}</{tag}>", tag = self.underline_tag,)
    }

    /// Decorate text with `strikethrough` tag
    fn strikethrough(&self, text: &str) -> String {
        format!("<{tag}>{text}</{tag}>", tag = self.strikethrough_tag,)
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
        format!("<pre><code class=\"language-{language}\">{text}</code></pre>",)
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

impl<'a> HtmlDecoration<'a> {
    #[must_use]
    pub fn new(
        bold_tag: &'a str,
        italic_tag: &'a str,
        underline_tag: &'a str,
        strikethrough_tag: &'a str,
        spoiler_tag: &'a str,
        emoji_tag: &'a str,
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

impl Default for HtmlDecoration<'_> {
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

pub static HTML_DECORATION: Lazy<HtmlDecoration<'static>> = Lazy::new(HtmlDecoration::default);
