use crate::types::StoryAreaType;
use serde::{Deserialize, Serialize};
use strum_macros::{AsRefStr, Display, EnumString, IntoStaticStr};
/// Describes the type of a clickable area on a story. Currently, it can be one of
/// - [`crate::types::StoryAreaTypeLocation`]
/// - [`crate::types::StoryAreaTypeSuggestedReaction`]
/// - [`crate::types::StoryAreaTypeLink`]
/// - [`crate::types::StoryAreaTypeWeather`]
/// - [`crate::types::StoryAreaTypeUniqueGift`]
/// # Documentation
/// <https://core.telegram.org/bots/api#storyareatype>
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
pub enum StoryAreaTypeType {
    #[strum(serialize = "location")]
    Location,
    #[strum(serialize = "suggested_reaction")]
    SuggestedReaction,
    #[strum(serialize = "link")]
    Link,
    #[strum(serialize = "weather")]
    Weather,
    #[strum(serialize = "unique_gift")]
    UniqueGift,
}
impl StoryAreaTypeType {
    #[must_use]
    pub const fn all() -> [StoryAreaTypeType; 5usize] {
        [
            StoryAreaTypeType::Location,
            StoryAreaTypeType::SuggestedReaction,
            StoryAreaTypeType::Link,
            StoryAreaTypeType::Weather,
            StoryAreaTypeType::UniqueGift,
        ]
    }
}
impl From<StoryAreaTypeType> for Box<str> {
    fn from(val: StoryAreaTypeType) -> Self {
        Into::<&'static str>::into(val).into()
    }
}
impl From<StoryAreaTypeType> for String {
    fn from(val: StoryAreaTypeType) -> Self {
        val.as_ref().to_owned()
    }
}
impl<'a> PartialEq<&'a str> for StoryAreaTypeType {
    fn eq(&self, other: &&'a str) -> bool {
        self.as_ref() == *other
    }
}
impl<'a> From<&'a StoryAreaType> for StoryAreaTypeType {
    fn from(val: &'a StoryAreaType) -> Self {
        match val {
            StoryAreaType::Location(_) => StoryAreaTypeType::Location,
            StoryAreaType::SuggestedReaction(_) => StoryAreaTypeType::SuggestedReaction,
            StoryAreaType::Link(_) => StoryAreaTypeType::Link,
            StoryAreaType::Weather(_) => StoryAreaTypeType::Weather,
            StoryAreaType::UniqueGift(_) => StoryAreaTypeType::UniqueGift,
        }
    }
}
impl From<StoryAreaType> for StoryAreaTypeType {
    fn from(val: StoryAreaType) -> Self {
        StoryAreaTypeType::from(&val)
    }
}
