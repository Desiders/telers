/// Describes a service message about the rejection of a suggested post.
/// # Documentation
/// <https://core.telegram.org/bots/api#suggestedpostdeclined>
#[derive(Clone, Debug)]
#[cfg_attr(feature = "deser", derive(deser::Serialize, deser::Deserialize))]
#[cfg_attr(not(feature = "deser"), derive(serde::Serialize, serde::Deserialize))]
pub struct SuggestedPostDeclined {
    /// Message containing the suggested post. Note that the Message object in this field will not contain the `reply_to_message` field even if it itself is a reply.
    #[cfg_attr(not(feature = "deser"), serde(skip_serializing_if = "Option::is_none"))]
    #[cfg_attr(feature = "deser", deser(skip_serializing_if = Option::is_none))]
    pub suggested_post_message: Option<Box<crate::types::Message>>,
    /// Comment with which the post was declined
    #[cfg_attr(not(feature = "deser"), serde(skip_serializing_if = "Option::is_none"))]
    #[cfg_attr(feature = "deser", deser(skip_serializing_if = Option::is_none))]
    pub comment: Option<Box<str>>,
}
impl SuggestedPostDeclined {
    /// Creates a new `SuggestedPostDeclined`.
    ///
    /// # Notes
    /// Use builder methods to set optional fields.
    #[must_use]
    pub fn new() -> Self {
        Self {
            suggested_post_message: None,
            comment: None,
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

    /// Comment with which the post was declined
    #[must_use]
    pub fn comment<T: Into<Box<str>>>(mut self, val: T) -> Self {
        self.comment = Some(val.into());
        self
    }

    /// Comment with which the post was declined
    #[must_use]
    pub fn comment_option<T: Into<Box<str>>>(mut self, val: Option<T>) -> Self {
        self.comment = val.map(Into::into);
        self
    }
}
impl Default for SuggestedPostDeclined {
    fn default() -> Self {
        Self::new()
    }
}
