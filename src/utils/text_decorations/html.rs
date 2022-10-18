use super::TextDecoration;

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
    fn bold(&self, text: &str) -> String {
        format!("<{tag}>{text}</{tag}>", tag = self.bold_tag, text = text)
    }

    fn italic(&self, text: &str) -> String {
        format!("<{tag}>{text}</{tag}>", tag = self.italic_tag, text = text)
    }

    fn code(&self, text: &str) -> String {
        format!("<code>{text}</code>", text = text)
    }

    fn underline(&self, text: &str) -> String {
        format!(
            "<{tag}>{text}</{tag}>",
            tag = self.underline_tag,
            text = text
        )
    }

    fn strikethrough(&self, text: &str) -> String {
        format!(
            "<{tag}>{text}</{tag}>",
            tag = self.strikethrough_tag,
            text = text
        )
    }

    fn spoiler(&self, text: &str) -> String {
        format!("<{tag}>{text}</{tag}>", tag = self.spoiler_tag, text = text)
    }

    fn pre(&self, text: &str) -> String {
        format!("<pre>{text}</pre>", text = text)
    }

    fn pre_language(&self, text: &str, language: &str) -> String {
        format!(
            "<pre><code class=\"language-{language}\">{text}</code></pre>",
            language = language,
            text = text
        )
    }

    fn link(&self, text: &str, url: &str) -> String {
        format!("<a href=\"{}\">{}</a>", url, text)
    }

    fn custom_emoji(&self, text: &str, emoji_id: &str) -> String {
        format!(
            "<{tag} data-emoji-id=\"{emoji_id}\">{text}</{tag}>",
            tag = self.emoji_tag,
            emoji_id = emoji_id,
            text = text
        )
    }

    fn quote(&self, text: &str) -> String {
        let mut string = text.to_string();

        for (from, to) in [('&', "&amp;"), ('<', "&lt;"), ('>', "&gt;")] {
            string = string.replace(from, to);
        }

        string
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

pub static HTML_DECORATION: HtmlDecoration<'static> = HtmlDecoration {
    bold_tag: BOLD_TAG,
    italic_tag: ITALIC_TAG,
    underline_tag: UNDERLINE_TAG,
    strikethrough_tag: STRIKETHROUGH_TAG,
    spoiler_tag: SPOILER_TAG,
    emoji_tag: EMOJI_TAG,
};
