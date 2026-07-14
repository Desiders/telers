use serde::{Deserialize, Serialize};
/// A block with a video, corresponding to the HTML tag <`video`>.
/// # Documentation
/// <https://core.telegram.org/bots/api#inputrichblockvideo>
#[derive(Clone, Debug, Serialize, Deserialize)]
pub struct InputRichBlockVideo {
    /// The video. Caption is ignored.
    pub video: crate::types::InputMediaVideo,
    /// Caption of the block
    #[serde(skip_serializing_if = "Option::is_none")]
    pub caption: Option<crate::types::RichBlockCaption>,
}
impl InputRichBlockVideo {
    /// Creates a new `InputRichBlockVideo`.
    ///
    /// # Arguments
    /// * `video` - The video. Caption is ignored.
    ///
    /// # Notes
    /// Use builder methods to set optional fields.
    #[must_use]
    pub fn new<T0: Into<crate::types::InputMediaVideo>>(video: T0) -> Self {
        Self {
            video: video.into(),
            caption: None,
        }
    }

    /// The video. Caption is ignored.
    #[must_use]
    pub fn video<T: Into<crate::types::InputMediaVideo>>(mut self, val: T) -> Self {
        self.video = val.into();
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
