use std::fmt::{self, Debug, Display};

/// This enum represents all possible types of the poll
/// # Documentation
/// <https://core.telegram.org/bots/api#poll>
#[derive(Debug, Copy, Clone, Eq, PartialEq, Hash)]
pub enum PollType {
    Regular,
    Quiz,
}

impl PollType {
    #[must_use]
    pub const fn as_str(&self) -> &'static str {
        match self {
            PollType::Regular => "regular",
            PollType::Quiz => "quiz",
        }
    }

    #[must_use]
    pub const fn all() -> &'static [PollType; 2] {
        &[PollType::Regular, PollType::Quiz]
    }
}

impl Display for PollType {
    fn fmt(&self, f: &mut fmt::Formatter<'_>) -> fmt::Result {
        write!(f, "{}", self.as_str())
    }
}

impl From<PollType> for String {
    fn from(action: PollType) -> Self {
        action.as_str().to_owned()
    }
}

impl<'a> PartialEq<&'a str> for PollType {
    fn eq(&self, other: &&'a str) -> bool {
        self.as_str() == *other
    }
}
