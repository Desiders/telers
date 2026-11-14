/// This object describes the theme of the screen for sharing a link to a user or a chat. Currently, it can be one of
/// - [`crate::types::ChatShareThemeEmoji`]
/// - [`crate::types::ChatShareThemeUniqueGift`]
/// # Documentation
/// <https://core.telegram.org/bots/api#chatsharetheme>
#[derive(Clone, Debug)]
#[cfg_attr(feature = "deser", derive(deser::Serialize, deser::Deserialize))]
#[cfg_attr(not(feature = "deser"), derive(serde::Serialize, serde::Deserialize))]
#[cfg_attr(not(feature = "deser"), serde(tag = "type", rename_all = "snake_case"))]
#[cfg_attr(feature = "deser", deser(tag = "type", rename_all = "snake_case"))]
pub enum ChatShareTheme {
    Emoji(crate::types::ChatShareThemeEmoji),
    UniqueGift(crate::types::ChatShareThemeUniqueGift),
    /// Content unknown to this version of the library
    #[cfg_attr(not(feature = "deser"), serde(untagged))]
    #[cfg_attr(feature = "deser", deser(untagged))]
    Unknown(crate::types::ChatShareThemeUnknown),
}
impl ChatShareTheme {
    /// Helper method for field `background_backdrop_id`.
    ///
    /// Identifier of the backdrop used as the background; omitted if the default background is used. Use [`telers_methods::methods::GetChatShareThemeBackdrops`](https://docs.rs/telers-methods/latest/telers_methods/methods/get_chat_share_theme_backdrops/struct.GetChatShareThemeBackdrops.html) to get details about the backdrop.
    #[must_use]
    pub fn background_backdrop_id(&self) -> Option<i64> {
        match self {
            Self::Emoji(val) => val.background_backdrop_id,
            _ => None,
        }
    }

    /// Helper method for field `emojis`.
    ///
    /// The emoji; 0-6 regular or custom emoji
    #[must_use]
    pub fn emojis(&self) -> Option<&[crate::types::EmojiType]> {
        match self {
            Self::Emoji(val) => Some(val.emojis.as_ref()),
            _ => None,
        }
    }

    /// Helper method for field `gift`.
    ///
    /// The gift
    #[must_use]
    pub fn gift(&self) -> Option<&crate::types::UniqueGift> {
        match self {
            Self::UniqueGift(val) => Some(val.gift.as_ref()),
            _ => None,
        }
    }

    /// Helper method for nested field `backdrop`.
    #[must_use]
    pub fn backdrop(&self) -> Option<&crate::types::UniqueGiftBackdrop> {
        match self {
            Self::UniqueGift(val) => {
                let inner = val.gift.as_ref();
                Some(&inner.backdrop)
            }
            _ => None,
        }
    }

    /// Helper method for nested field `base_name`.
    #[must_use]
    pub fn base_name(&self) -> Option<&str> {
        match self {
            Self::UniqueGift(val) => {
                let inner = val.gift.as_ref();
                Some(inner.base_name.as_ref())
            }
            _ => None,
        }
    }

    /// Helper method for nested field `colors`.
    #[must_use]
    pub fn colors(&self) -> Option<&crate::types::UniqueGiftColors> {
        match self {
            Self::UniqueGift(val) => {
                let inner = val.gift.as_ref();
                inner.colors.as_ref()
            }
            _ => None,
        }
    }

    /// Helper method for nested field `gift_id`.
    #[must_use]
    pub fn gift_id(&self) -> Option<&str> {
        match self {
            Self::UniqueGift(val) => {
                let inner = val.gift.as_ref();
                Some(inner.gift_id.as_ref())
            }
            _ => None,
        }
    }

    /// Helper method for nested field `is_burned`.
    #[must_use]
    pub fn is_burned(&self) -> Option<bool> {
        match self {
            Self::UniqueGift(val) => {
                let inner = val.gift.as_ref();
                inner.is_burned
            }
            _ => None,
        }
    }

