use serde::{Deserialize, Serialize};
/// Describes a service message about the rejection of a suggested post.
/// # Documentation
/// <https://core.telegram.org/bots/api#suggestedpostdeclined>
#[derive(Clone, Debug, Serialize, Deserialize)]
pub struct SuggestedPostDeclined {
    /// Message containing the suggested post. Note that the Message object in this field will not contain the `reply_to_message` field even if it itself is a reply.
    #[serde(skip_serializing_if = "Option::is_none")]
    pub suggested_post_message: Option<Box<crate::types::Message>>,
    /// Comment with which the post was declined
    #[serde(skip_serializing_if = "Option::is_none")]
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

    /// Comment with which the post was declined
    #[must_use]
    pub fn comment<T: Into<Box<str>>>(self, val: T) -> Self {
        let mut this = self;
        this.comment = Some(val.into());
        this
    }

    /// Comment with which the post was declined
    #[must_use]
    pub fn comment_option<T: Into<Box<str>>>(self, val: Option<T>) -> Self {
        let mut this = self;
        this.comment = val.map(Into::into);
        this
    }
}
impl Default for SuggestedPostDeclined {
    fn default() -> Self {
        Self::new()
    }
}
