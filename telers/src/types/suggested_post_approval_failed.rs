/// Describes a service message about the failed approval of a suggested post. Currently, only caused by insufficient user funds at the time of approval.
/// # Documentation
/// <https://core.telegram.org/bots/api#suggestedpostapprovalfailed>
#[derive(Clone, Debug)]
#[cfg_attr(feature = "deser", derive(deser::Serialize, deser::Deserialize))]
#[cfg_attr(not(feature = "deser"), derive(serde::Serialize, serde::Deserialize))]
pub struct SuggestedPostApprovalFailed {
    /// Message containing the suggested post whose approval has failed. Note that the Message object in this field will not contain the `reply_to_message` field even if it itself is a reply.
    #[cfg_attr(not(feature = "deser"), serde(skip_serializing_if = "Option::is_none"))]
    #[cfg_attr(feature = "deser", deser(skip_serializing_if = Option::is_none))]
    pub suggested_post_message: Option<Box<crate::types::Message>>,
    /// Expected price of the post
    pub price: crate::types::SuggestedPostPrice,
}
impl SuggestedPostApprovalFailed {
    /// Creates a new `SuggestedPostApprovalFailed`.
    ///
    /// # Arguments
    /// * `price` - Expected price of the post
    ///
    /// # Notes
    /// Use builder methods to set optional fields.
    #[must_use]
    pub fn new<T0: Into<crate::types::SuggestedPostPrice>>(price: T0) -> Self {
        Self {
            suggested_post_message: None,
            price: price.into(),
        }
    }

    /// Message containing the suggested post whose approval has failed. Note that the Message object in this field will not contain the `reply_to_message` field even if it itself is a reply.
    #[must_use]
    pub fn suggested_post_message<T: Into<crate::types::Message>>(mut self, val: T) -> Self {
        self.suggested_post_message = Some(Box::new(val.into()));
        self
    }

    /// Message containing the suggested post whose approval has failed. Note that the Message object in this field will not contain the `reply_to_message` field even if it itself is a reply.
    #[must_use]
    pub fn suggested_post_message_option<T: Into<crate::types::Message>>(
        mut self,
        val: Option<T>,
    ) -> Self {
        self.suggested_post_message = val.map(|val| Box::new(val.into()));
        self
    }

    /// Expected price of the post
    #[must_use]
    pub fn price<T: Into<crate::types::SuggestedPostPrice>>(mut self, val: T) -> Self {
        self.price = val.into();
        self
    }
}
