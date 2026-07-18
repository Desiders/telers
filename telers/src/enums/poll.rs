use crate::types::Poll;
use serde::{Deserialize, Serialize};
use strum_macros::{AsRefStr, Display, EnumString, IntoStaticStr};
/// This object contains information about a poll.
/// Currently, it can be one of
/// - [`crate::types::PollQuiz`]
/// - [`crate::types::PollRegular`]
/// # Documentation
/// <https://core.telegram.org/bots/api#poll>
#[derive(
    Debug,
    Display,
    Clone,
    Copy,
    PartialEq,
    Eq,
    Hash,
    EnumString,
    AsRefStr,
    IntoStaticStr,
    Deserialize,
    Serialize,
)]
pub enum PollType {
    #[strum(serialize = "regular")]
    Regular,
    #[strum(serialize = "quiz")]
    Quiz,
    #[strum(serialize = "unknown")]
    Unknown,
}
impl PollType {
    #[must_use]
    pub const fn all() -> [PollType; 3usize] {
        [PollType::Regular, PollType::Quiz, PollType::Unknown]
    }
}
impl From<PollType> for Box<str> {
    fn from(val: PollType) -> Self {
        Into::<&'static str>::into(val).into()
    }
}
impl From<PollType> for String {
    fn from(val: PollType) -> Self {
        val.as_ref().to_owned()
    }
}
impl<'a> PartialEq<&'a str> for PollType {
    fn eq(&self, other: &&'a str) -> bool {
        self.as_ref() == *other
    }
}
impl<'a> From<&'a Poll> for PollType {
    fn from(val: &'a Poll) -> Self {
        match val {
            Poll::Regular(_) => PollType::Regular,
            Poll::Quiz(_) => PollType::Quiz,
            Poll::Unknown(_) => PollType::Unknown,
        }
    }
}
impl From<Poll> for PollType {
    fn from(val: Poll) -> Self {
        PollType::from(&val)
    }
}
