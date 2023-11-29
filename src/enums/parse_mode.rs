use strum_macros::{AsRefStr, Display, EnumString, IntoStaticStr};

/// This enum represents all possible types of the parse mode
/// # Documentation
/// <https://core.telegram.org/bots/api#formatting-options>
#[derive(Debug, Display, Clone, Copy, PartialEq, Eq, Hash, EnumString, AsRefStr, IntoStaticStr)]
pub enum ParseMode {
    #[strum(serialize = "Markdown")]
    Markdown,
    #[strum(serialize = "MarkdownV2")]
    MarkdownV2,
    #[strum(serialize = "HTML")]
    HTML,
}

impl ParseMode {
    #[must_use]
    pub const fn all() -> [ParseMode; 3] {
        [ParseMode::Markdown, ParseMode::MarkdownV2, ParseMode::HTML]
    }
}

impl From<ParseMode> for Box<str> {
    fn from(parse_mode: ParseMode) -> Self {
        Into::<&'static str>::into(parse_mode).into()
    }
}

impl From<ParseMode> for String {
    fn from(parse_mode: ParseMode) -> Self {
        parse_mode.as_ref().to_owned()
    }
}

impl<'a> PartialEq<&'a str> for ParseMode {
    fn eq(&self, other: &&'a str) -> bool {
        self.as_ref().to_lowercase() == other.to_lowercase()
    }
}
