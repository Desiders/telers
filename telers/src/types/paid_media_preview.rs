use serde::{Deserialize, Serialize};
/// The paid media isn't available before the payment.
/// # Documentation
/// <https://core.telegram.org/bots/api#paidmediapreview>
#[derive(Clone, Debug, Serialize, Deserialize)]
pub struct PaidMediaPreview {
    /// Media width as defined by the sender
    #[serde(skip_serializing_if = "Option::is_none")]
    pub width: Option<i64>,
    /// Media height as defined by the sender
    #[serde(skip_serializing_if = "Option::is_none")]
    pub height: Option<i64>,
    /// Duration of the media in seconds as defined by the sender
    #[serde(skip_serializing_if = "Option::is_none")]
    pub duration: Option<i64>,
}
impl PaidMediaPreview {
    /// Creates a new `PaidMediaPreview`.
    ///
    /// # Notes
    /// Use builder methods to set optional fields.
    #[must_use]
    pub fn new() -> Self {
        Self {
            width: None,
            height: None,
            duration: None,
        }
    }

    /// Media width as defined by the sender
    #[must_use]
    pub fn width<T: Into<i64>>(self, val: T) -> Self {
        let mut this = self;
        this.width = Some(val.into());
        this
    }

    /// Media width as defined by the sender
    #[must_use]
    pub fn width_option<T: Into<i64>>(self, val: Option<T>) -> Self {
        let mut this = self;
        this.width = val.map(Into::into);
        this
    }

    /// Media height as defined by the sender
    #[must_use]
    pub fn height<T: Into<i64>>(self, val: T) -> Self {
        let mut this = self;
        this.height = Some(val.into());
        this
    }

    /// Media height as defined by the sender
    #[must_use]
    pub fn height_option<T: Into<i64>>(self, val: Option<T>) -> Self {
        let mut this = self;
        this.height = val.map(Into::into);
        this
    }

    /// Duration of the media in seconds as defined by the sender
    #[must_use]
    pub fn duration<T: Into<i64>>(self, val: T) -> Self {
        let mut this = self;
        this.duration = Some(val.into());
        this
    }

    /// Duration of the media in seconds as defined by the sender
    #[must_use]
    pub fn duration_option<T: Into<i64>>(self, val: Option<T>) -> Self {
        let mut this = self;
        this.duration = val.map(Into::into);
        this
    }
}
impl Default for PaidMediaPreview {
    fn default() -> Self {
        Self::new()
    }
}
