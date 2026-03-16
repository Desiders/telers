use serde::{Deserialize, Serialize};
/// Contains information about a suggested post.
/// # Documentation
/// <https://core.telegram.org/bots/api#suggestedpostinfo>
#[derive(Clone, Debug, Serialize, Deserialize)]
pub struct SuggestedPostInfo {
    /// State of the suggested post. Currently, it can be one of `pending`, `approved`, `declined`.
    pub state: Box<str>,
    /// Proposed price of the post. If the field is omitted, then the post is unpaid.
    #[serde(skip_serializing_if = "Option::is_none")]
    pub price: Option<crate::types::SuggestedPostPrice>,
    /// Proposed send date of the post. If the field is omitted, then the post can be published at any time within 30 days at the sole discretion of the user or administrator who approves it.
    #[serde(skip_serializing_if = "Option::is_none")]
    pub send_date: Option<i64>,
}
impl SuggestedPostInfo {
    /// Creates a new `SuggestedPostInfo`.
    ///
    /// # Arguments
    /// * `state` - State of the suggested post. Currently, it can be one of `pending`, `approved`, `declined`.
    ///
    /// # Notes
    /// Use builder methods to set optional fields.
    #[must_use]
    pub fn new<T0: Into<Box<str>>>(state: T0) -> Self {
        Self {
            state: state.into(),
            price: None,
            send_date: None,
        }
    }

    /// State of the suggested post. Currently, it can be one of `pending`, `approved`, `declined`.
    #[must_use]
    pub fn state<T: Into<Box<str>>>(self, val: T) -> Self {
        let mut this = self;
        this.state = val.into();
        this
    }

    /// Proposed price of the post. If the field is omitted, then the post is unpaid.
    #[must_use]
    pub fn price<T: Into<crate::types::SuggestedPostPrice>>(self, val: T) -> Self {
        let mut this = self;
        this.price = Some(val.into());
        this
    }

    /// Proposed price of the post. If the field is omitted, then the post is unpaid.
    #[must_use]
    pub fn price_option<T: Into<crate::types::SuggestedPostPrice>>(self, val: Option<T>) -> Self {
        let mut this = self;
        this.price = val.map(Into::into);
        this
    }

    /// Proposed send date of the post. If the field is omitted, then the post can be published at any time within 30 days at the sole discretion of the user or administrator who approves it.
    #[must_use]
    pub fn send_date<T: Into<i64>>(self, val: T) -> Self {
        let mut this = self;
        this.send_date = Some(val.into());
        this
    }

    /// Proposed send date of the post. If the field is omitted, then the post can be published at any time within 30 days at the sole discretion of the user or administrator who approves it.
    #[must_use]
    pub fn send_date_option<T: Into<i64>>(self, val: Option<T>) -> Self {
        let mut this = self;
        this.send_date = val.map(Into::into);
        this
    }
}
