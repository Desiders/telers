use serde::{Deserialize, Serialize};
/// Media is an animation, information about the animation
/// # Notes
/// This object represents a poll media from original field `animation`.
/// # Documentation
/// <https://core.telegram.org/bots/api#pollmedia>
#[derive(Clone, Debug, Serialize, Deserialize)]
pub struct PollMediaAnimation {
    /// Media is an animation, information about the animation
    pub animation: Box<crate::types::Animation>,
}
impl PollMediaAnimation {
    /// Creates a new `PollMediaAnimation`.
    ///
    /// # Arguments
    /// * `animation` - Media is an animation, information about the animation
    #[must_use]
    pub fn new<T0: Into<crate::types::Animation>>(animation: T0) -> Self {
        Self {
            animation: Box::new(animation.into()),
        }
    }

    /// Media is an animation, information about the animation
    #[must_use]
    pub fn animation<T: Into<crate::types::Animation>>(mut self, val: T) -> Self {
        self.animation = Box::new(val.into());
        self
    }
}
