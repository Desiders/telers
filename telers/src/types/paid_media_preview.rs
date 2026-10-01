/// The paid media isn't available before the payment.
/// # Documentation
/// <https://core.telegram.org/bots/api#paidmediapreview>
#[derive(Clone, Debug)]
#[cfg_attr(feature = "deser", derive(deser::Serialize, deser::Deserialize))]
#[cfg_attr(not(feature = "deser"), derive(serde::Serialize, serde::Deserialize))]
pub struct PaidMediaPreview {
    /// Media width as defined by the sender
    #[cfg_attr(not(feature = "deser"), serde(skip_serializing_if = "Option::is_none"))]
    #[cfg_attr(feature = "deser", deser(skip_serializing_if = Option::is_none))]
    pub width: Option<i64>,
    /// Media height as defined by the sender
    #[cfg_attr(not(feature = "deser"), serde(skip_serializing_if = "Option::is_none"))]
    #[cfg_attr(feature = "deser", deser(skip_serializing_if = Option::is_none))]
    pub height: Option<i64>,
    /// Duration of the media in seconds as defined by the sender
    #[cfg_attr(not(feature = "deser"), serde(skip_serializing_if = "Option::is_none"))]
    #[cfg_attr(feature = "deser", deser(skip_serializing_if = Option::is_none))]
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
    pub fn width<T: Into<i64>>(mut self, val: T) -> Self {
        self.width = Some(val.into());
        self
    }

    /// Media width as defined by the sender
    #[must_use]
    pub fn width_option<T: Into<i64>>(mut self, val: Option<T>) -> Self {
        self.width = val.map(Into::into);
        self
    }

    /// Media height as defined by the sender
    #[must_use]
    pub fn height<T: Into<i64>>(mut self, val: T) -> Self {
        self.height = Some(val.into());
        self
    }

    /// Media height as defined by the sender
    #[must_use]
    pub fn height_option<T: Into<i64>>(mut self, val: Option<T>) -> Self {
        self.height = val.map(Into::into);
        self
    }

    /// Duration of the media in seconds as defined by the sender
    #[must_use]
    pub fn duration<T: Into<i64>>(mut self, val: T) -> Self {
        self.duration = Some(val.into());
        self
    }

    /// Duration of the media in seconds as defined by the sender
    #[must_use]
    pub fn duration_option<T: Into<i64>>(mut self, val: Option<T>) -> Self {
        self.duration = val.map(Into::into);
        self
    }
}
impl Default for PaidMediaPreview {
    fn default() -> Self {
        Self::new()
    }
}
