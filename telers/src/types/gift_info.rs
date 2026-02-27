use serde::{Deserialize, Serialize};
/// Describes a service message about a regular gift that was sent or received.
/// # Documentation
/// <https://core.telegram.org/bots/api#giftinfo>
#[derive(Clone, Debug, Serialize, Deserialize)]
pub struct GiftInfo {
    /// Information about the gift
    pub gift: Box<crate::types::Gift>,
    /// Unique identifier of the received gift for the bot; only present for gifts received on behalf of business accounts
    #[serde(skip_serializing_if = "Option::is_none")]
    pub owned_gift_id: Option<Box<str>>,
    /// Number of Telegram Stars that can be claimed by the receiver by converting the gift; omitted if conversion to Telegram Stars is impossible
    #[serde(skip_serializing_if = "Option::is_none")]
    pub convert_star_count: Option<i64>,
    /// Number of Telegram Stars that were prepaid for the ability to upgrade the gift
    #[serde(skip_serializing_if = "Option::is_none")]
    pub prepaid_upgrade_star_count: Option<i64>,
    /// `true`, if the gift's upgrade was purchased after the gift was sent
    #[serde(skip_serializing_if = "Option::is_none")]
    pub is_upgrade_separate: Option<bool>,
    /// `true`, if the gift can be upgraded to a unique gift
    #[serde(skip_serializing_if = "Option::is_none")]
    pub can_be_upgraded: Option<bool>,
    /// Text of the message that was added to the gift
    #[serde(skip_serializing_if = "Option::is_none")]
    pub text: Option<Box<str>>,
    /// Special entities that appear in the text
    #[serde(skip_serializing_if = "Option::is_none")]
    pub entities: Option<Box<[crate::types::MessageEntity]>>,
    /// `true`, if the sender and gift text are shown only to the gift receiver; otherwise, everyone will be able to see them
    #[serde(skip_serializing_if = "Option::is_none")]
    pub is_private: Option<bool>,
    /// Unique number reserved for this gift when upgraded. See the number field in [`UniqueGift`]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub unique_gift_number: Option<i64>,
}
impl GiftInfo {
    /// Creates a new `GiftInfo`.
    ///
    /// # Arguments
    /// * `gift` - Information about the gift
    ///
    /// # Notes
    /// Use builder methods to set optional fields.
    #[must_use]
    pub fn new<T0: Into<crate::types::Gift>>(gift: T0) -> Self {
        Self {
            gift: Box::new(gift.into()),
            owned_gift_id: None,
            convert_star_count: None,
            prepaid_upgrade_star_count: None,
            is_upgrade_separate: None,
            can_be_upgraded: None,
            text: None,
            entities: None,
            is_private: None,
            unique_gift_number: None,
        }
    }

    /// Information about the gift
    #[must_use]
    pub fn gift<T: Into<crate::types::Gift>>(self, val: T) -> Self {
        let mut this = self;
        this.gift = Box::new(val.into());
        this
    }

    /// Unique identifier of the received gift for the bot; only present for gifts received on behalf of business accounts
    #[must_use]
    pub fn owned_gift_id<T: Into<Box<str>>>(self, val: T) -> Self {
        let mut this = self;
        this.owned_gift_id = Some(val.into());
        this
    }

    /// Unique identifier of the received gift for the bot; only present for gifts received on behalf of business accounts
    #[must_use]
    pub fn owned_gift_id_option<T: Into<Box<str>>>(self, val: Option<T>) -> Self {
        let mut this = self;
        this.owned_gift_id = val.map(Into::into);
        this
    }

    /// Number of Telegram Stars that can be claimed by the receiver by converting the gift; omitted if conversion to Telegram Stars is impossible
    #[must_use]
    pub fn convert_star_count<T: Into<i64>>(self, val: T) -> Self {
        let mut this = self;
        this.convert_star_count = Some(val.into());
        this
    }

    /// Number of Telegram Stars that can be claimed by the receiver by converting the gift; omitted if conversion to Telegram Stars is impossible
    #[must_use]
    pub fn convert_star_count_option<T: Into<i64>>(self, val: Option<T>) -> Self {
        let mut this = self;
        this.convert_star_count = val.map(Into::into);
        this
    }

    /// Number of Telegram Stars that were prepaid for the ability to upgrade the gift
    #[must_use]
    pub fn prepaid_upgrade_star_count<T: Into<i64>>(self, val: T) -> Self {
        let mut this = self;
        this.prepaid_upgrade_star_count = Some(val.into());
        this
    }

