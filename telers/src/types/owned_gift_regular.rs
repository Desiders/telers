use serde::{Deserialize, Serialize};
/// Describes a regular gift owned by a user or a chat.
/// # Documentation
/// <https://core.telegram.org/bots/api#ownedgiftregular>
#[derive(Clone, Debug, Serialize, Deserialize)]
pub struct OwnedGiftRegular {
    /// Information about the regular gift
    pub gift: Box<crate::types::Gift>,
    /// Unique identifier of the gift for the bot; for gifts received on behalf of business accounts only
    #[serde(skip_serializing_if = "Option::is_none")]
    pub owned_gift_id: Option<Box<str>>,
    /// Sender of the gift if it is a known user
    #[serde(skip_serializing_if = "Option::is_none")]
    pub sender_user: Option<Box<crate::types::User>>,
    /// Date the gift was sent in Unix time
    pub send_date: i64,
    /// Text of the message that was added to the gift
    #[serde(skip_serializing_if = "Option::is_none")]
    pub text: Option<Box<str>>,
    /// Special entities that appear in the text
    #[serde(skip_serializing_if = "Option::is_none")]
    pub entities: Option<Box<[crate::types::MessageEntity]>>,
    /// `true`, if the sender and gift text are shown only to the gift receiver; otherwise, everyone will be able to see them
    #[serde(skip_serializing_if = "Option::is_none")]
    pub is_private: Option<bool>,
    /// `true`, if the gift is displayed on the account's profile page; for gifts received on behalf of business accounts only
    #[serde(skip_serializing_if = "Option::is_none")]
    pub is_saved: Option<bool>,
    /// `true`, if the gift can be upgraded to a unique gift; for gifts received on behalf of business accounts only
    #[serde(skip_serializing_if = "Option::is_none")]
    pub can_be_upgraded: Option<bool>,
    /// `true`, if the gift was refunded and isn't available anymore
    #[serde(skip_serializing_if = "Option::is_none")]
    pub was_refunded: Option<bool>,
    /// Number of Telegram Stars that can be claimed by the receiver instead of the gift; omitted if the gift cannot be converted to Telegram Stars; for gifts received on behalf of business accounts only
    #[serde(skip_serializing_if = "Option::is_none")]
    pub convert_star_count: Option<i64>,
    /// Number of Telegram Stars that were paid for the ability to upgrade the gift
    #[serde(skip_serializing_if = "Option::is_none")]
    pub prepaid_upgrade_star_count: Option<i64>,
    /// `true`, if the gift's upgrade was purchased after the gift was sent; for gifts received on behalf of business accounts only
    #[serde(skip_serializing_if = "Option::is_none")]
    pub is_upgrade_separate: Option<bool>,
    /// Unique number reserved for this gift when upgraded. See the number field in [`crate::types::UniqueGift`].
    #[serde(skip_serializing_if = "Option::is_none")]
    pub unique_gift_number: Option<i64>,
}
impl OwnedGiftRegular {
    /// Creates a new `OwnedGiftRegular`.
    ///
    /// # Arguments
    /// * `gift` - Information about the regular gift
    /// * `send_date` - Date the gift was sent in Unix time
    ///
    /// # Notes
    /// Use builder methods to set optional fields.
    #[must_use]
    pub fn new<T0: Into<crate::types::Gift>, T1: Into<i64>>(gift: T0, send_date: T1) -> Self {
        Self {
            gift: Box::new(gift.into()),
            owned_gift_id: None,
            sender_user: None,
            send_date: send_date.into(),
            text: None,
            entities: None,
            is_private: None,
            is_saved: None,
            can_be_upgraded: None,
            was_refunded: None,
            convert_star_count: None,
            prepaid_upgrade_star_count: None,
            is_upgrade_separate: None,
            unique_gift_number: None,
        }
    }

    /// Information about the regular gift
    #[must_use]
    pub fn gift<T: Into<crate::types::Gift>>(mut self, val: T) -> Self {
        self.gift = Box::new(val.into());
        self
    }

    /// Unique identifier of the gift for the bot; for gifts received on behalf of business accounts only
    #[must_use]
    pub fn owned_gift_id<T: Into<Box<str>>>(mut self, val: T) -> Self {
        self.owned_gift_id = Some(val.into());
        self
    }

    /// Unique identifier of the gift for the bot; for gifts received on behalf of business accounts only
    #[must_use]
    pub fn owned_gift_id_option<T: Into<Box<str>>>(mut self, val: Option<T>) -> Self {
        self.owned_gift_id = val.map(Into::into);
        self
    }

    /// Sender of the gift if it is a known user
    #[must_use]
    pub fn sender_user<T: Into<crate::types::User>>(mut self, val: T) -> Self {
        self.sender_user = Some(Box::new(val.into()));
        self
    }

    /// Sender of the gift if it is a known user
    #[must_use]
    pub fn sender_user_option<T: Into<crate::types::User>>(mut self, val: Option<T>) -> Self {
        self.sender_user = val.map(|val| Box::new(val.into()));
        self
    }

    /// Date the gift was sent in Unix time
    #[must_use]
    pub fn send_date<T: Into<i64>>(mut self, val: T) -> Self {
        self.send_date = val.into();
        self
    }

    /// Text of the message that was added to the gift
    #[must_use]
    pub fn text<T: Into<Box<str>>>(mut self, val: T) -> Self {
        self.text = Some(val.into());
        self
    }

    /// Text of the message that was added to the gift
    #[must_use]
    pub fn text_option<T: Into<Box<str>>>(mut self, val: Option<T>) -> Self {
        self.text = val.map(Into::into);
        self
    }

