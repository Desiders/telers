use serde::{Deserialize, Serialize};
/// Formatted date and time.
/// # Documentation
/// <https://core.telegram.org/bots/api#richtextdatetime>
#[derive(Clone, Debug, Serialize, Deserialize)]
pub struct RichTextDateTime {
    /// The text
    pub text: Box<crate::types::RichText>,
    /// The Unix time associated with the entity
    pub unix_time: i64,
    /// The string that defines the formatting of the date and time. See date-time entity formatting for more details.
    pub date_time_format: Box<str>,
}
impl RichTextDateTime {
    /// Creates a new `RichTextDateTime`.
    ///
    /// # Arguments
    /// * `text` - The text
    /// * `unix_time` - The Unix time associated with the entity
    /// * `date_time_format` - The string that defines the formatting of the date and time. See date-time entity formatting for more details.
    #[must_use]
    pub fn new<T0: Into<crate::types::RichText>, T1: Into<i64>, T2: Into<Box<str>>>(
        text: T0,
        unix_time: T1,
        date_time_format: T2,
    ) -> Self {
        Self {
            text: Box::new(text.into()),
            unix_time: unix_time.into(),
            date_time_format: date_time_format.into(),
        }
    }

    /// The text
    #[must_use]
    pub fn text<T: Into<crate::types::RichText>>(mut self, val: T) -> Self {
        self.text = Box::new(val.into());
        self
    }

    /// The Unix time associated with the entity
    #[must_use]
    pub fn unix_time<T: Into<i64>>(mut self, val: T) -> Self {
        self.unix_time = val.into();
        self
    }

    /// The string that defines the formatting of the date and time. See date-time entity formatting for more details.
    #[must_use]
    pub fn date_time_format<T: Into<Box<str>>>(mut self, val: T) -> Self {
        self.date_time_format = val.into();
        self
    }
}
