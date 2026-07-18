use crate::types::ReactionType;
use serde::{Deserialize, Serialize};
use strum_macros::{AsRefStr, Display, EnumString, IntoStaticStr};
/// This object describes the type of a reaction. Currently, it can be one of
/// - [`crate::types::ReactionTypeEmoji`]
/// - [`crate::types::ReactionTypeCustomEmoji`]
/// - [`crate::types::ReactionTypePaid`]
/// # Documentation
/// <https://core.telegram.org/bots/api#reactiontype>
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
pub enum ReactionTypeType {
    #[strum(serialize = "emoji")]
    Emoji,
    #[strum(serialize = "custom_emoji")]
    CustomEmoji,
    #[strum(serialize = "paid")]
    Paid,
    #[strum(serialize = "unknown")]
    Unknown,
}
impl ReactionTypeType {
    #[must_use]
    pub const fn all() -> [ReactionTypeType; 4usize] {
        [
            ReactionTypeType::Emoji,
            ReactionTypeType::CustomEmoji,
            ReactionTypeType::Paid,
            ReactionTypeType::Unknown,
        ]
    }
}
impl From<ReactionTypeType> for Box<str> {
    fn from(val: ReactionTypeType) -> Self {
        Into::<&'static str>::into(val).into()
    }
}
impl From<ReactionTypeType> for String {
    fn from(val: ReactionTypeType) -> Self {
        val.as_ref().to_owned()
    }
}
impl<'a> PartialEq<&'a str> for ReactionTypeType {
    fn eq(&self, other: &&'a str) -> bool {
        self.as_ref() == *other
    }
}
impl<'a> From<&'a ReactionType> for ReactionTypeType {
    fn from(val: &'a ReactionType) -> Self {
        match val {
            ReactionType::Emoji(_) => ReactionTypeType::Emoji,
            ReactionType::CustomEmoji(_) => ReactionTypeType::CustomEmoji,
            ReactionType::Paid(_) => ReactionTypeType::Paid,
            ReactionType::Unknown(_) => ReactionTypeType::Unknown,
        }
    }
}
impl From<ReactionType> for ReactionTypeType {
    fn from(val: ReactionType) -> Self {
        ReactionTypeType::from(&val)
    }
}
