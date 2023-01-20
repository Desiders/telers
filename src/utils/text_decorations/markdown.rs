use super::text;

use once_cell::sync::Lazy;
use regex::Regex;

const QUOTE_PATTERN: &str = r"([_*\[\]()~`>#+\-=|{}.!\\])";

/// This is a legacy mode, retained for backward compatibility. To use this mode, pass Markdown in the `parse_mode` field. Use the following syntax in your message: \
/// *bold text* \
/// _italic text_ \
/// [inline URL](http://www.example.com/) \
/// [inline mention of a user](tg://user?id=123456789) \
/// `inline fixed-width code` \
/// ``` \
/// pre-formatted fixed-width code block
/// ```
/// ```python
/// pre-formatted fixed-width code block written in the Python programming language
/// ```
/// Please note:
/// - Entities must not be nested, use parse mode `MarkdownV2` instead.
/// - There is no way to specify underline and strikethrough entities, use parse mode `MarkdownV2` instead.
/// - To escape characters `_`, `*`, `[` outside of an entity, prepend the characters `\` before them.
/// - Escaping inside entities is not allowed, so entity must be closed first and reopened again: use `_snake_\__case_` for italic `snake_case` and `*2*\**2=4*` for bold `2*2=4`.
pub struct Decoration {
    regex: Regex,
}

impl text::Decoration for Decoration {
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
        format!("```{language}\n{text}\n```")
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
        self.regex.replace_all(text, r"\\\1").to_string()
    }
}

impl Decoration {
    /// Create new instance of `Decoration`
    /// # Arguments
    /// * `quote_pattern` - pattern for quote symbols
    /// # Panics
    /// If `quote_pattern` is invalid
    #[must_use]
    pub fn new(quote_pattern: &str) -> Self {
        Self {
            regex: Regex::new(quote_pattern).unwrap(),
        }
    }
}

impl Default for Decoration {
    #[must_use]
    fn default() -> Self {
        Self::new(QUOTE_PATTERN)
    }
}

pub static DECORATION: Lazy<Decoration> = Lazy::new(Decoration::default);