    /// Special entities that appear in the text
    ///
    /// # Notes
    /// Adds multiple elements.
    #[must_use]
    pub fn entities<T: Into<Box<[crate::types::MessageEntity]>>>(mut self, val: T) -> Self {
        self.entities = Some(
            self.entities
                .unwrap_or_default()
                .into_vec()
                .into_iter()
                .chain(val.into())
                .collect(),
        );
        self
    }

    /// Special entities that appear in the text
    ///
    /// # Notes
    /// Adds a single element.
    #[must_use]
    pub fn entity<T: Into<crate::types::MessageEntity>>(mut self, val: T) -> Self {
        self.entities = Some(
            self.entities
                .unwrap_or_default()
                .into_vec()
                .into_iter()
                .chain(Some(val.into()))
                .collect(),
        );
        self
    }

    /// Special entities that appear in the text
    ///
    /// # Notes
    /// Adds a single element.
    #[must_use]
    pub fn entities_option<T: Into<Box<[crate::types::MessageEntity]>>>(
        mut self,
        val: Option<T>,
    ) -> Self {
        self.entities = val.map(Into::into);
        self
    }

    /// `true`, if the sender and gift text are shown only to the gift receiver; otherwise, everyone will be able to see them
    #[must_use]
    pub fn is_private<T: Into<bool>>(mut self, val: T) -> Self {
        self.is_private = Some(val.into());
        self
    }

    /// `true`, if the sender and gift text are shown only to the gift receiver; otherwise, everyone will be able to see them
    #[must_use]
    pub fn is_private_option<T: Into<bool>>(mut self, val: Option<T>) -> Self {
        self.is_private = val.map(Into::into);
        self
    }

    /// `true`, if the gift is displayed on the account's profile page; for gifts received on behalf of business accounts only
    #[must_use]
    pub fn is_saved<T: Into<bool>>(mut self, val: T) -> Self {
        self.is_saved = Some(val.into());
        self
    }

    /// `true`, if the gift is displayed on the account's profile page; for gifts received on behalf of business accounts only
    #[must_use]
    pub fn is_saved_option<T: Into<bool>>(mut self, val: Option<T>) -> Self {
        self.is_saved = val.map(Into::into);
        self
    }

    /// `true`, if the gift can be upgraded to a unique gift; for gifts received on behalf of business accounts only
    #[must_use]
    pub fn can_be_upgraded<T: Into<bool>>(mut self, val: T) -> Self {
        self.can_be_upgraded = Some(val.into());
        self
    }

    /// `true`, if the gift can be upgraded to a unique gift; for gifts received on behalf of business accounts only
    #[must_use]
    pub fn can_be_upgraded_option<T: Into<bool>>(mut self, val: Option<T>) -> Self {
        self.can_be_upgraded = val.map(Into::into);
        self
    }

    /// `true`, if the gift was refunded and isn't available anymore
    #[must_use]
    pub fn was_refunded<T: Into<bool>>(mut self, val: T) -> Self {
        self.was_refunded = Some(val.into());
        self
    }

    /// `true`, if the gift was refunded and isn't available anymore
    #[must_use]
    pub fn was_refunded_option<T: Into<bool>>(mut self, val: Option<T>) -> Self {
        self.was_refunded = val.map(Into::into);
        self
    }

    /// Number of Telegram Stars that can be claimed by the receiver instead of the gift; omitted if the gift cannot be converted to Telegram Stars; for gifts received on behalf of business accounts only
    #[must_use]
    pub fn convert_star_count<T: Into<i64>>(mut self, val: T) -> Self {
        self.convert_star_count = Some(val.into());
        self
    }

    /// Number of Telegram Stars that can be claimed by the receiver instead of the gift; omitted if the gift cannot be converted to Telegram Stars; for gifts received on behalf of business accounts only
    #[must_use]
    pub fn convert_star_count_option<T: Into<i64>>(mut self, val: Option<T>) -> Self {
        self.convert_star_count = val.map(Into::into);
        self
    }

    /// Number of Telegram Stars that were paid for the ability to upgrade the gift
    #[must_use]
    pub fn prepaid_upgrade_star_count<T: Into<i64>>(mut self, val: T) -> Self {
        self.prepaid_upgrade_star_count = Some(val.into());
        self
    }

    /// Number of Telegram Stars that were paid for the ability to upgrade the gift
    #[must_use]
    pub fn prepaid_upgrade_star_count_option<T: Into<i64>>(mut self, val: Option<T>) -> Self {
        self.prepaid_upgrade_star_count = val.map(Into::into);
        self
    }

    /// `true`, if the gift's upgrade was purchased after the gift was sent; for gifts received on behalf of business accounts only
    #[must_use]
    pub fn is_upgrade_separate<T: Into<bool>>(mut self, val: T) -> Self {
        self.is_upgrade_separate = Some(val.into());
        self
    }

    /// `true`, if the gift's upgrade was purchased after the gift was sent; for gifts received on behalf of business accounts only
    #[must_use]
    pub fn is_upgrade_separate_option<T: Into<bool>>(mut self, val: Option<T>) -> Self {
        self.is_upgrade_separate = val.map(Into::into);
        self
    }

    /// Unique number reserved for this gift when upgraded. See the number field in [`crate::types::UniqueGift`].
    #[must_use]
    pub fn unique_gift_number<T: Into<i64>>(mut self, val: T) -> Self {
        self.unique_gift_number = Some(val.into());
        self
    }

    /// Unique number reserved for this gift when upgraded. See the number field in [`crate::types::UniqueGift`].
    #[must_use]
    pub fn unique_gift_number_option<T: Into<i64>>(mut self, val: Option<T>) -> Self {
        self.unique_gift_number = val.map(Into::into);
        self
    }
}
