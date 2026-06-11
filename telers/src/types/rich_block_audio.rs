use serde::{Deserialize, Serialize};
/// A block with a music file, corresponding to the HTML tag <`audio`>.
/// # Documentation
/// <https://core.telegram.org/bots/api#richblockaudio>
#[derive(Clone, Debug, Serialize, Deserialize)]
pub struct RichBlockAudio {
    /// The audio
    pub audio: Box<crate::types::Audio>,
    /// Caption of the block
    #[serde(skip_serializing_if = "Option::is_none")]
    pub caption: Option<crate::types::RichBlockCaption>,
}
impl RichBlockAudio {
    /// Creates a new `RichBlockAudio`.
    ///
    /// # Arguments
    /// * `audio` - The audio
    ///
    /// # Notes
    /// Use builder methods to set optional fields.
    #[must_use]
    pub fn new<T0: Into<crate::types::Audio>>(audio: T0) -> Self {
        Self {
            audio: Box::new(audio.into()),
            caption: None,
        }
    }

    /// The audio
    #[must_use]
    pub fn audio<T: Into<crate::types::Audio>>(mut self, val: T) -> Self {
        self.audio = Box::new(val.into());
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
