use serde::{Deserialize, Serialize};
/// This object represents a message about the completion of a giveaway with public winners.
/// Currently, it can be one of
/// - [`GiveawayWinnersPremium`]
/// - [`GiveawayWinnersStar`]
/// # Documentation
/// <https://core.telegram.org/bots/api#giveawaywinners>
#[derive(Clone, Debug, Serialize, Deserialize)]
#[serde(untagged)]
pub enum GiveawayWinners {
    Premium(crate::types::GiveawayWinnersPremium),
    Star(crate::types::GiveawayWinnersStar),
}
impl GiveawayWinners {
    /// Helper method for field `additional_chat_count`.
    ///
    /// # Variants
    /// - `GiveawayWinnersPremium`. The number of other chats the user had to join in order to be eligible for the giveaway
    /// - `GiveawayWinnersStar`. The number of other chats the user had to join in order to be eligible for the giveaway
    #[must_use]
    pub fn additional_chat_count(&self) -> Option<i64> {
        match self {
            Self::Premium(val) => val.additional_chat_count,
            Self::Star(val) => val.additional_chat_count,
        }
    }

    /// Helper method for field `chat`.
    ///
    /// # Variants
    /// - `GiveawayWinnersPremium`. The chat that created the giveaway
    /// - `GiveawayWinnersStar`. The chat that created the giveaway
    #[must_use]
    pub fn chat(&self) -> &crate::types::Chat {
        match self {
            Self::Premium(val) => val.chat.as_ref(),
            Self::Star(val) => val.chat.as_ref(),
        }
    }

    /// Helper method for field `giveaway_message_id`.
    ///
    /// # Variants
    /// - `GiveawayWinnersPremium`. Identifier of the message with the giveaway in the chat
    /// - `GiveawayWinnersStar`. Identifier of the message with the giveaway in the chat
    #[must_use]
    pub fn giveaway_message_id(&self) -> i64 {
        match self {
            Self::Premium(val) => val.giveaway_message_id,
            Self::Star(val) => val.giveaway_message_id,
        }
    }

    /// Helper method for field `only_new_members`.
    ///
    /// # Variants
    /// - `GiveawayWinnersPremium`. `true`, if only users who had joined the chats after the giveaway started were eligible to win
    /// - `GiveawayWinnersStar`. `true`, if only users who had joined the chats after the giveaway started were eligible to win
    #[must_use]
    pub fn only_new_members(&self) -> Option<bool> {
        match self {
            Self::Premium(val) => val.only_new_members,
            Self::Star(val) => val.only_new_members,
        }
    }

    /// Helper method for field `premium_subscription_month_count`.
    ///
    /// # Variants
    /// - `GiveawayWinnersPremium`. The number of months the Telegram Premium subscription won from the giveaway will be active for; for Telegram Premium giveaways only
    #[must_use]
    pub fn premium_subscription_month_count(&self) -> Option<i64> {
        match self {
            Self::Premium(val) => Some(val.premium_subscription_month_count),
            Self::Star(_) => None,
        }
    }

    /// Helper method for field `prize_description`.
    ///
    /// # Variants
    /// - `GiveawayWinnersPremium`. Description of additional giveaway prize
    /// - `GiveawayWinnersStar`. Description of additional giveaway prize
    #[must_use]
    pub fn prize_description(&self) -> Option<&str> {
        match self {
            Self::Premium(val) => val.prize_description.as_deref(),
            Self::Star(val) => val.prize_description.as_deref(),
        }
    }

    /// Helper method for field `prize_star_count`.
    ///
    /// # Variants
    /// - `GiveawayWinnersStar`. The number of Telegram Stars that were split between giveaway winners; for Telegram Star giveaways only
    #[must_use]
    pub fn prize_star_count(&self) -> Option<i64> {
        match self {
            Self::Star(val) => Some(val.prize_star_count),
            Self::Premium(_) => None,
        }
    }

    /// Helper method for field `unclaimed_prize_count`.
    ///
    /// # Variants
    /// - `GiveawayWinnersPremium`. Number of undistributed prizes
    /// - `GiveawayWinnersStar`. Number of undistributed prizes
    #[must_use]
    pub fn unclaimed_prize_count(&self) -> Option<i64> {
        match self {
            Self::Premium(val) => val.unclaimed_prize_count,
            Self::Star(val) => val.unclaimed_prize_count,
        }
    }

