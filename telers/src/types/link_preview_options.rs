use serde::{Deserialize, Serialize};
/// Describes the options used for link preview generation.
/// # Documentation
/// <https://core.telegram.org/bots/api#linkpreviewoptions>
#[derive(Clone, Debug, Serialize, Deserialize)]
pub struct LinkPreviewOptions {
    /// `true`, if the link preview is disabled
    #[serde(skip_serializing_if = "Option::is_none")]
    pub is_disabled: Option<bool>,
    /// URL to use for the link preview. If empty, then the first URL found in the message text will be used.
    #[serde(skip_serializing_if = "Option::is_none")]
    pub url: Option<Box<str>>,
    /// `true`, if the media in the link preview is supposed to be shrunk; ignored if the URL isn't explicitly specified or media size change isn't supported for the preview
    #[serde(skip_serializing_if = "Option::is_none")]
    pub prefer_small_media: Option<bool>,
    /// `true`, if the media in the link preview is supposed to be enlarged; ignored if the URL isn't explicitly specified or media size change isn't supported for the preview
    #[serde(skip_serializing_if = "Option::is_none")]
    pub prefer_large_media: Option<bool>,
    /// `true`, if the link preview must be shown above the message text; otherwise, the link preview will be shown below the message text
    #[serde(skip_serializing_if = "Option::is_none")]
    pub show_above_text: Option<bool>,
}
impl LinkPreviewOptions {
    /// Creates a new `LinkPreviewOptions`.
    ///
    /// # Notes
    /// Use builder methods to set optional fields.
    #[must_use]
    pub fn new() -> Self {
        Self {
            is_disabled: None,
            url: None,
            prefer_small_media: None,
            prefer_large_media: None,
            show_above_text: None,
        }
    }

    /// `true`, if the link preview is disabled
    #[must_use]
    pub fn is_disabled<T: Into<bool>>(mut self, val: T) -> Self {
        self.is_disabled = Some(val.into());
        self
    }

    /// `true`, if the link preview is disabled
    #[must_use]
    pub fn is_disabled_option<T: Into<bool>>(mut self, val: Option<T>) -> Self {
        self.is_disabled = val.map(Into::into);
        self
    }

    /// URL to use for the link preview. If empty, then the first URL found in the message text will be used.
    #[must_use]
    pub fn url<T: Into<Box<str>>>(mut self, val: T) -> Self {
        self.url = Some(val.into());
        self
    }

    /// URL to use for the link preview. If empty, then the first URL found in the message text will be used.
    #[must_use]
    pub fn url_option<T: Into<Box<str>>>(mut self, val: Option<T>) -> Self {
        self.url = val.map(Into::into);
        self
    }

    /// `true`, if the media in the link preview is supposed to be shrunk; ignored if the URL isn't explicitly specified or media size change isn't supported for the preview
    #[must_use]
    pub fn prefer_small_media<T: Into<bool>>(mut self, val: T) -> Self {
        self.prefer_small_media = Some(val.into());
        self
    }

    /// `true`, if the media in the link preview is supposed to be shrunk; ignored if the URL isn't explicitly specified or media size change isn't supported for the preview
    #[must_use]
    pub fn prefer_small_media_option<T: Into<bool>>(mut self, val: Option<T>) -> Self {
        self.prefer_small_media = val.map(Into::into);
        self
    }

    /// `true`, if the media in the link preview is supposed to be enlarged; ignored if the URL isn't explicitly specified or media size change isn't supported for the preview
    #[must_use]
    pub fn prefer_large_media<T: Into<bool>>(mut self, val: T) -> Self {
        self.prefer_large_media = Some(val.into());
        self
    }

    /// `true`, if the media in the link preview is supposed to be enlarged; ignored if the URL isn't explicitly specified or media size change isn't supported for the preview
    #[must_use]
    pub fn prefer_large_media_option<T: Into<bool>>(mut self, val: Option<T>) -> Self {
        self.prefer_large_media = val.map(Into::into);
        self
    }

    /// `true`, if the link preview must be shown above the message text; otherwise, the link preview will be shown below the message text
    #[must_use]
    pub fn show_above_text<T: Into<bool>>(mut self, val: T) -> Self {
        self.show_above_text = Some(val.into());
        self
    }

    /// `true`, if the link preview must be shown above the message text; otherwise, the link preview will be shown below the message text
    #[must_use]
    pub fn show_above_text_option<T: Into<bool>>(mut self, val: Option<T>) -> Self {
        self.show_above_text = val.map(Into::into);
        self
    }
}
impl Default for LinkPreviewOptions {
    fn default() -> Self {
        Self::new()
    }
}
