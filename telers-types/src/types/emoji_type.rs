/// This object describes an emoji. Currently, it can be one of
/// - [`crate::types::EmojiTypeEmoji`]
/// - [`crate::types::EmojiTypeCustomEmoji`]
/// # Documentation
/// <https://core.telegram.org/bots/api#emojitype>
#[derive(Clone, Debug)]
#[cfg_attr(feature = "deser", derive(deser::Serialize, deser::Deserialize))]
#[cfg_attr(not(feature = "deser"), derive(serde::Serialize, serde::Deserialize))]
#[cfg_attr(not(feature = "deser"), serde(tag = "type", rename_all = "snake_case"))]
#[cfg_attr(feature = "deser", deser(tag = "type", rename_all = "snake_case"))]
pub enum EmojiType {
    Emoji(crate::types::EmojiTypeEmoji),
    CustomEmoji(crate::types::EmojiTypeCustomEmoji),
    /// Content unknown to this version of the library
    #[cfg_attr(not(feature = "deser"), serde(untagged))]
    #[cfg_attr(feature = "deser", deser(untagged))]
    Unknown(crate::types::EmojiTypeUnknown),
}
impl EmojiType {
    /// Helper method for field `custom_emoji_id`.
    ///
    /// Identifier of the custom emoji. Use [`telers_methods::methods::GetCustomEmojiStickers`](https://docs.rs/telers-methods/latest/telers_methods/methods/get_custom_emoji_stickers/struct.GetCustomEmojiStickers.html) to get full information about the sticker.
    #[must_use]
    pub fn custom_emoji_id(&self) -> Option<&str> {
        match self {
            Self::CustomEmoji(val) => Some(val.custom_emoji_id.as_ref()),
            _ => None,
        }
    }

    /// Helper method for field `emoji`.
    ///
    /// The emoji
    #[must_use]
    pub fn emoji(&self) -> Option<&str> {
        match self {
            Self::Emoji(val) => Some(val.emoji.as_ref()),
            _ => None,
        }
    }
}
impl From<crate::types::EmojiTypeEmoji> for EmojiType {
    fn from(val: crate::types::EmojiTypeEmoji) -> Self {
        Self::Emoji(val)
    }
}
impl TryFrom<EmojiType> for crate::types::EmojiTypeEmoji {
    type Error = crate::errors::ConvertToTypeError;

    fn try_from(val: EmojiType) -> Result<Self, Self::Error> {
        if let EmojiType::Emoji(inner) = val {
            Ok(inner)
        } else {
            Err(Self::Error::new(
                stringify!(EmojiType),
                stringify!(EmojiTypeEmoji),
            ))
        }
    }
}
impl From<crate::types::EmojiTypeCustomEmoji> for EmojiType {
    fn from(val: crate::types::EmojiTypeCustomEmoji) -> Self {
        Self::CustomEmoji(val)
    }
}
impl TryFrom<EmojiType> for crate::types::EmojiTypeCustomEmoji {
    type Error = crate::errors::ConvertToTypeError;

    fn try_from(val: EmojiType) -> Result<Self, Self::Error> {
        if let EmojiType::CustomEmoji(inner) = val {
            Ok(inner)
        } else {
            Err(Self::Error::new(
                stringify!(EmojiType),
                stringify!(EmojiTypeCustomEmoji),
            ))
        }
    }
}
impl From<crate::types::EmojiTypeUnknown> for EmojiType {
    fn from(val: crate::types::EmojiTypeUnknown) -> Self {
        Self::Unknown(val)
    }
}
impl TryFrom<EmojiType> for crate::types::EmojiTypeUnknown {
    type Error = crate::errors::ConvertToTypeError;

    fn try_from(val: EmojiType) -> Result<Self, Self::Error> {
        if let EmojiType::Unknown(inner) = val {
            Ok(inner)
        } else {
            Err(Self::Error::new(
                stringify!(EmojiType),
                stringify!(EmojiTypeUnknown),
            ))
        }
    }
}
