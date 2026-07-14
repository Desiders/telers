use serde::{Deserialize, Serialize};
/// A block with a music file, corresponding to the HTML tag <`audio`>.
/// # Documentation
/// <https://core.telegram.org/bots/api#inputrichblockaudio>
#[derive(Clone, Debug, Serialize, Deserialize)]
pub struct InputRichBlockAudio {
    /// The audio. Caption is ignored.
    pub audio: crate::types::InputMediaAudio,
    /// Caption of the block
    #[serde(skip_serializing_if = "Option::is_none")]
    pub caption: Option<crate::types::RichBlockCaption>,
}
impl InputRichBlockAudio {
    /// Creates a new `InputRichBlockAudio`.
    ///
    /// # Arguments
    /// * `audio` - The audio. Caption is ignored.
    ///
    /// # Notes
    /// Use builder methods to set optional fields.
    #[must_use]
    pub fn new<T0: Into<crate::types::InputMediaAudio>>(audio: T0) -> Self {
        Self {
            audio: audio.into(),
            caption: None,
        }
    }

    /// The audio. Caption is ignored.
    #[must_use]
    pub fn audio<T: Into<crate::types::InputMediaAudio>>(mut self, val: T) -> Self {
        self.audio = val.into();
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
