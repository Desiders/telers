use serde::{Deserialize, Serialize};
/// Describes the birthdate of a user.
/// # Documentation
/// <https://core.telegram.org/bots/api#birthdate>
#[derive(Clone, Debug, Serialize, Deserialize)]
pub struct Birthdate {
    /// Day of the user's birth; 1-31
    pub day: u8,
    /// Month of the user's birth; 1-12
    pub month: u8,
    /// Year of the user's birth
    #[serde(skip_serializing_if = "Option::is_none")]
    pub year: Option<i64>,
}
impl Birthdate {
    /// Creates a new `Birthdate`.
    ///
    /// # Arguments
    /// * `day` - Day of the user's birth; 1-31
    /// * `month` - Month of the user's birth; 1-12
    ///
    /// # Notes
    /// Use builder methods to set optional fields.
    #[must_use]
    pub fn new<T0: Into<u8>, T1: Into<u8>>(day: T0, month: T1) -> Self {
        Self {
            day: day.into(),
            month: month.into(),
            year: None,
        }
    }

    /// Day of the user's birth; 1-31
    #[must_use]
    pub fn day<T: Into<u8>>(self, val: T) -> Self {
        let mut this = self;
        this.day = val.into();
        this
    }

    /// Month of the user's birth; 1-12
    #[must_use]
    pub fn month<T: Into<u8>>(self, val: T) -> Self {
        let mut this = self;
        this.month = val.into();
        this
    }

    /// Year of the user's birth
    #[must_use]
    pub fn year<T: Into<i64>>(self, val: T) -> Self {
        let mut this = self;
        this.year = Some(val.into());
        this
    }

    /// Year of the user's birth
    #[must_use]
    pub fn year_option<T: Into<i64>>(self, val: Option<T>) -> Self {
        let mut this = self;
        this.year = val.map(Into::into);
        this
    }
}
