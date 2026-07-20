use serde::{Deserialize, Serialize};
/// A block with a map, corresponding to the custom HTML tag `<tg-map>`.
/// # Documentation
/// <https://core.telegram.org/bots/api#richblockmap>
#[derive(Clone, Debug, Serialize, Deserialize)]
pub struct RichBlockMap {
    /// Location of the center of the map
    pub location: crate::types::Location,
    /// Map zoom level; 13-20
    pub zoom: u8,
    /// Expected width of the map
    pub width: i64,
    /// Expected height of the map
    pub height: i64,
    /// Caption of the block
    #[serde(skip_serializing_if = "Option::is_none")]
    pub caption: Option<crate::types::RichBlockCaption>,
}
impl RichBlockMap {
    /// Creates a new `RichBlockMap`.
    ///
    /// # Arguments
    /// * `location` - Location of the center of the map
    /// * `zoom` - Map zoom level; 13-20
    /// * `width` - Expected width of the map
    /// * `height` - Expected height of the map
    ///
    /// # Notes
    /// Use builder methods to set optional fields.
    #[must_use]
    pub fn new<T0: Into<crate::types::Location>, T1: Into<u8>, T2: Into<i64>, T3: Into<i64>>(
        location: T0,
        zoom: T1,
        width: T2,
        height: T3,
    ) -> Self {
        Self {
            location: location.into(),
            zoom: zoom.into(),
            width: width.into(),
            height: height.into(),
            caption: None,
        }
    }

    /// Location of the center of the map
    #[must_use]
    pub fn location<T: Into<crate::types::Location>>(mut self, val: T) -> Self {
        self.location = val.into();
        self
    }

    /// Map zoom level; 13-20
    #[must_use]
    pub fn zoom<T: Into<u8>>(mut self, val: T) -> Self {
        self.zoom = val.into();
        self
    }

    /// Expected width of the map
    #[must_use]
    pub fn width<T: Into<i64>>(mut self, val: T) -> Self {
        self.width = val.into();
        self
    }

    /// Expected height of the map
    #[must_use]
    pub fn height<T: Into<i64>>(mut self, val: T) -> Self {
        self.height = val.into();
        self
    }

    /// Caption of the block
    #[must_use]
    pub fn caption<T: Into<crate::types::RichBlockCaption>>(mut self, val: T) -> Self {
        self.caption = Some(val.into());
        self
    }

    /// Caption of the block
    #[must_use]
    pub fn caption_option<T: Into<crate::types::RichBlockCaption>>(
        mut self,
        val: Option<T>,
    ) -> Self {
        self.caption = val.map(Into::into);
        self
    }
}
