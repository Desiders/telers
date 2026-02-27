use serde::{Deserialize, Serialize};
/// Describes the type of a clickable area on a story. Currently, it can be one of
/// - [`StoryAreaTypeLocation`]
/// - [`StoryAreaTypeSuggestedReaction`]
/// - [`StoryAreaTypeLink`]
/// - [`StoryAreaTypeWeather`]
/// - [`StoryAreaTypeUniqueGift`]
/// # Documentation
/// <https://core.telegram.org/bots/api#storyareatype>
#[derive(Clone, Debug, Serialize, Deserialize)]
#[serde(tag = "type", rename_all = "snake_case")]
pub enum StoryAreaType {
    Location(crate::types::StoryAreaTypeLocation),
    SuggestedReaction(crate::types::StoryAreaTypeSuggestedReaction),
    Link(crate::types::StoryAreaTypeLink),
    Weather(crate::types::StoryAreaTypeWeather),
    UniqueGift(crate::types::StoryAreaTypeUniqueGift),
}
impl StoryAreaType {
    /// Helper method for field `address`.
    ///
    /// # Variants
    /// - `StoryAreaTypeLocation`. Address of the location
    #[must_use]
    pub fn address(&self) -> Option<&crate::types::LocationAddress> {
        match self {
            Self::Location(val) => val.address.as_ref(),
            _ => None,
        }
    }

    /// Helper method for field `background_color`.
    ///
    /// # Variants
    /// - `StoryAreaTypeWeather`. A color of the area background in the ARGB format
    #[must_use]
    pub fn background_color(&self) -> Option<i64> {
        match self {
            Self::Weather(val) => Some(val.background_color),
            _ => None,
        }
    }

    /// Helper method for field `emoji`.
    ///
    /// # Variants
    /// - `StoryAreaTypeWeather`. Emoji representing the weather
    #[must_use]
    pub fn emoji(&self) -> Option<&str> {
        match self {
            Self::Weather(val) => Some(val.emoji.as_ref()),
            _ => None,
        }
    }

    /// Helper method for field `is_dark`.
    ///
    /// # Variants
    /// - `StoryAreaTypeSuggestedReaction`. Pass `true` if the reaction area has a dark background
    #[must_use]
    pub fn is_dark(&self) -> Option<bool> {
        match self {
            Self::SuggestedReaction(val) => val.is_dark,
            _ => None,
        }
    }

    /// Helper method for field `is_flipped`.
    ///
    /// # Variants
    /// - `StoryAreaTypeSuggestedReaction`. Pass `true` if reaction area corner is flipped
    #[must_use]
    pub fn is_flipped(&self) -> Option<bool> {
        match self {
            Self::SuggestedReaction(val) => val.is_flipped,
            _ => None,
        }
    }

    /// Helper method for field `latitude`.
    ///
    /// # Variants
    /// - `StoryAreaTypeLocation`. Location latitude in degrees
    #[must_use]
    pub fn latitude(&self) -> Option<f64> {
        match self {
            Self::Location(val) => Some(val.latitude),
            _ => None,
        }
    }

    /// Helper method for field `longitude`.
    ///
    /// # Variants
    /// - `StoryAreaTypeLocation`. Location longitude in degrees
    #[must_use]
    pub fn longitude(&self) -> Option<f64> {
        match self {
            Self::Location(val) => Some(val.longitude),
            _ => None,
        }
    }

    /// Helper method for field `name`.
    ///
    /// # Variants
    /// - `StoryAreaTypeUniqueGift`. Unique name of the gift
    #[must_use]
    pub fn name(&self) -> Option<&str> {
        match self {
            Self::UniqueGift(val) => Some(val.name.as_ref()),
            _ => None,
        }
    }

    /// Helper method for field `reaction_type`.
    ///
    /// # Variants
    /// - `StoryAreaTypeSuggestedReaction`. Type of the reaction
    #[must_use]
    pub fn reaction_type(&self) -> Option<&crate::types::ReactionType> {
        match self {
            Self::SuggestedReaction(val) => Some(&val.reaction_type),
            _ => None,
        }
    }

    /// Helper method for field `temperature`.
    ///
    /// # Variants
    /// - `StoryAreaTypeWeather`. Temperature, in degree Celsius
    #[must_use]
    pub fn temperature(&self) -> Option<f64> {
        match self {
            Self::Weather(val) => Some(val.temperature),
            _ => None,
        }
    }

    /// Helper method for field `url`.
    ///
    /// # Variants
    /// - `StoryAreaTypeLink`. HTTP or tg:// URL to be opened when the area is clicked
    #[must_use]
    pub fn url(&self) -> Option<&str> {
        match self {
            Self::Link(val) => Some(val.url.as_ref()),
            _ => None,
        }
    }

