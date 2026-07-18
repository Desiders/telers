use serde::{Deserialize, Serialize};
/// This object describes the type of a reaction. Currently, it can be one of
/// - [`crate::types::ReactionTypeEmoji`]
/// - [`crate::types::ReactionTypeCustomEmoji`]
/// - [`crate::types::ReactionTypePaid`]
/// # Documentation
/// <https://core.telegram.org/bots/api#reactiontype>
#[derive(Clone, Debug, Serialize, Deserialize)]
#[serde(tag = "type", rename_all = "snake_case")]
pub enum ReactionType {
    Emoji(crate::types::ReactionTypeEmoji),
    CustomEmoji(crate::types::ReactionTypeCustomEmoji),
    Paid(crate::types::ReactionTypePaid),
    /// Content unknown to this version of the library
    #[serde(untagged)]
    Unknown(crate::types::ReactionTypeUnknown),
}
impl ReactionType {
    /// Helper method for field `custom_emoji_id`.
    ///
    /// Custom emoji identifier
    #[must_use]
    pub fn custom_emoji_id(&self) -> Option<&str> {
        match self {
            Self::CustomEmoji(val) => Some(val.custom_emoji_id.as_ref()),
            _ => None,
        }
    }

    /// Helper method for field `emoji`.
    ///
    /// Reaction emoji. Currently, it can be one of `❤`, `👍`, `👎`, `🔥`, `🥰`, `👏`, `😁`, `🤔`, `🤯`, `😱`, `🤬`, `😢`, `🎉`, `🤩`, `🤮`, `💩`, `🙏`, `👌`, `🕊`, `🤡`, `🥱`, `🥴`, `😍`, `🐳`, `❤‍🔥`, `🌚`, `🌭`, `💯`, `🤣`, `⚡`, `🍌`, `🏆`, `💔`, `🤨`, `😐`, `🍓`, `🍾`, `💋`, `🖕`, `😈`, `😴`, `😭`, `🤓`, `👻`, `👨‍💻`, `👀`, `🎃`, `🙈`, `😇`, `😨`, `🤝`, `✍`, `🤗`, `🫡`, `🎅`, `🎄`, `☃`, `💅`, `🤪`, `🗿`, `🆒`, `💘`, `🙉`, `🦄`, `😘`, `💊`, `🙊`, `😎`, `👾`, `🤷‍♂`, `🤷`, `🤷‍♀`, `😡`.
    #[must_use]
    pub fn emoji(&self) -> Option<&str> {
        match self {
            Self::Emoji(val) => Some(val.emoji.as_ref()),
            _ => None,
        }
    }
}
impl From<crate::types::ReactionTypeEmoji> for ReactionType {
    fn from(val: crate::types::ReactionTypeEmoji) -> Self {
        Self::Emoji(val)
    }
}
impl TryFrom<ReactionType> for crate::types::ReactionTypeEmoji {
    type Error = crate::errors::ConvertToTypeError;

    fn try_from(val: ReactionType) -> Result<Self, Self::Error> {
        if let ReactionType::Emoji(inner) = val {
            Ok(inner)
        } else {
            Err(Self::Error::new(
                stringify!(ReactionType),
                stringify!(ReactionTypeEmoji),
            ))
        }
    }
}
impl From<crate::types::ReactionTypeCustomEmoji> for ReactionType {
    fn from(val: crate::types::ReactionTypeCustomEmoji) -> Self {
        Self::CustomEmoji(val)
    }
}
impl TryFrom<ReactionType> for crate::types::ReactionTypeCustomEmoji {
    type Error = crate::errors::ConvertToTypeError;

    fn try_from(val: ReactionType) -> Result<Self, Self::Error> {
        if let ReactionType::CustomEmoji(inner) = val {
            Ok(inner)
        } else {
            Err(Self::Error::new(
                stringify!(ReactionType),
                stringify!(ReactionTypeCustomEmoji),
            ))
        }
    }
}
impl From<crate::types::ReactionTypePaid> for ReactionType {
    fn from(val: crate::types::ReactionTypePaid) -> Self {
        Self::Paid(val)
    }
}
impl TryFrom<ReactionType> for crate::types::ReactionTypePaid {
    type Error = crate::errors::ConvertToTypeError;

    fn try_from(val: ReactionType) -> Result<Self, Self::Error> {
        if let ReactionType::Paid(inner) = val {
            Ok(inner)
        } else {
            Err(Self::Error::new(
                stringify!(ReactionType),
                stringify!(ReactionTypePaid),
            ))
        }
    }
}
impl From<crate::types::ReactionTypeUnknown> for ReactionType {
    fn from(val: crate::types::ReactionTypeUnknown) -> Self {
        Self::Unknown(val)
    }
}
impl TryFrom<ReactionType> for crate::types::ReactionTypeUnknown {
    type Error = crate::errors::ConvertToTypeError;

    fn try_from(val: ReactionType) -> Result<Self, Self::Error> {
        if let ReactionType::Unknown(inner) = val {
            Ok(inner)
        } else {
            Err(Self::Error::new(
                stringify!(ReactionType),
                stringify!(ReactionTypeUnknown),
            ))
        }
    }
}
