use serde::{Deserialize, Serialize};
/// This object contains information about changes to a user payment subscription toward the current bot.
/// # Documentation
/// <https://core.telegram.org/bots/api#botsubscriptionupdated>
#[derive(Clone, Debug, Serialize, Deserialize)]
pub struct BotSubscriptionUpdated {
    /// User who subscribed for payments toward the bot
    pub user: Box<crate::types::User>,
    /// Bot-specified invoice payload
    pub invoice_payload: Box<str>,
    /// The new state of the subscription. Currently, it can be one of `canceled` if the user canceled the subscription, `active` if the user re-enabled a previously canceled subscription, or `failed` if payment for the subscription failed.
    pub state: Box<str>,
}
impl BotSubscriptionUpdated {
    /// Creates a new `BotSubscriptionUpdated`.
    ///
    /// # Arguments
    /// * `user` - User who subscribed for payments toward the bot
    /// * `invoice_payload` - Bot-specified invoice payload
    /// * `state` - The new state of the subscription. Currently, it can be one of `canceled` if the user canceled the subscription, `active` if the user re-enabled a previously canceled subscription, or `failed` if payment for the subscription failed.
    #[must_use]
    pub fn new<T0: Into<crate::types::User>, T1: Into<Box<str>>, T2: Into<Box<str>>>(
        user: T0,
        invoice_payload: T1,
        state: T2,
    ) -> Self {
        Self {
            user: Box::new(user.into()),
            invoice_payload: invoice_payload.into(),
            state: state.into(),
        }
    }

    /// User who subscribed for payments toward the bot
    #[must_use]
    pub fn user<T: Into<crate::types::User>>(mut self, val: T) -> Self {
        self.user = Box::new(val.into());
        self
    }

    /// Bot-specified invoice payload
    #[must_use]
    pub fn invoice_payload<T: Into<Box<str>>>(mut self, val: T) -> Self {
        self.invoice_payload = val.into();
        self
    }

    /// The new state of the subscription. Currently, it can be one of `canceled` if the user canceled the subscription, `active` if the user re-enabled a previously canceled subscription, or `failed` if payment for the subscription failed.
    #[must_use]
    pub fn state<T: Into<Box<str>>>(mut self, val: T) -> Self {
        self.state = val.into();
        self
    }
}
