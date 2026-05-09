use serde::{Deserialize, Serialize};
/// The background is a .PNG or .TGV (gzipped subset of SVG with MIME type `application/x-tgwallpattern`) pattern to be combined with the background fill chosen by the user.
/// # Documentation
/// <https://core.telegram.org/bots/api#backgroundtypepattern>
#[derive(Clone, Debug, Serialize, Deserialize)]
pub struct BackgroundTypePattern {
    /// Document with the pattern
    pub document: Box<crate::types::Document>,
    /// The background fill that is combined with the pattern
    pub fill: crate::types::BackgroundFill,
    /// Intensity of the pattern when it is shown above the filled background; 0-100
    pub intensity: u8,
    /// `true`, if the background fill must be applied only to the pattern itself. All other pixels are black in this case. For dark themes only
    #[serde(skip_serializing_if = "Option::is_none")]
    pub is_inverted: Option<bool>,
    /// `true`, if the background moves slightly when the device is tilted
    #[serde(skip_serializing_if = "Option::is_none")]
    pub is_moving: Option<bool>,
}
impl BackgroundTypePattern {
    /// Creates a new `BackgroundTypePattern`.
    ///
    /// # Arguments
    /// * `document` - Document with the pattern
    /// * `fill` - The background fill that is combined with the pattern
    /// * `intensity` - Intensity of the pattern when it is shown above the filled background; 0-100
    ///
    /// # Notes
    /// Use builder methods to set optional fields.
    #[must_use]
    pub fn new<
        T0: Into<crate::types::Document>,
        T1: Into<crate::types::BackgroundFill>,
        T2: Into<u8>,
    >(
        document: T0,
        fill: T1,
        intensity: T2,
    ) -> Self {
        Self {
            document: Box::new(document.into()),
            fill: fill.into(),
            intensity: intensity.into(),
            is_inverted: None,
            is_moving: None,
        }
    }

    /// Document with the pattern
    #[must_use]
    pub fn document<T: Into<crate::types::Document>>(mut self, val: T) -> Self {
        self.document = Box::new(val.into());
        self
    }

    /// The background fill that is combined with the pattern
    #[must_use]
    pub fn fill<T: Into<crate::types::BackgroundFill>>(mut self, val: T) -> Self {
        self.fill = val.into();
        self
    }

    /// Intensity of the pattern when it is shown above the filled background; 0-100
    #[must_use]
    pub fn intensity<T: Into<u8>>(mut self, val: T) -> Self {
        self.intensity = val.into();
        self
    }

    /// `true`, if the background fill must be applied only to the pattern itself. All other pixels are black in this case. For dark themes only
    #[must_use]
    pub fn is_inverted<T: Into<bool>>(mut self, val: T) -> Self {
        self.is_inverted = Some(val.into());
        self
    }

    /// `true`, if the background fill must be applied only to the pattern itself. All other pixels are black in this case. For dark themes only
    #[must_use]
    pub fn is_inverted_option<T: Into<bool>>(mut self, val: Option<T>) -> Self {
        self.is_inverted = val.map(Into::into);
        self
    }

    /// `true`, if the background moves slightly when the device is tilted
    #[must_use]
    pub fn is_moving<T: Into<bool>>(mut self, val: T) -> Self {
        self.is_moving = Some(val.into());
        self
    }

    /// `true`, if the background moves slightly when the device is tilted
    #[must_use]
    pub fn is_moving_option<T: Into<bool>>(mut self, val: Option<T>) -> Self {
        self.is_moving = val.map(Into::into);
        self
    }
}
