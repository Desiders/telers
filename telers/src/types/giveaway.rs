use serde::{Deserialize, Serialize};
/// This object represents a message about a scheduled giveaway.
/// Currently, it can be one of
/// - [`crate::types::GiveawayPremium`]
/// - [`crate::types::GiveawayStar`]
/// # Documentation
/// <https://core.telegram.org/bots/api#giveaway>
#[derive(Clone, Debug, Serialize, Deserialize)]
#[serde(untagged)]
pub enum Giveaway {
    Premium(crate::types::GiveawayPremium),
    Star(crate::types::GiveawayStar),
    Unknown(crate::types::GiveawayUnknown),
}
impl Giveaway {
    /// Helper method for field `chats`.
    ///
    /// The list of chats which the user must join to participate in the giveaway
    #[must_use]
    pub fn chats(&self) -> &[crate::types::Chat] {
        match self {
            Self::Premium(val) => val.chats.as_ref(),
            Self::Star(val) => val.chats.as_ref(),
            Self::Unknown(val) => val.chats.as_ref(),
        }
    }

    /// Helper method for field `country_codes`.
    ///
    /// A list of two-letter ISO 3166-1 alpha-2 country codes indicating the countries from which eligible users for the giveaway must come. If empty, then all users can participate in the giveaway. Users with a phone number that was bought on Fragment can always participate in giveaways.
    #[must_use]
    pub fn country_codes(&self) -> Option<&[Box<str>]> {
        match self {
            Self::Premium(val) => val.country_codes.as_deref(),
            Self::Star(val) => val.country_codes.as_deref(),
            Self::Unknown(val) => val.country_codes.as_deref(),
        }
    }

    /// Helper method for field `has_public_winners`.
    ///
    /// `true`, if the list of giveaway winners will be visible to everyone
    #[must_use]
    pub fn has_public_winners(&self) -> Option<bool> {
        match self {
            Self::Premium(val) => val.has_public_winners,
            Self::Star(val) => val.has_public_winners,
            Self::Unknown(val) => val.has_public_winners,
        }
    }

    /// Helper method for field `only_new_members`.
    ///
    /// `true`, if only users who join the chats after the giveaway started should be eligible to win
    #[must_use]
    pub fn only_new_members(&self) -> Option<bool> {
        match self {
            Self::Premium(val) => val.only_new_members,
            Self::Star(val) => val.only_new_members,
            Self::Unknown(val) => val.only_new_members,
        }
    }

    /// Helper method for field `premium_subscription_month_count`.
    ///
    /// The number of months the Telegram Premium subscription won from the giveaway will be active for; for Telegram Premium giveaways only
    #[must_use]
    pub fn premium_subscription_month_count(&self) -> Option<i64> {
        match self {
            Self::Premium(val) => Some(val.premium_subscription_month_count),
            _ => None,
        }
    }

    /// Helper method for field `prize_description`.
    ///
    /// Description of additional giveaway prize
    #[must_use]
    pub fn prize_description(&self) -> Option<&str> {
        match self {
            Self::Premium(val) => val.prize_description.as_deref(),
            Self::Star(val) => val.prize_description.as_deref(),
            Self::Unknown(val) => val.prize_description.as_deref(),
        }
    }

    /// Helper method for field `prize_star_count`.
    ///
    /// The number of Telegram Stars to be split between giveaway winners; for Telegram Star giveaways only
    #[must_use]
    pub fn prize_star_count(&self) -> Option<i64> {
        match self {
            Self::Star(val) => Some(val.prize_star_count),
            _ => None,
        }
    }

    /// Helper method for field `winner_count`.
    ///
    /// The number of users which are supposed to be selected as winners of the giveaway
    #[must_use]
    pub fn winner_count(&self) -> i64 {
        match self {
            Self::Premium(val) => val.winner_count,
            Self::Star(val) => val.winner_count,
            Self::Unknown(val) => val.winner_count,
        }
    }

    /// Helper method for field `winners_selection_date`.
    ///
    /// Point in time (Unix timestamp) when winners of the giveaway will be selected
    #[must_use]
    pub fn winners_selection_date(&self) -> i64 {
        match self {
            Self::Premium(val) => val.winners_selection_date,
            Self::Star(val) => val.winners_selection_date,
            Self::Unknown(val) => val.winners_selection_date,
        }
    }
}
impl From<crate::types::GiveawayPremium> for Giveaway {
    fn from(val: crate::types::GiveawayPremium) -> Self {
        Self::Premium(val)
    }
}
impl TryFrom<Giveaway> for crate::types::GiveawayPremium {
    type Error = crate::errors::ConvertToTypeError;

    fn try_from(val: Giveaway) -> Result<Self, Self::Error> {
        if let Giveaway::Premium(inner) = val {
            Ok(inner)
        } else {
            Err(Self::Error::new(
                stringify!(Giveaway),
                stringify!(GiveawayPremium),
            ))
        }
    }
}
impl From<crate::types::GiveawayStar> for Giveaway {
    fn from(val: crate::types::GiveawayStar) -> Self {
        Self::Star(val)
    }
}
impl TryFrom<Giveaway> for crate::types::GiveawayStar {
    type Error = crate::errors::ConvertToTypeError;

    fn try_from(val: Giveaway) -> Result<Self, Self::Error> {
        if let Giveaway::Star(inner) = val {
            Ok(inner)
        } else {
            Err(Self::Error::new(
                stringify!(Giveaway),
                stringify!(GiveawayStar),
            ))
        }
    }
}
impl From<crate::types::GiveawayUnknown> for Giveaway {
    fn from(val: crate::types::GiveawayUnknown) -> Self {
        Self::Unknown(val)
    }
}
impl TryFrom<Giveaway> for crate::types::GiveawayUnknown {
    type Error = crate::errors::ConvertToTypeError;

    fn try_from(val: Giveaway) -> Result<Self, Self::Error> {
        if let Giveaway::Unknown(inner) = val {
            Ok(inner)
        } else {
            Err(Self::Error::new(
                stringify!(Giveaway),
                stringify!(GiveawayUnknown),
            ))
        }
    }
}