    /// Number of Telegram Stars that were prepaid for the ability to upgrade the gift
    #[must_use]
    pub fn prepaid_upgrade_star_count_option<T: Into<i64>>(self, val: Option<T>) -> Self {
        let mut this = self;
        this.prepaid_upgrade_star_count = val.map(Into::into);
        this
    }

    /// `true`, if the gift's upgrade was purchased after the gift was sent
    #[must_use]
    pub fn is_upgrade_separate<T: Into<bool>>(self, val: T) -> Self {
        let mut this = self;
        this.is_upgrade_separate = Some(val.into());
        this
    }

    /// `true`, if the gift's upgrade was purchased after the gift was sent
    #[must_use]
    pub fn is_upgrade_separate_option<T: Into<bool>>(self, val: Option<T>) -> Self {
        let mut this = self;
        this.is_upgrade_separate = val.map(Into::into);
        this
    }

    /// `true`, if the gift can be upgraded to a unique gift
    #[must_use]
    pub fn can_be_upgraded<T: Into<bool>>(self, val: T) -> Self {
        let mut this = self;
        this.can_be_upgraded = Some(val.into());
        this
    }

    /// `true`, if the gift can be upgraded to a unique gift
    #[must_use]
    pub fn can_be_upgraded_option<T: Into<bool>>(self, val: Option<T>) -> Self {
        let mut this = self;
        this.can_be_upgraded = val.map(Into::into);
        this
    }

    /// Text of the message that was added to the gift
    #[must_use]
    pub fn text<T: Into<Box<str>>>(self, val: T) -> Self {
        let mut this = self;
        this.text = Some(val.into());
        this
    }

    /// Text of the message that was added to the gift
    #[must_use]
    pub fn text_option<T: Into<Box<str>>>(self, val: Option<T>) -> Self {
        let mut this = self;
        this.text = val.map(Into::into);
        this
    }

    /// Special entities that appear in the text
    ///
    /// # Notes
    /// Adds multiple elements.
    #[must_use]
    pub fn entities<T: Into<Box<[crate::types::MessageEntity]>>>(self, val: T) -> Self {
        let mut this = self;
        this.entities = Some(
            this.entities
                .unwrap_or_default()
                .into_vec()
                .into_iter()
                .chain(val.into())
                .collect(),
        );
        this
    }

    /// Special entities that appear in the text
    ///
    /// # Notes
    /// Adds a single element.
    #[must_use]
    pub fn entity<T: Into<crate::types::MessageEntity>>(self, val: T) -> Self {
        let mut this = self;
        this.entities = Some(
            this.entities
                .unwrap_or_default()
                .into_vec()
                .into_iter()
                .chain(Some(val.into()))
                .collect(),
        );
        this
    }

    /// Special entities that appear in the text
    ///
    /// # Notes
    /// Adds a single element.
    #[must_use]
    pub fn entities_option<T: Into<Box<[crate::types::MessageEntity]>>>(
        self,
        val: Option<T>,
    ) -> Self {
        let mut this = self;
        this.entities = val.map(Into::into);
        this
    }

    /// `true`, if the sender and gift text are shown only to the gift receiver; otherwise, everyone will be able to see them
    #[must_use]
    pub fn is_private<T: Into<bool>>(self, val: T) -> Self {
        let mut this = self;
        this.is_private = Some(val.into());
        this
    }

    /// `true`, if the sender and gift text are shown only to the gift receiver; otherwise, everyone will be able to see them
    #[must_use]
    pub fn is_private_option<T: Into<bool>>(self, val: Option<T>) -> Self {
        let mut this = self;
        this.is_private = val.map(Into::into);
        this
    }

    /// Unique number reserved for this gift when upgraded. See the number field in [`UniqueGift`]
    #[must_use]
    pub fn unique_gift_number<T: Into<i64>>(self, val: T) -> Self {
        let mut this = self;
        this.unique_gift_number = Some(val.into());
        this
    }

    /// Unique number reserved for this gift when upgraded. See the number field in [`UniqueGift`]
    #[must_use]
    pub fn unique_gift_number_option<T: Into<i64>>(self, val: Option<T>) -> Self {
        let mut this = self;
        this.unique_gift_number = val.map(Into::into);
        this
    }
}
