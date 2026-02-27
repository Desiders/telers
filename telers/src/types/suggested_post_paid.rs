use serde::{Deserialize, Serialize};
/// Describes a service message about a successful payment for a suggested post.
/// # Documentation
/// <https://core.telegram.org/bots/api#suggestedpostpaid>
#[derive(Clone, Debug, Serialize, Deserialize)]
pub struct SuggestedPostPaid {
    /// Message containing the suggested post. Note that the Message object in this field will not contain the `reply_to_message` field even if it itself is a reply.
    #[serde(skip_serializing_if = "Option::is_none")]
    pub suggested_post_message: Option<Box<crate::types::Message>>,
    /// Currency in which the payment was made. Currently, one of `XTR` for Telegram Stars or `TON` for toncoins
    pub currency: Box<str>,
    /// The amount of the currency that was received by the channel in nanotoncoins; for payments in toncoins only
    #[serde(skip_serializing_if = "Option::is_none")]
    pub amount: Option<i64>,
    /// The amount of Telegram Stars that was received by the channel; for payments in Telegram Stars only
    #[serde(skip_serializing_if = "Option::is_none")]
    pub star_amount: Option<crate::types::StarAmount>,
}
impl SuggestedPostPaid {
    /// Creates a new `SuggestedPostPaid`.
    ///
    /// # Arguments
    /// * `currency` - Currency in which the payment was made. Currently, one of `XTR` for Telegram Stars or `TON` for toncoins
    ///
    /// # Notes
    /// Use builder methods to set optional fields.
    #[must_use]
    pub fn new<T0: Into<Box<str>>>(currency: T0) -> Self {
        Self {
            suggested_post_message: None,
            currency: currency.into(),
            amount: None,
            star_amount: None,
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

    /// Currency in which the payment was made. Currently, one of `XTR` for Telegram Stars or `TON` for toncoins
    #[must_use]
    pub fn currency<T: Into<Box<str>>>(self, val: T) -> Self {
        let mut this = self;
        this.currency = val.into();
        this
    }

    /// The amount of the currency that was received by the channel in nanotoncoins; for payments in toncoins only
    #[must_use]
    pub fn amount<T: Into<i64>>(self, val: T) -> Self {
        let mut this = self;
        this.amount = Some(val.into());
        this
    }

    /// The amount of the currency that was received by the channel in nanotoncoins; for payments in toncoins only
    #[must_use]
    pub fn amount_option<T: Into<i64>>(self, val: Option<T>) -> Self {
        let mut this = self;
        this.amount = val.map(Into::into);
        this
    }

    /// The amount of Telegram Stars that was received by the channel; for payments in Telegram Stars only
    #[must_use]
    pub fn star_amount<T: Into<crate::types::StarAmount>>(self, val: T) -> Self {
        let mut this = self;
        this.star_amount = Some(val.into());
        this
    }

    /// The amount of Telegram Stars that was received by the channel; for payments in Telegram Stars only
    #[must_use]
    pub fn star_amount_option<T: Into<crate::types::StarAmount>>(self, val: Option<T>) -> Self {
        let mut this = self;
        this.star_amount = val.map(Into::into);
        this
    }
}
