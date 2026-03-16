use serde::{Deserialize, Serialize};
/// Contains parameters of a post that is being suggested by the bot.
/// # Documentation
/// <https://core.telegram.org/bots/api#suggestedpostparameters>
#[derive(Clone, Debug, Serialize, Deserialize)]
pub struct SuggestedPostParameters {
    /// Proposed price for the post. If the field is omitted, then the post is unpaid.
    #[serde(skip_serializing_if = "Option::is_none")]
    pub price: Option<crate::types::SuggestedPostPrice>,
    /// Proposed send date of the post. If specified, then the date must be between 300 second and 2678400 seconds (30 days) in the future. If the field is omitted, then the post can be published at any time within 30 days at the sole discretion of the user who approves it.
    #[serde(skip_serializing_if = "Option::is_none")]
    pub send_date: Option<i64>,
}
impl SuggestedPostParameters {
    /// Creates a new `SuggestedPostParameters`.
    ///
    /// # Notes
    /// Use builder methods to set optional fields.
    #[must_use]
    pub fn new() -> Self {
        Self {
            price: None,
            send_date: None,
        }
    }

    /// Proposed price for the post. If the field is omitted, then the post is unpaid.
    #[must_use]
    pub fn price<T: Into<crate::types::SuggestedPostPrice>>(self, val: T) -> Self {
        let mut this = self;
        this.price = Some(val.into());
        this
    }

    /// Proposed price for the post. If the field is omitted, then the post is unpaid.
    #[must_use]
    pub fn price_option<T: Into<crate::types::SuggestedPostPrice>>(self, val: Option<T>) -> Self {
        let mut this = self;
        this.price = val.map(Into::into);
        this
    }

    /// Proposed send date of the post. If specified, then the date must be between 300 second and 2678400 seconds (30 days) in the future. If the field is omitted, then the post can be published at any time within 30 days at the sole discretion of the user who approves it.
    #[must_use]
    pub fn send_date<T: Into<i64>>(self, val: T) -> Self {
        let mut this = self;
        this.send_date = Some(val.into());
        this
    }

    /// Proposed send date of the post. If specified, then the date must be between 300 second and 2678400 seconds (30 days) in the future. If the field is omitted, then the post can be published at any time within 30 days at the sole discretion of the user who approves it.
    #[must_use]
    pub fn send_date_option<T: Into<i64>>(self, val: Option<T>) -> Self {
        let mut this = self;
        this.send_date = val.map(Into::into);
        this
    }
}
impl Default for SuggestedPostParameters {
    fn default() -> Self {
        Self::new()
    }
}
