use serde::{Deserialize, Serialize};
/// This object represents a star giveaway winners.
/// # Notes
/// This object represents giveaway winners from original field `star`.
/// # Documentation
/// <https://core.telegram.org/bots/api#giveawaywinners>
#[derive(Clone, Debug, Serialize, Deserialize)]
pub struct GiveawayWinnersStar {
    /// The chat that created the giveaway
    pub chat: Box<crate::types::Chat>,
    /// Identifier of the message with the giveaway in the chat
    pub giveaway_message_id: i64,
    /// Point in time (Unix timestamp) when winners of the giveaway were selected
    pub winners_selection_date: i64,
    /// Total number of winners in the giveaway
    pub winner_count: i64,
    /// List of up to 100 winners of the giveaway
    pub winners: Box<[crate::types::User]>,
    /// The number of other chats the user had to join in order to be eligible for the giveaway
    #[serde(skip_serializing_if = "Option::is_none")]
    pub additional_chat_count: Option<i64>,
    /// The number of Telegram Stars that were split between giveaway winners; for Telegram Star giveaways only
    pub prize_star_count: i64,
    /// Number of undistributed prizes
    #[serde(skip_serializing_if = "Option::is_none")]
    pub unclaimed_prize_count: Option<i64>,
    /// `true`, if only users who had joined the chats after the giveaway started were eligible to win
    #[serde(skip_serializing_if = "Option::is_none")]
    pub only_new_members: Option<bool>,
    /// `true`, if the giveaway was canceled because the payment for it was refunded
    #[serde(skip_serializing_if = "Option::is_none")]
    pub was_refunded: Option<bool>,
    /// Description of additional giveaway prize
    #[serde(skip_serializing_if = "Option::is_none")]
    pub prize_description: Option<Box<str>>,
}
impl GiveawayWinnersStar {
    /// Creates a new `GiveawayWinnersStar`.
    ///
    /// # Arguments
    /// * `chat` - The chat that created the giveaway
    /// * `giveaway_message_id` - Identifier of the message with the giveaway in the chat
    /// * `winners_selection_date` - Point in time (Unix timestamp) when winners of the giveaway were selected
    /// * `winner_count` - Total number of winners in the giveaway
    /// * `winners` - List of up to 100 winners of the giveaway
    /// * `prize_star_count` - The number of Telegram Stars that were split between giveaway winners; for Telegram Star giveaways only
    ///
    /// # Notes
    /// Use builder methods to set optional fields.
    #[must_use]
    pub fn new<
        T0: Into<crate::types::Chat>,
        T1: Into<i64>,
        T2: Into<i64>,
        T3: Into<i64>,
        T4Item: Into<crate::types::User>,
        T4: IntoIterator<Item = T4Item>,
        T5: Into<i64>,
    >(
        chat: T0,
        giveaway_message_id: T1,
        winners_selection_date: T2,
        winner_count: T3,
        winners: T4,
        prize_star_count: T5,
    ) -> Self {
        Self {
            chat: Box::new(chat.into()),
            giveaway_message_id: giveaway_message_id.into(),
            winners_selection_date: winners_selection_date.into(),
            winner_count: winner_count.into(),
            winners: winners.into_iter().map(Into::into).collect(),
            additional_chat_count: None,
            prize_star_count: prize_star_count.into(),
            unclaimed_prize_count: None,
            only_new_members: None,
            was_refunded: None,
            prize_description: None,
        }
    }

    /// The chat that created the giveaway
    #[must_use]
    pub fn chat<T: Into<crate::types::Chat>>(self, val: T) -> Self {
        let mut this = self;
        this.chat = Box::new(val.into());
        this
    }

    /// Identifier of the message with the giveaway in the chat
    #[must_use]
    pub fn giveaway_message_id<T: Into<i64>>(self, val: T) -> Self {
        let mut this = self;
        this.giveaway_message_id = val.into();
        this
    }

