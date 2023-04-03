use std::fmt::{self, Debug};

#[derive(Copy, Clone, Eq, PartialEq, Hash)]
pub enum ParseMode {
    Markdown,
    MarkdownV2,
    HTML,
}

impl Debug for ParseMode {
    fn fmt(&self, f: &mut fmt::Formatter<'_>) -> fmt::Result {
        write!(f, "{}", self.as_str())
    }
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

impl From<ParseMode> for String {
    fn from(parse_mode: ParseMode) -> Self {
        parse_mode.as_str().to_string()
    }
}