    /// Helper method for nested field `city`.
    #[must_use]
    pub fn city(&self) -> Option<&str> {
        match self {
            Self::Location(val) => val.address.as_ref().and_then(|inner| inner.city.as_deref()),
            _ => None,
        }
    }

    /// Helper method for nested field `country_code`.
    #[must_use]
    pub fn country_code(&self) -> Option<&str> {
        match self {
            Self::Location(val) => val
                .address
                .as_ref()
                .map(|inner| inner.country_code.as_ref()),
            _ => None,
        }
    }

    /// Helper method for nested field `custom_emoji_id`.
    #[must_use]
    pub fn custom_emoji_id(&self) -> Option<&str> {
        match self {
            Self::SuggestedReaction(val) => {
                let inner = &val.reaction_type;
                crate::types::ReactionType::custom_emoji_id(inner)
            }
            _ => None,
        }
    }

    /// Helper method for nested field `state`.
    #[must_use]
    pub fn state(&self) -> Option<&str> {
        match self {
            Self::Location(val) => val
                .address
                .as_ref()
                .and_then(|inner| inner.state.as_deref()),
            _ => None,
        }
    }

    /// Helper method for nested field `street`.
    #[must_use]
    pub fn street(&self) -> Option<&str> {
        match self {
            Self::Location(val) => val
                .address
                .as_ref()
                .and_then(|inner| inner.street.as_deref()),
            _ => None,
        }
    }
}
impl From<crate::types::StoryAreaTypeLocation> for StoryAreaType {
    fn from(val: crate::types::StoryAreaTypeLocation) -> Self {
        Self::Location(val)
    }
}
impl TryFrom<StoryAreaType> for crate::types::StoryAreaTypeLocation {
    type Error = crate::errors::ConvertToTypeError;

    fn try_from(val: StoryAreaType) -> Result<Self, Self::Error> {
        if let StoryAreaType::Location(inner) = val {
            Ok(inner)
        } else {
            Err(Self::Error::new(
                stringify!(StoryAreaType),
                stringify!(StoryAreaTypeLocation),
            ))
        }
    }
}
impl From<crate::types::StoryAreaTypeSuggestedReaction> for StoryAreaType {
    fn from(val: crate::types::StoryAreaTypeSuggestedReaction) -> Self {
        Self::SuggestedReaction(val)
    }
}
impl TryFrom<StoryAreaType> for crate::types::StoryAreaTypeSuggestedReaction {
    type Error = crate::errors::ConvertToTypeError;

    fn try_from(val: StoryAreaType) -> Result<Self, Self::Error> {
        if let StoryAreaType::SuggestedReaction(inner) = val {
            Ok(inner)
        } else {
            Err(Self::Error::new(
                stringify!(StoryAreaType),
                stringify!(StoryAreaTypeSuggestedReaction),
            ))
        }
    }
}
impl From<crate::types::StoryAreaTypeLink> for StoryAreaType {
    fn from(val: crate::types::StoryAreaTypeLink) -> Self {
        Self::Link(val)
    }
}
impl TryFrom<StoryAreaType> for crate::types::StoryAreaTypeLink {
    type Error = crate::errors::ConvertToTypeError;

    fn try_from(val: StoryAreaType) -> Result<Self, Self::Error> {
        if let StoryAreaType::Link(inner) = val {
            Ok(inner)
        } else {
            Err(Self::Error::new(
                stringify!(StoryAreaType),
                stringify!(StoryAreaTypeLink),
            ))
        }
    }
}
impl From<crate::types::StoryAreaTypeWeather> for StoryAreaType {
    fn from(val: crate::types::StoryAreaTypeWeather) -> Self {
        Self::Weather(val)
    }
}
impl TryFrom<StoryAreaType> for crate::types::StoryAreaTypeWeather {
    type Error = crate::errors::ConvertToTypeError;

    fn try_from(val: StoryAreaType) -> Result<Self, Self::Error> {
        if let StoryAreaType::Weather(inner) = val {
            Ok(inner)
        } else {
            Err(Self::Error::new(
                stringify!(StoryAreaType),
                stringify!(StoryAreaTypeWeather),
            ))
        }
    }
}
impl From<crate::types::StoryAreaTypeUniqueGift> for StoryAreaType {
    fn from(val: crate::types::StoryAreaTypeUniqueGift) -> Self {
        Self::UniqueGift(val)
    }
}
impl TryFrom<StoryAreaType> for crate::types::StoryAreaTypeUniqueGift {
    type Error = crate::errors::ConvertToTypeError;

    fn try_from(val: StoryAreaType) -> Result<Self, Self::Error> {
        if let StoryAreaType::UniqueGift(inner) = val {
            Ok(inner)
        } else {
            Err(Self::Error::new(
                stringify!(StoryAreaType),
                stringify!(StoryAreaTypeUniqueGift),
            ))
        }
    }
}
