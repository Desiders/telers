use serde::{Deserialize, Serialize};
use serde_with::skip_serializing_none;

/// Describes the options used for link preview generation.
/// # Documentation
/// <https://core.telegram.org/bots/api#linkpreviewoptions>
#[skip_serializing_none]
#[derive(Debug, Default, Clone, Hash, PartialEq, Eq, Deserialize, Serialize)]
pub struct LinkPreviewOptions {
    /// `true`, if the link preview is disabled
    pub is_disabled: Option<bool>,
    /// URL to use for the link preview. If empty, then the first URL found in the message text will be used
    pub url: Option<String>,
    /// `true`, if the media in the link preview is suppposed to be shrunk; ignored if the URL isn't explicitly specified or media size change isn't supported for the preview
    pub prefer_small_media: Option<bool>,
    /// `true`, if the media in the link preview is suppposed to be enlarged; ignored if the URL isn't explicitly specified or media size change isn't supported for the preview
    pub prefer_large_media: Option<bool>,
    /// `true`, if the link preview must be shown above the message text; otherwise, the link preview will be shown below the message text
    pub show_above_text: Option<bool>,
}

impl LinkPreviewOptions {
    #[must_use]
    pub const fn new() -> Self {
        Self {
            is_disabled: None,
            url: None,
            prefer_small_media: None,
            prefer_large_media: None,
            show_above_text: None,
        }
    }

    #[must_use]
    pub fn is_disabled(self, val: bool) -> Self {
        Self {
            is_disabled: Some(val),
            ..self
        }
    }

    #[must_use]
    pub fn url(self, val: impl Into<String>) -> Self {
        Self {
            url: Some(val.into()),
            ..self
        }
    }

    #[must_use]
    pub fn prefer_small_media(self, val: bool) -> Self {
        Self {
            prefer_small_media: Some(val),
            ..self
        }
    }

    #[must_use]
    pub fn prefer_large_media(self, val: bool) -> Self {
        Self {
            prefer_large_media: Some(val),
            ..self
        }
    }

    #[must_use]
    pub fn show_above_text(self, val: bool) -> Self {
        Self {
            show_above_text: Some(val),
            ..self
        }
    }
}

impl LinkPreviewOptions {
    #[must_use]
    pub fn is_disabled_option(self, val: Option<bool>) -> Self {
        Self {
            is_disabled: val,
            ..self
        }
    }

    #[must_use]
    pub fn url_option(self, val: Option<impl Into<String>>) -> Self {
        Self {
            url: val.map(Into::into),
            ..self
        }
    }

    #[must_use]
    pub fn prefer_small_media_option(self, val: Option<bool>) -> Self {
        Self {
            prefer_small_media: val,
            ..self
        }
    }

    #[must_use]
    pub fn prefer_large_media_option(self, val: Option<bool>) -> Self {
        Self {
            prefer_large_media: val,
            ..self
        }
    }

    #[must_use]
    pub fn show_above_text_option(self, val: Option<bool>) -> Self {
        Self {
            show_above_text: val,
            ..self
        }
    }
}
