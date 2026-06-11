use serde::{Deserialize, Serialize};
/// A block with a voice note, corresponding to the HTML tag <`audio`>.
/// # Documentation
/// <https://core.telegram.org/bots/api#richblockvoicenote>
#[derive(Clone, Debug, Serialize, Deserialize)]
pub struct RichBlockVoiceNote {
    /// The voice note
    pub voice_note: crate::types::Voice,
    /// Caption of the block
    #[serde(skip_serializing_if = "Option::is_none")]
    pub caption: Option<crate::types::RichBlockCaption>,
}
impl RichBlockVoiceNote {
    /// Creates a new `RichBlockVoiceNote`.
    ///
    /// # Arguments
    /// * `voice_note` - The voice note
    ///
    /// # Notes
    /// Use builder methods to set optional fields.
    #[must_use]
    pub fn new<T0: Into<crate::types::Voice>>(voice_note: T0) -> Self {
        Self {
            voice_note: voice_note.into(),
            caption: None,
        }
    }

    /// The voice note
    #[must_use]
    pub fn voice_note<T: Into<crate::types::Voice>>(mut self, val: T) -> Self {
        self.voice_note = val.into();
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
