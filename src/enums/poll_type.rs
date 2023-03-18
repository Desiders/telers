use std::fmt::{self, Debug};

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
    pub const fn as_str(&self) -> &'static str {
        match self {
            PollType::Regular => "regular",
            PollType::Quiz => "quiz",
        }
    }

    pub const fn all() -> &'static [PollType; 2] {
        &[PollType::Regular, PollType::Quiz]
    }
}

impl From<PollType> for String {
    fn from(action: PollType) -> Self {
        action.as_str().to_string()
    }
}

impl<'a> From<&'a PollType> for String {
    fn from(action: &'a PollType) -> Self {
        action.as_str().to_string()
    }
}