use serde::{Deserialize, Serialize};
/// A block with a map, corresponding to the custom HTML tag `<tg-map>`. The map's width and height must not exceed 10000 in total. The width and height ratio must be at most 20.
/// # Documentation
/// <https://core.telegram.org/bots/api#inputrichblockmap>
#[derive(Clone, Debug, Serialize, Deserialize)]
pub struct InputRichBlockMap {
    /// Location of the center of the map
    pub location: crate::types::Location,
    /// Map zoom level; 0-24
    #[serde(skip_serializing_if = "Option::is_none")]
    pub zoom: Option<u8>,
    /// Map width; 0-10000
    #[serde(skip_serializing_if = "Option::is_none")]
    pub width: Option<u16>,
    /// Map height; 0-10000
    #[serde(skip_serializing_if = "Option::is_none")]
    pub height: Option<u16>,
    /// Caption of the block
    #[serde(skip_serializing_if = "Option::is_none")]
    pub caption: Option<crate::types::RichBlockCaption>,
}
impl InputRichBlockMap {
    /// Creates a new `InputRichBlockMap`.
    ///
    /// # Arguments
    /// * `location` - Location of the center of the map
    ///
    /// # Notes
    /// Use builder methods to set optional fields.
    #[must_use]
    pub fn new<T0: Into<crate::types::Location>>(location: T0) -> Self {
        Self {
            location: location.into(),
            zoom: None,
            width: None,
            height: None,
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
        self.zoom = Some(val.into());
        self
    }

    /// Map zoom level; 0-24
    #[must_use]
    pub fn zoom_option<T: Into<u8>>(mut self, val: Option<T>) -> Self {
        self.zoom = val.map(Into::into);
        self
    }

    /// Map width; 0-10000
    #[must_use]
    pub fn width<T: Into<u16>>(mut self, val: T) -> Self {
        self.width = Some(val.into());
        self
    }

    /// Map width; 0-10000
    #[must_use]
    pub fn width_option<T: Into<u16>>(mut self, val: Option<T>) -> Self {
        self.width = val.map(Into::into);
        self
    }

    /// Map height; 0-10000
    #[must_use]
    pub fn height<T: Into<u16>>(mut self, val: T) -> Self {
        self.height = Some(val.into());
        self
    }

    /// Map height; 0-10000
    #[must_use]
    pub fn height_option<T: Into<u16>>(mut self, val: Option<T>) -> Self {
        self.height = val.map(Into::into);
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