    /// Point in time (Unix timestamp) when winners of the giveaway were selected
    #[must_use]
    pub fn winners_selection_date<T: Into<i64>>(self, val: T) -> Self {
        let mut this = self;
        this.winners_selection_date = val.into();
        this
    }

    /// Total number of winners in the giveaway
    #[must_use]
    pub fn winner_count<T: Into<i64>>(self, val: T) -> Self {
        let mut this = self;
        this.winner_count = val.into();
        this
    }

    /// List of up to 100 winners of the giveaway
    ///
    /// # Notes
    /// Adds multiple elements.
    #[must_use]
    pub fn winners<T: Into<Box<[crate::types::User]>>>(self, val: T) -> Self {
        let mut this = self;
        this.winners = this
            .winners
            .into_vec()
            .into_iter()
            .chain(val.into())
            .collect();
        this
    }

    /// List of up to 100 winners of the giveaway
    ///
    /// # Notes
    /// Adds a single element.
    #[must_use]
    pub fn winner<T: Into<crate::types::User>>(self, val: T) -> Self {
        let mut this = self;
        this.winners = this
            .winners
            .into_vec()
            .into_iter()
            .chain(Some(val.into()))
            .collect();
        this
    }

    /// The number of other chats the user had to join in order to be eligible for the giveaway
    #[must_use]
    pub fn additional_chat_count<T: Into<i64>>(self, val: T) -> Self {
        let mut this = self;
        this.additional_chat_count = Some(val.into());
        this
    }

    /// The number of other chats the user had to join in order to be eligible for the giveaway
    #[must_use]
    pub fn additional_chat_count_option<T: Into<i64>>(self, val: Option<T>) -> Self {
        let mut this = self;
        this.additional_chat_count = val.map(Into::into);
        this
    }

    /// The number of Telegram Stars that were split between giveaway winners; for Telegram Star giveaways only
    #[must_use]
    pub fn prize_star_count<T: Into<i64>>(self, val: T) -> Self {
        let mut this = self;
        this.prize_star_count = val.into();
        this
    }

    /// Number of undistributed prizes
    #[must_use]
    pub fn unclaimed_prize_count<T: Into<i64>>(self, val: T) -> Self {
        let mut this = self;
        this.unclaimed_prize_count = Some(val.into());
        this
    }

    /// Number of undistributed prizes
    #[must_use]
    pub fn unclaimed_prize_count_option<T: Into<i64>>(self, val: Option<T>) -> Self {
        let mut this = self;
        this.unclaimed_prize_count = val.map(Into::into);
        this
    }

    /// `true`, if only users who had joined the chats after the giveaway started were eligible to win
    #[must_use]
    pub fn only_new_members<T: Into<bool>>(self, val: T) -> Self {
        let mut this = self;
        this.only_new_members = Some(val.into());
        this
    }

    /// `true`, if only users who had joined the chats after the giveaway started were eligible to win
    #[must_use]
    pub fn only_new_members_option<T: Into<bool>>(self, val: Option<T>) -> Self {
        let mut this = self;
        this.only_new_members = val.map(Into::into);
        this
    }

    /// `true`, if the giveaway was canceled because the payment for it was refunded
    #[must_use]
    pub fn was_refunded<T: Into<bool>>(self, val: T) -> Self {
        let mut this = self;
        this.was_refunded = Some(val.into());
        this
    }

    /// `true`, if the giveaway was canceled because the payment for it was refunded
    #[must_use]
    pub fn was_refunded_option<T: Into<bool>>(self, val: Option<T>) -> Self {
        let mut this = self;
        this.was_refunded = val.map(Into::into);
        this
    }

    /// Description of additional giveaway prize
    #[must_use]
    pub fn prize_description<T: Into<Box<str>>>(self, val: T) -> Self {
        let mut this = self;
        this.prize_description = Some(val.into());
        this
    }

    /// Description of additional giveaway prize
    #[must_use]
    pub fn prize_description_option<T: Into<Box<str>>>(self, val: Option<T>) -> Self {
        let mut this = self;
        this.prize_description = val.map(Into::into);
        this
    }
}
