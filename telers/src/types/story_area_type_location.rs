use serde::Serialize;
use serde_with::skip_serializing_none;

use crate::types::LocationAddress;

/// Describes a story area pointing to a location. Currently, a story can have up to 10 location areas.
/// # Documentation
/// <https://core.telegram.org/bots/api#storyareatypelocation>
#[skip_serializing_none]
#[derive(Debug, Clone, PartialEq, Serialize)]
pub struct StoryAreaTypeLocation {
    /// Location latitude in degrees
    pub latitude: f64,
    /// Location longitude in degrees
    pub longitude: f64,
    /// Address of the location
    pub address: Option<LocationAddress>,
}

impl StoryAreaTypeLocation {
    #[must_use]
    pub fn new(latitude: f64, longitude: f64) -> Self {
        Self {
            latitude,
            longitude,
            address: None,
        }
    }

    #[must_use]
    pub fn latitude(self, val: f64) -> Self {
        Self {
            latitude: val,
            ..self
        }
    }

    #[must_use]
    pub fn longitude(self, val: f64) -> Self {
        Self {
            longitude: val,
            ..self
        }
    }

    #[must_use]
    pub fn address(self, val: LocationAddress) -> Self {
        Self {
            address: Some(val),
            ..self
        }
    }
}

impl StoryAreaTypeLocation {
    #[must_use]
    pub fn address_optional(self, val: Option<LocationAddress>) -> Self {
        Self {
            address: val,
            ..self
        }
    }
}
