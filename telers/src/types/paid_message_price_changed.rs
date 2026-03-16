use serde::{Deserialize, Serialize};
/// Describes a service message about a change in the price of paid messages within a chat.
/// # Documentation
/// <https://core.telegram.org/bots/api#paidmessagepricechanged>
#[derive(Clone, Debug, Serialize, Deserialize)]
pub struct PaidMessagePriceChanged {
    /// The new number of Telegram Stars that must be paid by non-administrator users of the supergroup chat for each sent message
    pub paid_message_star_count: i64,
}
impl PaidMessagePriceChanged {
    /// Creates a new `PaidMessagePriceChanged`.
    ///
    /// # Arguments
    /// * `paid_message_star_count` - The new number of Telegram Stars that must be paid by non-administrator users of the supergroup chat for each sent message
    #[must_use]
    pub fn new<T0: Into<i64>>(paid_message_star_count: T0) -> Self {
        Self {
            paid_message_star_count: paid_message_star_count.into(),
        }
    }

    /// The new number of Telegram Stars that must be paid by non-administrator users of the supergroup chat for each sent message
    #[must_use]
    pub fn paid_message_star_count<T: Into<i64>>(self, val: T) -> Self {
        let mut this = self;
        this.paid_message_star_count = val.into();
        this
    }
}
