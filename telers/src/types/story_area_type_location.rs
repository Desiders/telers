use serde::{Deserialize, Serialize};
/// Describes a story area pointing to a location. Currently, a story can have up to 10 location areas.
/// # Documentation
/// <https://core.telegram.org/bots/api#storyareatypelocation>
#[derive(Clone, Debug, Serialize, Deserialize)]
pub struct StoryAreaTypeLocation {
    /// Location latitude in degrees
    pub latitude: f64,
    /// Location longitude in degrees
    pub longitude: f64,
    /// Address of the location
    #[serde(skip_serializing_if = "Option::is_none")]
    pub address: Option<crate::types::LocationAddress>,
}
impl StoryAreaTypeLocation {
    /// Creates a new `StoryAreaTypeLocation`.
    ///
    /// # Arguments
    /// * `latitude` - Location latitude in degrees
    /// * `longitude` - Location longitude in degrees
    ///
    /// # Notes
    /// Use builder methods to set optional fields.
    #[must_use]
    pub fn new<T0: Into<f64>, T1: Into<f64>>(latitude: T0, longitude: T1) -> Self {
        Self {
            latitude: latitude.into(),
            longitude: longitude.into(),
            address: None,
        }
    }

    /// Location latitude in degrees
    #[must_use]
    pub fn latitude<T: Into<f64>>(mut self, val: T) -> Self {
        self.latitude = val.into();
        self
    }

    /// Location longitude in degrees
    #[must_use]
    pub fn longitude<T: Into<f64>>(mut self, val: T) -> Self {
        self.longitude = val.into();
        self
    }

    /// Address of the location
    #[must_use]
    pub fn address<T: Into<crate::types::LocationAddress>>(mut self, val: T) -> Self {
        self.address = Some(val.into());
        self
    }

    /// Address of the location
    #[must_use]
    pub fn address_option<T: Into<crate::types::LocationAddress>>(
        mut self,
        val: Option<T>,
    ) -> Self {
        self.address = val.map(Into::into);
        self
    }
}
