/// Describes a service message about the sender of the message using a link of the other user, which allows sending messages to the user without paying Telegram Stars. Currently holds no information.
/// # Documentation
/// <https://core.telegram.org/bots/api#paidmessageswaived>
#[derive(Clone, Debug)]
#[cfg_attr(feature = "deser", derive(deser::Serialize, deser::Deserialize))]
#[cfg_attr(not(feature = "deser"), derive(serde::Serialize, serde::Deserialize))]
pub struct PaidMessagesWaived {}
impl PaidMessagesWaived {
    /// Creates a new `PaidMessagesWaived`.
    #[must_use]
    pub const fn new() -> Self {
        Self {}
    }
}
impl Default for PaidMessagesWaived {
    fn default() -> Self {
        Self::new()
    }
}
