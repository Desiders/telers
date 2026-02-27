use serde::{Deserialize, Serialize};
/// This object represents a/an text link message entity.
/// # Notes
/// This object represents a message entity from original field `text_link`.
/// # Documentation
/// <https://core.telegram.org/bots/api#messageentity>
#[derive(Clone, Debug, Serialize, Deserialize)]
pub struct MessageEntityTextLink {
    /// Offset in UTF-16 code units to the start of the entity
    pub offset: i64,
    /// Length of the entity in UTF-16 code units
    pub length: i64,
    /// For `text_link` only, URL that will be opened after user taps on the text
    pub url: Box<str>,
}
impl MessageEntityTextLink {
    /// Creates a new `MessageEntityTextLink`.
    ///
    /// # Arguments
    /// * `offset` - Offset in UTF-16 code units to the start of the entity
    /// * `length` - Length of the entity in UTF-16 code units
    /// * `url` - For `text_link` only, URL that will be opened after user taps on the text
    #[must_use]
    pub fn new<T0: Into<i64>, T1: Into<i64>, T2: Into<Box<str>>>(
        offset: T0,
        length: T1,
        url: T2,
    ) -> Self {
        Self {
            offset: offset.into(),
            length: length.into(),
            url: url.into(),
        }
    }

    /// Offset in UTF-16 code units to the start of the entity
    #[must_use]
    pub fn offset<T: Into<i64>>(self, val: T) -> Self {
        let mut this = self;
        this.offset = val.into();
        this
    }

    /// Length of the entity in UTF-16 code units
    #[must_use]
    pub fn length<T: Into<i64>>(self, val: T) -> Self {
        let mut this = self;
        this.length = val.into();
        this
    }

    /// For `text_link` only, URL that will be opened after user taps on the text
    #[must_use]
    pub fn url<T: Into<Box<str>>>(self, val: T) -> Self {
        let mut this = self;
        this.url = val.into();
        this
    }
}
