use serde::{Deserialize, Serialize};
/// Describes a service message about a change in the price of direct messages sent to a channel chat.
/// # Documentation
/// <https://core.telegram.org/bots/api#directmessagepricechanged>
#[derive(Clone, Debug, Serialize, Deserialize)]
pub struct DirectMessagePriceChanged {
    /// `true`, if direct messages are enabled for the channel chat; false otherwise
    pub are_direct_messages_enabled: bool,
    /// The new number of Telegram Stars that must be paid by users for each direct message sent to the channel. Does not apply to users who have been exempted by administrators. Defaults to 0.
    #[serde(skip_serializing_if = "Option::is_none")]
    pub direct_message_star_count: Option<i64>,
}
impl DirectMessagePriceChanged {
    /// Creates a new `DirectMessagePriceChanged`.
    ///
    /// # Arguments
    /// * `are_direct_messages_enabled` - `true`, if direct messages are enabled for the channel chat; false otherwise
    ///
    /// # Notes
    /// Use builder methods to set optional fields.
    #[must_use]
    pub fn new<T0: Into<bool>>(are_direct_messages_enabled: T0) -> Self {
        Self {
            are_direct_messages_enabled: are_direct_messages_enabled.into(),
            direct_message_star_count: None,
        }
    }

    /// `true`, if direct messages are enabled for the channel chat; false otherwise
    #[must_use]
    pub fn are_direct_messages_enabled<T: Into<bool>>(mut self, val: T) -> Self {
        self.are_direct_messages_enabled = val.into();
        self
    }

    /// The new number of Telegram Stars that must be paid by users for each direct message sent to the channel. Does not apply to users who have been exempted by administrators. Defaults to 0.
    #[must_use]
    pub fn direct_message_star_count<T: Into<i64>>(mut self, val: T) -> Self {
        self.direct_message_star_count = Some(val.into());
        self
    }

    /// The new number of Telegram Stars that must be paid by users for each direct message sent to the channel. Does not apply to users who have been exempted by administrators. Defaults to 0.
    #[must_use]
    pub fn direct_message_star_count_option<T: Into<i64>>(mut self, val: Option<T>) -> Self {
        self.direct_message_star_count = val.map(Into::into);
        self
    }
}
