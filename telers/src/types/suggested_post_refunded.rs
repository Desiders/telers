use serde::{Deserialize, Serialize};
/// Describes a service message about a payment refund for a suggested post.
/// # Documentation
/// <https://core.telegram.org/bots/api#suggestedpostrefunded>
#[derive(Clone, Debug, Serialize, Deserialize)]
pub struct SuggestedPostRefunded {
    /// Message containing the suggested post. Note that the Message object in this field will not contain the `reply_to_message` field even if it itself is a reply.
    #[serde(skip_serializing_if = "Option::is_none")]
    pub suggested_post_message: Option<Box<crate::types::Message>>,
    /// Reason for the refund. Currently, one of `post_deleted` if the post was deleted within 24 hours of being posted or removed from scheduled messages without being posted, or `payment_refunded` if the payer refunded their payment.
    pub reason: Box<str>,
}
impl SuggestedPostRefunded {
    /// Creates a new `SuggestedPostRefunded`.
    ///
    /// # Arguments
    /// * `reason` - Reason for the refund. Currently, one of `post_deleted` if the post was deleted within 24 hours of being posted or removed from scheduled messages without being posted, or `payment_refunded` if the payer refunded their payment.
    ///
    /// # Notes
    /// Use builder methods to set optional fields.
    #[must_use]
    pub fn new<T0: Into<Box<str>>>(reason: T0) -> Self {
        Self {
            suggested_post_message: None,
            reason: reason.into(),
        }
    }

    /// Message containing the suggested post. Note that the Message object in this field will not contain the `reply_to_message` field even if it itself is a reply.
    #[must_use]
    pub fn suggested_post_message<T: Into<crate::types::Message>>(self, val: T) -> Self {
        let mut this = self;
        this.suggested_post_message = Some(Box::new(val.into()));
        this
    }

    /// Message containing the suggested post. Note that the Message object in this field will not contain the `reply_to_message` field even if it itself is a reply.
    #[must_use]
    pub fn suggested_post_message_option<T: Into<crate::types::Message>>(
        self,
        val: Option<T>,
    ) -> Self {
        let mut this = self;
        this.suggested_post_message = val.map(|val| Box::new(val.into()));
        this
    }

    /// Reason for the refund. Currently, one of `post_deleted` if the post was deleted within 24 hours of being posted or removed from scheduled messages without being posted, or `payment_refunded` if the payer refunded their payment.
    #[must_use]
    pub fn reason<T: Into<Box<str>>>(self, val: T) -> Self {
        let mut this = self;
        this.reason = val.into();
        this
    }
}
