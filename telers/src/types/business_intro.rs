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
    pub fn title<T: Into<Box<str>>>(self, val: T) -> Self {
        let mut this = self;
        this.title = Some(val.into());
        this
    }

    /// Title text of the business intro
    #[must_use]
    pub fn title_option<T: Into<Box<str>>>(self, val: Option<T>) -> Self {
        let mut this = self;
        this.title = val.map(Into::into);
        this
    }

    /// Message text of the business intro
    #[must_use]
    pub fn message<T: Into<Box<str>>>(self, val: T) -> Self {
        let mut this = self;
        this.message = Some(val.into());
        this
    }

    /// Message text of the business intro
    #[must_use]
    pub fn message_option<T: Into<Box<str>>>(self, val: Option<T>) -> Self {
        let mut this = self;
        this.message = val.map(Into::into);
        this
    }

    /// Sticker of the business intro
    #[must_use]
    pub fn sticker<T: Into<crate::types::Sticker>>(self, val: T) -> Self {
        let mut this = self;
        this.sticker = Some(Box::new(val.into()));
        this
    }

    /// Sticker of the business intro
    #[must_use]
    pub fn sticker_option<T: Into<crate::types::Sticker>>(self, val: Option<T>) -> Self {
        let mut this = self;
        this.sticker = val.map(|val| Box::new(val.into()));
        this
    }
}
impl Default for BusinessIntro {
    fn default() -> Self {
        Self::new()
    }
}
