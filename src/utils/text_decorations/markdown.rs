use super::TextDecoration;

use regex::Regex;

const QUOTE_PATTERN_STR: &str = r"([_*\[\]()~`>#+\-=|{}.!\\])";

#[allow(clippy::module_name_repetitions)]
pub struct MarkdownDecoration<'a> {
    quote_pattern_str: &'a str,
}

impl<'a> TextDecoration for MarkdownDecoration<'a> {
    /// Decorate text with `bold` tag
    fn bold(&self, text: &str) -> String {
        format!("*{text}*")
    }

    /// Decorate text with `italic` tag
    fn italic(&self, text: &str) -> String {
        format!("_\r{text}_\r")
    }

    /// Decorate text with `code` tag
    fn code(&self, text: &str) -> String {
        format!("`{text}`")
    }

    /// Decorate text with `underline` tag
    fn underline(&self, text: &str) -> String {
        format!("__\r{text}__\r")
    }

    /// Decorate text with `strikethrough` tag
    fn strikethrough(&self, text: &str) -> String {
        format!("~{text}~")
    }

    /// Decorate text with `spoiler` tag
    fn spoiler(&self, text: &str) -> String {
        format!("|{text}|")
    }

    /// Decorate text with `pre` tag
    fn pre(&self, text: &str) -> String {
        format!("```\n{text}\n```")
    }

    /// Decorate text with `pre_language` tag
    fn pre_language(&self, text: &str, language: &str) -> String {
        format!("```{language}\n{text}\n```",)
    }

    /// Decorate text with `link` tag
    fn link(&self, text: &str, url: &str) -> String {
        format!("[{text}]({url})")
    }

    /// Decorate text with `custom_emoji` tag
    fn custom_emoji(&self, text: &str, emoji_id: &str) -> String {
        self.link(text, format!("tg://emoji?id={emoji_id}").as_str())
    }

    /// Quote symbols, that can be interpreted as markdown
    fn quote(&self, text: &str) -> String {
        Regex::new(self.quote_pattern_str)
            .unwrap()
            .replace_all(text, r"\\\1")
            .to_string()
    }
}

impl<'a> MarkdownDecoration<'a> {
    #[must_use]
    pub fn new(quote_pattern_str: &'a str) -> Self {
        Self { quote_pattern_str }
    }
}

impl Default for MarkdownDecoration<'_> {
    #[must_use]
    fn default() -> Self {
        Self::new(QUOTE_PATTERN_STR)
    }
}

pub static MARKDOWN_DECORATION: MarkdownDecoration<'static> = MarkdownDecoration {
    quote_pattern_str: QUOTE_PATTERN_STR,
};
