use serde::{Deserialize, Serialize};
/// A block with a voice note, corresponding to the HTML tag <`audio`>.
/// # Documentation
/// <https://core.telegram.org/bots/api#inputrichblockvoicenote>
#[derive(Clone, Debug, Serialize, Deserialize)]
pub struct InputRichBlockVoiceNote {
    /// The voice note. Caption is ignored.
    pub voice_note: crate::types::InputMediaVoiceNote,
    /// Caption of the block
    #[serde(skip_serializing_if = "Option::is_none")]
    pub caption: Option<crate::types::RichBlockCaption>,
}
impl InputRichBlockVoiceNote {
    /// Creates a new `InputRichBlockVoiceNote`.
    ///
    /// # Arguments
    /// * `voice_note` - The voice note. Caption is ignored.
    ///
    /// # Notes
    /// Use builder methods to set optional fields.
    #[must_use]
    pub fn new<T0: Into<crate::types::InputMediaVoiceNote>>(voice_note: T0) -> Self {
        Self {
            voice_note: voice_note.into(),
            caption: None,
        }
    }

    /// The voice note. Caption is ignored.
    #[must_use]
    pub fn voice_note<T: Into<crate::types::InputMediaVoiceNote>>(mut self, val: T) -> Self {
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
