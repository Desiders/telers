use std::{
    fmt::{self, Debug, Display},
    ops::Deref,
};

/// This enum represents all possible types of the parse mode
/// # Documentation
/// <https://core.telegram.org/bots/api#formatting-options>
#[derive(Debug, Copy, Clone, Eq, PartialEq, Hash)]
pub enum ParseMode {
    Markdown,
    MarkdownV2,
    HTML,
}

impl ParseMode {
    #[must_use]
    pub const fn as_str(&self) -> &'static str {
        match self {
            ParseMode::Markdown => "Markdown",
            ParseMode::MarkdownV2 => "MarkdownV2",
            ParseMode::HTML => "HTML",
        }
    }

    #[must_use]
    pub const fn all() -> &'static [ParseMode; 3] {
        &[ParseMode::Markdown, ParseMode::MarkdownV2, ParseMode::HTML]
    }
}

impl Deref for ParseMode {
    type Target = str;

    fn deref(&self) -> &Self::Target {
        self.as_str()
    }
}

impl Display for ParseMode {
    fn fmt(&self, f: &mut fmt::Formatter<'_>) -> fmt::Result {
        write!(f, "{}", self.as_str())
    }
}

impl From<ParseMode> for Box<str> {
    fn from(parse_mode: ParseMode) -> Self {
        parse_mode.into()
    }
}

impl From<ParseMode> for String {
    fn from(parse_mode: ParseMode) -> Self {
        parse_mode.as_str().to_owned()
    }
}

impl<'a> PartialEq<&'a str> for ParseMode {
    fn eq(&self, other: &&'a str) -> bool {
        self.as_str().to_lowercase() == other.to_lowercase()
    }
}
