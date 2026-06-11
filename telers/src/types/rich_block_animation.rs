use serde::{Deserialize, Serialize};
/// A block with an animation, corresponding to the HTML tag <`video`>.
/// # Documentation
/// <https://core.telegram.org/bots/api#richblockanimation>
#[derive(Clone, Debug, Serialize, Deserialize)]
pub struct RichBlockAnimation {
    /// The animation
    pub animation: Box<crate::types::Animation>,
    /// `true`, if the media preview is covered by a spoiler animation
    #[serde(skip_serializing_if = "Option::is_none")]
    pub has_spoiler: Option<bool>,
    /// Caption of the block
    #[serde(skip_serializing_if = "Option::is_none")]
    pub caption: Option<crate::types::RichBlockCaption>,
}
impl RichBlockAnimation {
    /// Creates a new `RichBlockAnimation`.
    ///
    /// # Arguments
    /// * `animation` - The animation
    ///
    /// # Notes
    /// Use builder methods to set optional fields.
    #[must_use]
    pub fn new<T0: Into<crate::types::Animation>>(animation: T0) -> Self {
        Self {
            animation: Box::new(animation.into()),
            has_spoiler: None,
            caption: None,
        }
    }

    /// The animation
    #[must_use]
    pub fn animation<T: Into<crate::types::Animation>>(mut self, val: T) -> Self {
        self.animation = Box::new(val.into());
        self
    }

    /// `true`, if the media preview is covered by a spoiler animation
    #[must_use]
    pub fn has_spoiler<T: Into<bool>>(mut self, val: T) -> Self {
        self.has_spoiler = Some(val.into());
        self
    }

    /// `true`, if the media preview is covered by a spoiler animation
    #[must_use]
    pub fn has_spoiler_option<T: Into<bool>>(mut self, val: Option<T>) -> Self {
        self.has_spoiler = val.map(Into::into);
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
