use serde::{Deserialize, Serialize};
/// Contains information about the start page settings of a Telegram Business account.
/// # Documentation
/// <https://core.telegram.org/bots/api#businessintro>
#[derive(Clone, Debug, Serialize, Deserialize)]
pub struct BusinessIntro {
    /// Title text of the business intro
    #[serde(skip_serializing_if = "Option::is_none")]
    pub title: Option<Box<str>>,
    /// Message text of the business intro
    #[serde(skip_serializing_if = "Option::is_none")]
    pub message: Option<Box<str>>,
    /// Sticker of the business intro
    #[serde(skip_serializing_if = "Option::is_none")]
    pub sticker: Option<Box<crate::types::Sticker>>,
}
impl BusinessIntro {
    /// Creates a new `BusinessIntro`.
    ///
    /// # Notes
    /// Use builder methods to set optional fields.
    #[must_use]
    pub fn new() -> Self {
        Self {
            title: None,
            message: None,
            sticker: None,
        }
    }

    /// Title text of the business intro
    #[must_use]
    pub fn title<T: Into<Box<str>>>(mut self, val: T) -> Self {
        self.title = Some(val.into());
        self
    }

    /// Title text of the business intro
    #[must_use]
    pub fn title_option<T: Into<Box<str>>>(mut self, val: Option<T>) -> Self {
        self.title = val.map(Into::into);
        self
    }

    /// Message text of the business intro
    #[must_use]
    pub fn message<T: Into<Box<str>>>(mut self, val: T) -> Self {
        self.message = Some(val.into());
        self
    }

    /// Message text of the business intro
    #[must_use]
    pub fn message_option<T: Into<Box<str>>>(mut self, val: Option<T>) -> Self {
        self.message = val.map(Into::into);
        self
    }

    /// Sticker of the business intro
    #[must_use]
    pub fn sticker<T: Into<crate::types::Sticker>>(mut self, val: T) -> Self {
        self.sticker = Some(Box::new(val.into()));
        self
    }

    /// Sticker of the business intro
    #[must_use]
    pub fn sticker_option<T: Into<crate::types::Sticker>>(mut self, val: Option<T>) -> Self {
        self.sticker = val.map(|val| Box::new(val.into()));
        self
    }
}
impl Default for BusinessIntro {
    fn default() -> Self {
        Self::new()
    }
}
