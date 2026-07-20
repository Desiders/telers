use serde::{Deserialize, Serialize};
/// A block with a map, corresponding to the custom HTML tag `<tg-map>`. The map's width and height must not exceed 10000 in total. The width and height ratio must be at most 20.
/// # Documentation
/// <https://core.telegram.org/bots/api#inputrichblockmap>
#[derive(Clone, Debug, Serialize, Deserialize)]
pub struct InputRichBlockMap {
    /// Location of the center of the map
    pub location: crate::types::Location,
    /// Map zoom level; 0-24
    pub zoom: u8,
    /// Map width; 0-10000
    pub width: u16,
    /// Map height; 0-10000
    pub height: u16,
    /// Caption of the block
    #[serde(skip_serializing_if = "Option::is_none")]
    pub caption: Option<crate::types::RichBlockCaption>,
}
impl InputRichBlockMap {
    /// Creates a new `InputRichBlockMap`.
    ///
    /// # Arguments
    /// * `location` - Location of the center of the map
    /// * `zoom` - Map zoom level; 0-24
    /// * `width` - Map width; 0-10000
    /// * `height` - Map height; 0-10000
    ///
    /// # Notes
    /// Use builder methods to set optional fields.
    #[must_use]
    pub fn new<T0: Into<crate::types::Location>, T1: Into<u8>, T2: Into<u16>, T3: Into<u16>>(
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

    /// Map zoom level; 0-24
    #[must_use]
    pub fn zoom<T: Into<u8>>(mut self, val: T) -> Self {
        self.zoom = val.into();
        self
    }

    /// Map width; 0-10000
    #[must_use]
    pub fn width<T: Into<u16>>(mut self, val: T) -> Self {
        self.width = val.into();
        self
    }

    /// Map height; 0-10000
    #[must_use]
    pub fn height<T: Into<u16>>(mut self, val: T) -> Self {
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
