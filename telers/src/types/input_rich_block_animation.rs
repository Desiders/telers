use serde::{Deserialize, Serialize};
/// A block with an animation, corresponding to the HTML tag <`video`>.
/// # Documentation
/// <https://core.telegram.org/bots/api#inputrichblockanimation>
#[derive(Clone, Debug, Serialize, Deserialize)]
pub struct InputRichBlockAnimation {
    /// The animation. Caption is ignored.
    pub animation: crate::types::InputMediaAnimation,
    /// Caption of the block
    #[serde(skip_serializing_if = "Option::is_none")]
    pub caption: Option<crate::types::RichBlockCaption>,
}
impl InputRichBlockAnimation {
    /// Creates a new `InputRichBlockAnimation`.
    ///
    /// # Arguments
    /// * `animation` - The animation. Caption is ignored.
    ///
    /// # Notes
    /// Use builder methods to set optional fields.
    #[must_use]
    pub fn new<T0: Into<crate::types::InputMediaAnimation>>(animation: T0) -> Self {
        Self {
            animation: animation.into(),
            caption: None,
        }
    }

    /// The animation. Caption is ignored.
    #[must_use]
    pub fn animation<T: Into<crate::types::InputMediaAnimation>>(mut self, val: T) -> Self {
        self.animation = val.into();
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
