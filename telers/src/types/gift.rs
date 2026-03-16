use serde::{Deserialize, Serialize};
/// This object represents a gift that can be sent by the bot.
/// # Documentation
/// <https://core.telegram.org/bots/api#gift>
#[derive(Clone, Debug, Serialize, Deserialize)]
pub struct Gift {
    /// Unique identifier of the gift
    pub id: Box<str>,
    /// The sticker that represents the gift
    pub sticker: Box<crate::types::Sticker>,
    /// The number of Telegram Stars that must be paid to send the sticker
    pub star_count: i64,
    /// The number of Telegram Stars that must be paid to upgrade the gift to a unique one
    #[serde(skip_serializing_if = "Option::is_none")]
    pub upgrade_star_count: Option<i64>,
    /// `true`, if the gift can only be purchased by Telegram Premium subscribers
    #[serde(skip_serializing_if = "Option::is_none")]
    pub is_premium: Option<bool>,
    /// `true`, if the gift can be used (after being upgraded) to customize a user's appearance
    #[serde(skip_serializing_if = "Option::is_none")]
    pub has_colors: Option<bool>,
    /// The total number of gifts of this type that can be sent by all users; for limited gifts only
    #[serde(skip_serializing_if = "Option::is_none")]
    pub total_count: Option<i64>,
    /// The number of remaining gifts of this type that can be sent by all users; for limited gifts only
    #[serde(skip_serializing_if = "Option::is_none")]
    pub remaining_count: Option<i64>,
    /// The total number of gifts of this type that can be sent by the bot; for limited gifts only
    #[serde(skip_serializing_if = "Option::is_none")]
    pub personal_total_count: Option<i64>,
    /// The number of remaining gifts of this type that can be sent by the bot; for limited gifts only
    #[serde(skip_serializing_if = "Option::is_none")]
    pub personal_remaining_count: Option<i64>,
    /// Background of the gift
    #[serde(skip_serializing_if = "Option::is_none")]
    pub background: Option<crate::types::GiftBackground>,
    /// The total number of different unique gifts that can be obtained by upgrading the gift
    #[serde(skip_serializing_if = "Option::is_none")]
    pub unique_gift_variant_count: Option<i64>,
    /// Information about the chat that published the gift
    #[serde(skip_serializing_if = "Option::is_none")]
    pub publisher_chat: Option<Box<crate::types::Chat>>,
}
impl Gift {
    /// Creates a new `Gift`.
    ///
    /// # Arguments
    /// * `id` - Unique identifier of the gift
    /// * `sticker` - The sticker that represents the gift
    /// * `star_count` - The number of Telegram Stars that must be paid to send the sticker
    ///
    /// # Notes
    /// Use builder methods to set optional fields.
    #[must_use]
    pub fn new<T0: Into<Box<str>>, T1: Into<crate::types::Sticker>, T2: Into<i64>>(
        id: T0,
        sticker: T1,
        star_count: T2,
    ) -> Self {
        Self {
            id: id.into(),
            sticker: Box::new(sticker.into()),
            star_count: star_count.into(),
            upgrade_star_count: None,
            is_premium: None,
            has_colors: None,
            total_count: None,
            remaining_count: None,
            personal_total_count: None,
            personal_remaining_count: None,
            background: None,
            unique_gift_variant_count: None,
            publisher_chat: None,
        }
    }

    /// Unique identifier of the gift
    #[must_use]
    pub fn id<T: Into<Box<str>>>(self, val: T) -> Self {
        let mut this = self;
        this.id = val.into();
        this
    }

    /// The sticker that represents the gift
    #[must_use]
    pub fn sticker<T: Into<crate::types::Sticker>>(self, val: T) -> Self {
        let mut this = self;
        this.sticker = Box::new(val.into());
        this
    }

    /// The number of Telegram Stars that must be paid to send the sticker
    #[must_use]
    pub fn star_count<T: Into<i64>>(self, val: T) -> Self {
        let mut this = self;
        this.star_count = val.into();
        this
    }

    /// The number of Telegram Stars that must be paid to upgrade the gift to a unique one
    #[must_use]
    pub fn upgrade_star_count<T: Into<i64>>(self, val: T) -> Self {
        let mut this = self;
        this.upgrade_star_count = Some(val.into());
        this
    }

    /// The number of Telegram Stars that must be paid to upgrade the gift to a unique one
    #[must_use]
    pub fn upgrade_star_count_option<T: Into<i64>>(self, val: Option<T>) -> Self {
        let mut this = self;
        this.upgrade_star_count = val.map(Into::into);
        this
    }

    /// `true`, if the gift can only be purchased by Telegram Premium subscribers
    #[must_use]
    pub fn is_premium<T: Into<bool>>(self, val: T) -> Self {
        let mut this = self;
        this.is_premium = Some(val.into());
        this
    }

