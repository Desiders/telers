use serde::{Deserialize, Serialize};
/// This object represents a premium giveaway.
/// # Notes
/// This object represents a giveaway from original giveaway type `premium`.
/// # Documentation
/// <https://core.telegram.org/bots/api#giveaway>
#[derive(Clone, Debug, Serialize, Deserialize)]
pub struct GiveawayPremium {
    /// The list of chats which the user must join to participate in the giveaway
    pub chats: Box<[crate::types::Chat]>,
    /// Point in time (Unix timestamp) when winners of the giveaway will be selected
    pub winners_selection_date: i64,
    /// The number of users which are supposed to be selected as winners of the giveaway
    pub winner_count: i64,
    /// `true`, if only users who join the chats after the giveaway started should be eligible to win
    #[serde(skip_serializing_if = "Option::is_none")]
    pub only_new_members: Option<bool>,
    /// `true`, if the list of giveaway winners will be visible to everyone
    #[serde(skip_serializing_if = "Option::is_none")]
    pub has_public_winners: Option<bool>,
    /// Description of additional giveaway prize
    #[serde(skip_serializing_if = "Option::is_none")]
    pub prize_description: Option<Box<str>>,
    /// A list of two-letter ISO 3166-1 alpha-2 country codes indicating the countries from which eligible users for the giveaway must come. If empty, then all users can participate in the giveaway. Users with a phone number that was bought on Fragment can always participate in giveaways.
    #[serde(skip_serializing_if = "Option::is_none")]
    pub country_codes: Option<Box<[Box<str>]>>,
    /// The number of months the Telegram Premium subscription won from the giveaway will be active for; for Telegram Premium giveaways only
    pub premium_subscription_month_count: i64,
}
impl GiveawayPremium {
    /// Creates a new `GiveawayPremium`.
    ///
    /// # Arguments
    /// * `chats` - The list of chats which the user must join to participate in the giveaway
    /// * `winners_selection_date` - Point in time (Unix timestamp) when winners of the giveaway will be selected
    /// * `winner_count` - The number of users which are supposed to be selected as winners of the giveaway
    /// * `premium_subscription_month_count` - The number of months the Telegram Premium subscription won from the giveaway will be active for; for Telegram Premium giveaways only
    ///
    /// # Notes
    /// Use builder methods to set optional fields.
    #[must_use]
    pub fn new<
        T0Item: Into<crate::types::Chat>,
        T0: IntoIterator<Item = T0Item>,
        T1: Into<i64>,
        T2: Into<i64>,
        T3: Into<i64>,
    >(
        chats: T0,
        winners_selection_date: T1,
        winner_count: T2,
        premium_subscription_month_count: T3,
    ) -> Self {
        Self {
            chats: chats.into_iter().map(Into::into).collect(),
            winners_selection_date: winners_selection_date.into(),
            winner_count: winner_count.into(),
            only_new_members: None,
            has_public_winners: None,
            prize_description: None,
            country_codes: None,
            premium_subscription_month_count: premium_subscription_month_count.into(),
        }
    }

    /// The list of chats which the user must join to participate in the giveaway
    ///
    /// # Notes
    /// Adds multiple elements.
    #[must_use]
    pub fn chats<T: Into<Box<[crate::types::Chat]>>>(mut self, val: T) -> Self {
        self.chats = self
            .chats
            .into_vec()
            .into_iter()
            .chain(val.into())
            .collect();
        self
    }

    /// The list of chats which the user must join to participate in the giveaway
    ///
    /// # Notes
    /// Adds a single element.
    #[must_use]
    pub fn chat<T: Into<crate::types::Chat>>(mut self, val: T) -> Self {
        self.chats = self
            .chats
            .into_vec()
            .into_iter()
            .chain(Some(val.into()))
            .collect();
        self
    }

    /// Point in time (Unix timestamp) when winners of the giveaway will be selected
    #[must_use]
    pub fn winners_selection_date<T: Into<i64>>(mut self, val: T) -> Self {
        self.winners_selection_date = val.into();
        self
    }

    /// The number of users which are supposed to be selected as winners of the giveaway
    #[must_use]
    pub fn winner_count<T: Into<i64>>(mut self, val: T) -> Self {
        self.winner_count = val.into();
        self
    }

    /// `true`, if only users who join the chats after the giveaway started should be eligible to win
    #[must_use]
    pub fn only_new_members<T: Into<bool>>(mut self, val: T) -> Self {
        self.only_new_members = Some(val.into());
        self
    }

    /// `true`, if only users who join the chats after the giveaway started should be eligible to win
    #[must_use]
    pub fn only_new_members_option<T: Into<bool>>(mut self, val: Option<T>) -> Self {
        self.only_new_members = val.map(Into::into);
        self
    }

    /// `true`, if the list of giveaway winners will be visible to everyone
    #[must_use]
    pub fn has_public_winners<T: Into<bool>>(mut self, val: T) -> Self {
        self.has_public_winners = Some(val.into());
        self
    }

    /// `true`, if the list of giveaway winners will be visible to everyone
    #[must_use]
    pub fn has_public_winners_option<T: Into<bool>>(mut self, val: Option<T>) -> Self {
        self.has_public_winners = val.map(Into::into);
        self
    }

    /// Description of additional giveaway prize
    #[must_use]
    pub fn prize_description<T: Into<Box<str>>>(mut self, val: T) -> Self {
        self.prize_description = Some(val.into());
        self
    }

    /// Description of additional giveaway prize
    #[must_use]
    pub fn prize_description_option<T: Into<Box<str>>>(mut self, val: Option<T>) -> Self {
        self.prize_description = val.map(Into::into);
        self
    }

    /// A list of two-letter ISO 3166-1 alpha-2 country codes indicating the countries from which eligible users for the giveaway must come. If empty, then all users can participate in the giveaway. Users with a phone number that was bought on Fragment can always participate in giveaways.
    ///
    /// # Notes
    /// Adds multiple elements.
    #[must_use]
    pub fn country_codes<T: Into<Box<[Box<str>]>>>(mut self, val: T) -> Self {
        self.country_codes = Some(
            self.country_codes
                .unwrap_or_default()
                .into_vec()
                .into_iter()
                .chain(val.into())
                .collect(),
        );
        self
    }

    /// A list of two-letter ISO 3166-1 alpha-2 country codes indicating the countries from which eligible users for the giveaway must come. If empty, then all users can participate in the giveaway. Users with a phone number that was bought on Fragment can always participate in giveaways.
    ///
    /// # Notes
    /// Adds a single element.
    #[must_use]
    pub fn country_code<T: Into<Box<str>>>(mut self, val: T) -> Self {
        self.country_codes = Some(
            self.country_codes
                .unwrap_or_default()
                .into_vec()
                .into_iter()
                .chain(Some(val.into()))
                .collect(),
        );
        self
    }

    /// A list of two-letter ISO 3166-1 alpha-2 country codes indicating the countries from which eligible users for the giveaway must come. If empty, then all users can participate in the giveaway. Users with a phone number that was bought on Fragment can always participate in giveaways.
    ///
    /// # Notes
    /// Adds a single element.
    #[must_use]
    pub fn country_codes_option<T: Into<Box<[Box<str>]>>>(mut self, val: Option<T>) -> Self {
        self.country_codes = val.map(Into::into);
        self
    }

    /// The number of months the Telegram Premium subscription won from the giveaway will be active for; for Telegram Premium giveaways only
    #[must_use]
    pub fn premium_subscription_month_count<T: Into<i64>>(mut self, val: T) -> Self {
        self.premium_subscription_month_count = val.into();
        self
    }
}