    /// Helper method for field `was_refunded`.
    ///
    /// # Variants
    /// - `GiveawayWinnersPremium`. `true`, if the giveaway was canceled because the payment for it was refunded
    /// - `GiveawayWinnersStar`. `true`, if the giveaway was canceled because the payment for it was refunded
    #[must_use]
    pub fn was_refunded(&self) -> Option<bool> {
        match self {
            Self::Premium(val) => val.was_refunded,
            Self::Star(val) => val.was_refunded,
        }
    }

    /// Helper method for field `winner_count`.
    ///
    /// # Variants
    /// - `GiveawayWinnersPremium`. Total number of winners in the giveaway
    /// - `GiveawayWinnersStar`. Total number of winners in the giveaway
    #[must_use]
    pub fn winner_count(&self) -> i64 {
        match self {
            Self::Premium(val) => val.winner_count,
            Self::Star(val) => val.winner_count,
        }
    }

    /// Helper method for field `winners`.
    ///
    /// # Variants
    /// - `GiveawayWinnersPremium`. List of up to 100 winners of the giveaway
    /// - `GiveawayWinnersStar`. List of up to 100 winners of the giveaway
    #[must_use]
    pub fn winners(&self) -> &[crate::types::User] {
        match self {
            Self::Premium(val) => val.winners.as_ref(),
            Self::Star(val) => val.winners.as_ref(),
        }
    }

    /// Helper method for field `winners_selection_date`.
    ///
    /// # Variants
    /// - `GiveawayWinnersPremium`. Point in time (Unix timestamp) when winners of the giveaway were selected
    /// - `GiveawayWinnersStar`. Point in time (Unix timestamp) when winners of the giveaway were selected
    #[must_use]
    pub fn winners_selection_date(&self) -> i64 {
        match self {
            Self::Premium(val) => val.winners_selection_date,
            Self::Star(val) => val.winners_selection_date,
        }
    }

    /// Helper method for nested field `first_name`.
    #[must_use]
    pub fn first_name(&self) -> Option<&str> {
        crate::types::Chat::first_name(self.chat())
    }

    /// Helper method for nested field `id`.
    #[must_use]
    pub fn id(&self) -> i64 {
        crate::types::Chat::id(self.chat())
    }

    /// Helper method for nested field `is_direct_messages`.
    #[must_use]
    pub fn is_direct_messages(&self) -> Option<bool> {
        crate::types::Chat::is_direct_messages(self.chat())
    }

    /// Helper method for nested field `is_forum`.
    #[must_use]
    pub fn is_forum(&self) -> Option<bool> {
        crate::types::Chat::is_forum(self.chat())
    }

    /// Helper method for nested field `last_name`.
    #[must_use]
    pub fn last_name(&self) -> Option<&str> {
        crate::types::Chat::last_name(self.chat())
    }

    /// Helper method for nested field `title`.
    #[must_use]
    pub fn title(&self) -> Option<&str> {
        crate::types::Chat::title(self.chat())
    }

    /// Helper method for nested field `username`.
    #[must_use]
    pub fn username(&self) -> Option<&str> {
        crate::types::Chat::username(self.chat())
    }
}
impl From<crate::types::GiveawayWinnersPremium> for GiveawayWinners {
    fn from(val: crate::types::GiveawayWinnersPremium) -> Self {
        Self::Premium(val)
    }
}
impl TryFrom<GiveawayWinners> for crate::types::GiveawayWinnersPremium {
    type Error = crate::errors::ConvertToTypeError;

    fn try_from(val: GiveawayWinners) -> Result<Self, Self::Error> {
        match val {
            GiveawayWinners::Premium(inner) => Ok(inner),
            GiveawayWinners::Star(_) => Err(Self::Error::new(
                stringify!(GiveawayWinners),
                stringify!(GiveawayWinnersPremium),
            )),
        }
    }
}
impl From<crate::types::GiveawayWinnersStar> for GiveawayWinners {
    fn from(val: crate::types::GiveawayWinnersStar) -> Self {
        Self::Star(val)
    }
}
impl TryFrom<GiveawayWinners> for crate::types::GiveawayWinnersStar {
    type Error = crate::errors::ConvertToTypeError;

    fn try_from(val: GiveawayWinners) -> Result<Self, Self::Error> {
        match val {
            GiveawayWinners::Star(inner) => Ok(inner),
            GiveawayWinners::Premium(_) => Err(Self::Error::new(
                stringify!(GiveawayWinners),
                stringify!(GiveawayWinnersStar),
            )),
        }
    }
}