    /// `true`, if the gift can only be purchased by Telegram Premium subscribers
    #[must_use]
    pub fn is_premium_option<T: Into<bool>>(self, val: Option<T>) -> Self {
        let mut this = self;
        this.is_premium = val.map(Into::into);
        this
    }

    /// `true`, if the gift can be used (after being upgraded) to customize a user's appearance
    #[must_use]
    pub fn has_colors<T: Into<bool>>(self, val: T) -> Self {
        let mut this = self;
        this.has_colors = Some(val.into());
        this
    }

    /// `true`, if the gift can be used (after being upgraded) to customize a user's appearance
    #[must_use]
    pub fn has_colors_option<T: Into<bool>>(self, val: Option<T>) -> Self {
        let mut this = self;
        this.has_colors = val.map(Into::into);
        this
    }

    /// The total number of gifts of this type that can be sent by all users; for limited gifts only
    #[must_use]
    pub fn total_count<T: Into<i64>>(self, val: T) -> Self {
        let mut this = self;
        this.total_count = Some(val.into());
        this
    }

    /// The total number of gifts of this type that can be sent by all users; for limited gifts only
    #[must_use]
    pub fn total_count_option<T: Into<i64>>(self, val: Option<T>) -> Self {
        let mut this = self;
        this.total_count = val.map(Into::into);
        this
    }

    /// The number of remaining gifts of this type that can be sent by all users; for limited gifts only
    #[must_use]
    pub fn remaining_count<T: Into<i64>>(self, val: T) -> Self {
        let mut this = self;
        this.remaining_count = Some(val.into());
        this
    }

    /// The number of remaining gifts of this type that can be sent by all users; for limited gifts only
    #[must_use]
    pub fn remaining_count_option<T: Into<i64>>(self, val: Option<T>) -> Self {
        let mut this = self;
        this.remaining_count = val.map(Into::into);
        this
    }

    /// The total number of gifts of this type that can be sent by the bot; for limited gifts only
    #[must_use]
    pub fn personal_total_count<T: Into<i64>>(self, val: T) -> Self {
        let mut this = self;
        this.personal_total_count = Some(val.into());
        this
    }

    /// The total number of gifts of this type that can be sent by the bot; for limited gifts only
    #[must_use]
    pub fn personal_total_count_option<T: Into<i64>>(self, val: Option<T>) -> Self {
        let mut this = self;
        this.personal_total_count = val.map(Into::into);
        this
    }

    /// The number of remaining gifts of this type that can be sent by the bot; for limited gifts only
    #[must_use]
    pub fn personal_remaining_count<T: Into<i64>>(self, val: T) -> Self {
        let mut this = self;
        this.personal_remaining_count = Some(val.into());
        this
    }

    /// The number of remaining gifts of this type that can be sent by the bot; for limited gifts only
    #[must_use]
    pub fn personal_remaining_count_option<T: Into<i64>>(self, val: Option<T>) -> Self {
        let mut this = self;
        this.personal_remaining_count = val.map(Into::into);
        this
    }

    /// Background of the gift
    #[must_use]
    pub fn background<T: Into<crate::types::GiftBackground>>(self, val: T) -> Self {
        let mut this = self;
        this.background = Some(val.into());
        this
    }

    /// Background of the gift
    #[must_use]
    pub fn background_option<T: Into<crate::types::GiftBackground>>(self, val: Option<T>) -> Self {
        let mut this = self;
        this.background = val.map(Into::into);
        this
    }

    /// The total number of different unique gifts that can be obtained by upgrading the gift
    #[must_use]
    pub fn unique_gift_variant_count<T: Into<i64>>(self, val: T) -> Self {
        let mut this = self;
        this.unique_gift_variant_count = Some(val.into());
        this
    }

    /// The total number of different unique gifts that can be obtained by upgrading the gift
    #[must_use]
    pub fn unique_gift_variant_count_option<T: Into<i64>>(self, val: Option<T>) -> Self {
        let mut this = self;
        this.unique_gift_variant_count = val.map(Into::into);
        this
    }

    /// Information about the chat that published the gift
    #[must_use]
    pub fn publisher_chat<T: Into<crate::types::Chat>>(self, val: T) -> Self {
        let mut this = self;
        this.publisher_chat = Some(Box::new(val.into()));
        this
    }

    /// Information about the chat that published the gift
    #[must_use]
    pub fn publisher_chat_option<T: Into<crate::types::Chat>>(self, val: Option<T>) -> Self {
        let mut this = self;
        this.publisher_chat = val.map(|val| Box::new(val.into()));
        this
    }
}
