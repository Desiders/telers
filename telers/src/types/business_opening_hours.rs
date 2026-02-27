use serde::{Deserialize, Serialize};
/// Describes the opening hours of a business.
/// # Documentation
/// <https://core.telegram.org/bots/api#businessopeninghours>
#[derive(Clone, Debug, Serialize, Deserialize)]
pub struct BusinessOpeningHours {
    /// Unique name of the time zone for which the opening hours are defined
    pub time_zone_name: Box<str>,
    /// List of time intervals describing business opening hours
    pub opening_hours: Box<[crate::types::BusinessOpeningHoursInterval]>,
}
impl BusinessOpeningHours {
    /// Creates a new `BusinessOpeningHours`.
    ///
    /// # Arguments
    /// * `time_zone_name` - Unique name of the time zone for which the opening hours are defined
    /// * `opening_hours` - List of time intervals describing business opening hours
    #[must_use]
    pub fn new<
        T0: Into<Box<str>>,
        T1Item: Into<crate::types::BusinessOpeningHoursInterval>,
        T1: IntoIterator<Item = T1Item>,
    >(
        time_zone_name: T0,
        opening_hours: T1,
    ) -> Self {
        Self {
            time_zone_name: time_zone_name.into(),
            opening_hours: opening_hours.into_iter().map(Into::into).collect(),
        }
    }

    /// Unique name of the time zone for which the opening hours are defined
    #[must_use]
    pub fn time_zone_name<T: Into<Box<str>>>(self, val: T) -> Self {
        let mut this = self;
        this.time_zone_name = val.into();
        this
    }

    /// List of time intervals describing business opening hours
    ///
    /// # Notes
    /// Adds multiple elements.
    #[must_use]
    pub fn opening_hours<T: Into<Box<[crate::types::BusinessOpeningHoursInterval]>>>(
        self,
        val: T,
    ) -> Self {
        let mut this = self;
        this.opening_hours = this
            .opening_hours
            .into_vec()
            .into_iter()
            .chain(val.into())
            .collect();
        this
    }

    /// List of time intervals describing business opening hours
    ///
    /// # Notes
    /// Adds a single element.
    #[must_use]
    pub fn opening_hour<T: Into<crate::types::BusinessOpeningHoursInterval>>(self, val: T) -> Self {
        let mut this = self;
        this.opening_hours = this
            .opening_hours
            .into_vec()
            .into_iter()
            .chain(Some(val.into()))
            .collect();
        this
    }
}
