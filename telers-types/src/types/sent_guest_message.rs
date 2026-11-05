/// Describes an inline message sent by a guest bot.
/// # Documentation
/// <https://core.telegram.org/bots/api#sentguestmessage>
#[derive(Clone, Debug)]
#[cfg_attr(feature = "deser", derive(deser::Serialize, deser::Deserialize))]
#[cfg_attr(not(feature = "deser"), derive(serde::Serialize, serde::Deserialize))]
pub struct SentGuestMessage {
    /// Identifier of the sent inline message
    pub inline_message_id: Box<str>,
}
impl SentGuestMessage {
    /// Creates a new `SentGuestMessage`.
    ///
    /// # Arguments
    /// * `inline_message_id` - Identifier of the sent inline message
    #[must_use]
    pub fn new<T0: Into<Box<str>>>(inline_message_id: T0) -> Self {
        Self {
            inline_message_id: inline_message_id.into(),
        }
    }

    /// Identifier of the sent inline message
    #[must_use]
    pub fn inline_message_id<T: Into<Box<str>>>(mut self, val: T) -> Self {
        self.inline_message_id = val.into();
        self
    }
}