    /// Helper method for nested field `is_from_blockchain`.
    #[must_use]
    pub fn is_from_blockchain(&self) -> Option<bool> {
        match self {
            Self::UniqueGift(val) => {
                let inner = val.gift.as_ref();
                inner.is_from_blockchain
            }
            _ => None,
        }
    }

    /// Helper method for nested field `is_premium`.
    #[must_use]
    pub fn is_premium(&self) -> Option<bool> {
        match self {
            Self::UniqueGift(val) => {
                let inner = val.gift.as_ref();
                inner.is_premium
            }
            _ => None,
        }
    }

    /// Helper method for nested field `model`.
    #[must_use]
    pub fn model(&self) -> Option<&crate::types::UniqueGiftModel> {
        match self {
            Self::UniqueGift(val) => {
                let inner = val.gift.as_ref();
                Some(&inner.model)
            }
            _ => None,
        }
    }

    /// Helper method for nested field `name`.
    #[must_use]
    pub fn name(&self) -> Option<&str> {
        match self {
            Self::UniqueGift(val) => {
                let inner = val.gift.as_ref();
                Some(inner.name.as_ref())
            }
            _ => None,
        }
    }

    /// Helper method for nested field `number`.
    #[must_use]
    pub fn number(&self) -> Option<i64> {
        match self {
            Self::UniqueGift(val) => {
                let inner = val.gift.as_ref();
                Some(inner.number)
            }
            _ => None,
        }
    }

    /// Helper method for nested field `publisher_chat`.
    #[must_use]
    pub fn publisher_chat(&self) -> Option<&crate::types::Chat> {
        match self {
            Self::UniqueGift(val) => {
                let inner = val.gift.as_ref();
                inner.publisher_chat.as_deref()
            }
            _ => None,
        }
    }

    /// Helper method for nested field `symbol`.
    #[must_use]
    pub fn symbol(&self) -> Option<&crate::types::UniqueGiftSymbol> {
        match self {
            Self::UniqueGift(val) => {
                let inner = val.gift.as_ref();
                Some(&inner.symbol)
            }
            _ => None,
        }
    }
}
impl From<crate::types::ChatShareThemeEmoji> for ChatShareTheme {
    fn from(val: crate::types::ChatShareThemeEmoji) -> Self {
        Self::Emoji(val)
    }
}
impl TryFrom<ChatShareTheme> for crate::types::ChatShareThemeEmoji {
    type Error = crate::errors::ConvertToTypeError;

    fn try_from(val: ChatShareTheme) -> Result<Self, Self::Error> {
        if let ChatShareTheme::Emoji(inner) = val {
            Ok(inner)
        } else {
            Err(Self::Error::new(
                stringify!(ChatShareTheme),
                stringify!(ChatShareThemeEmoji),
            ))
        }
    }
}
impl From<crate::types::ChatShareThemeUniqueGift> for ChatShareTheme {
    fn from(val: crate::types::ChatShareThemeUniqueGift) -> Self {
        Self::UniqueGift(val)
    }
}
impl TryFrom<ChatShareTheme> for crate::types::ChatShareThemeUniqueGift {
    type Error = crate::errors::ConvertToTypeError;

    fn try_from(val: ChatShareTheme) -> Result<Self, Self::Error> {
        if let ChatShareTheme::UniqueGift(inner) = val {
            Ok(inner)
        } else {
            Err(Self::Error::new(
                stringify!(ChatShareTheme),
                stringify!(ChatShareThemeUniqueGift),
            ))
        }
    }
}
impl From<crate::types::ChatShareThemeUnknown> for ChatShareTheme {
    fn from(val: crate::types::ChatShareThemeUnknown) -> Self {
        Self::Unknown(val)
    }
}
impl TryFrom<ChatShareTheme> for crate::types::ChatShareThemeUnknown {
    type Error = crate::errors::ConvertToTypeError;

    fn try_from(val: ChatShareTheme) -> Result<Self, Self::Error> {
        if let ChatShareTheme::Unknown(inner) = val {
            Ok(inner)
        } else {
            Err(Self::Error::new(
                stringify!(ChatShareTheme),
                stringify!(ChatShareThemeUnknown),
            ))
        }
    }
}
