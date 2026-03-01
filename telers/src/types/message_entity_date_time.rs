use serde::{Deserialize, Serialize};
/// This object represents a/an date time message entity.
/// # Notes
/// This object represents a message entity from original field `date_time`.
/// # Documentation
/// <https://core.telegram.org/bots/api#messageentity>
#[derive(Clone, Debug, Serialize, Deserialize)]
pub struct MessageEntityDateTime {
    /// Offset in UTF-16 code units to the start of the entity
    pub offset: i64,
    /// Length of the entity in UTF-16 code units
    pub length: i64,
    /// For `date_time` only, the Unix time associated with the entity
    pub unix_time: i64,
    /// For `date_time` only, the string that defines the formatting of the date and time. See date-time entity formatting for more details.
    #[serde(skip_serializing_if = "Option::is_none")]
    pub date_time_format: Option<Box<str>>,
}
impl MessageEntityDateTime {
    /// Creates a new `MessageEntityDateTime`.
    ///
    /// # Arguments
    /// * `offset` - Offset in UTF-16 code units to the start of the entity
    /// * `length` - Length of the entity in UTF-16 code units
    /// * `unix_time` - For `date_time` only, the Unix time associated with the entity
    ///
    /// # Notes
    /// Use builder methods to set optional fields.
    #[must_use]
    pub fn new<T0: Into<i64>, T1: Into<i64>, T2: Into<i64>>(
        offset: T0,
        length: T1,
        unix_time: T2,
    ) -> Self {
        Self {
            offset: offset.into(),
            length: length.into(),
            unix_time: unix_time.into(),
            date_time_format: None,
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

    /// For `date_time` only, the Unix time associated with the entity
    #[must_use]
    pub fn unix_time<T: Into<i64>>(self, val: T) -> Self {
        let mut this = self;
        this.unix_time = val.into();
        this
    }

    /// For `date_time` only, the string that defines the formatting of the date and time. See date-time entity formatting for more details.
    #[must_use]
    pub fn date_time_format<T: Into<Box<str>>>(self, val: T) -> Self {
        let mut this = self;
        this.date_time_format = Some(val.into());
        this
    }

    /// For `date_time` only, the string that defines the formatting of the date and time. See date-time entity formatting for more details.
    #[must_use]
    pub fn date_time_format_option<T: Into<Box<str>>>(self, val: Option<T>) -> Self {
        let mut this = self;
        this.date_time_format = val.map(Into::into);
        this
    }
}
