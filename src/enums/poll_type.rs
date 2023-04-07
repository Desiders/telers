use std::fmt::{self, Debug};

/// This enum represents all possible types of the poll
/// # Documentation
/// <https://core.telegram.org/bots/api#poll>
#[derive(Copy, Clone, Eq, PartialEq, Hash)]
pub enum PollType {
    Regular,
    Quiz,
}

impl Debug for PollType {
    fn fmt(&self, f: &mut fmt::Formatter<'_>) -> fmt::Result {
        write!(f, "{}", self.as_str())
    }
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

impl From<PollType> for String {
    fn from(action: PollType) -> Self {
        action.as_str().to_string()
    }
}
