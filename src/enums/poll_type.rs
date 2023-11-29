use strum_macros::{AsRefStr, Display, EnumString, IntoStaticStr};

/// This enum represents all possible types of the poll
/// # Documentation
/// <https://core.telegram.org/bots/api#poll>
#[derive(Debug, Display, Clone, Copy, PartialEq, Eq, Hash, EnumString, AsRefStr, IntoStaticStr)]
pub enum PollType {
    #[strum(serialize = "regular")]
    Regular,
    #[strum(serialize = "quiz")]
    Quiz,
}

impl PollType {
    #[must_use]
    pub const fn all() -> [PollType; 2] {
        [PollType::Regular, PollType::Quiz]
    }
}

impl From<PollType> for Box<str> {
    fn from(action: PollType) -> Self {
        Into::<&'static str>::into(action).into()
    }
}

impl From<PollType> for String {
    fn from(action: PollType) -> Self {
        action.as_ref().to_owned()
    }
}

impl<'a> PartialEq<&'a str> for PollType {
    fn eq(&self, other: &&'a str) -> bool {
        self.as_ref() == *other
    }
}
