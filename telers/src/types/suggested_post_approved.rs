use serde::{Deserialize, Serialize};
/// Describes a service message about the approval of a suggested post.
/// # Documentation
/// <https://core.telegram.org/bots/api#suggestedpostapproved>
#[derive(Clone, Debug, Serialize, Deserialize)]
pub struct SuggestedPostApproved {
    /// Message containing the suggested post. Note that the Message object in this field will not contain the `reply_to_message` field even if it itself is a reply.
    #[serde(skip_serializing_if = "Option::is_none")]
    pub suggested_post_message: Option<Box<crate::types::Message>>,
    /// Amount paid for the post
    #[serde(skip_serializing_if = "Option::is_none")]
    pub price: Option<crate::types::SuggestedPostPrice>,
    /// Date when the post will be published
    pub send_date: i64,
}
impl SuggestedPostApproved {
    /// Creates a new `SuggestedPostApproved`.
    ///
    /// # Arguments
    /// * `send_date` - Date when the post will be published
    ///
    /// # Notes
    /// Use builder methods to set optional fields.
    #[must_use]
    pub fn new<T0: Into<i64>>(send_date: T0) -> Self {
        Self {
            suggested_post_message: None,
            price: None,
            send_date: send_date.into(),
        }
    }

    /// Message containing the suggested post. Note that the Message object in this field will not contain the `reply_to_message` field even if it itself is a reply.
    #[must_use]
    pub fn suggested_post_message<T: Into<crate::types::Message>>(mut self, val: T) -> Self {
        self.suggested_post_message = Some(Box::new(val.into()));
        self
    }

    /// Message containing the suggested post. Note that the Message object in this field will not contain the `reply_to_message` field even if it itself is a reply.
    #[must_use]
    pub fn suggested_post_message_option<T: Into<crate::types::Message>>(
        mut self,
        val: Option<T>,
    ) -> Self {
        self.suggested_post_message = val.map(|val| Box::new(val.into()));
        self
    }

    /// Amount paid for the post
    #[must_use]
    pub fn price<T: Into<crate::types::SuggestedPostPrice>>(mut self, val: T) -> Self {
        self.price = Some(val.into());
        self
    }

    /// Amount paid for the post
    #[must_use]
    pub fn price_option<T: Into<crate::types::SuggestedPostPrice>>(
        mut self,
        val: Option<T>,
    ) -> Self {
        self.price = val.map(Into::into);
        self
    }

    /// Date when the post will be published
    #[must_use]
    pub fn send_date<T: Into<i64>>(mut self, val: T) -> Self {
        self.send_date = val.into();
        self
    }
}
