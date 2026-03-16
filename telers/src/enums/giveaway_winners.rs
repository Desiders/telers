use crate::types::GiveawayWinners;
use strum_macros::{AsRefStr, Display, EnumString, IntoStaticStr};
/// This object represents a message about the completion of a giveaway with public winners.
/// Currently, it can be one of
/// - [`crate::types::GiveawayWinnersPremium`]
/// - [`crate::types::GiveawayWinnersStar`]
/// # Documentation
/// <https://core.telegram.org/bots/api#giveawaywinners>
#[derive(Debug, Display, Clone, Copy, PartialEq, Eq, Hash, EnumString, AsRefStr, IntoStaticStr)]
pub enum GiveawayWinnersType {
    #[strum(serialize = "premium")]
    Premium,
    #[strum(serialize = "star")]
    Star,
}
impl GiveawayWinnersType {
    #[must_use]
    pub const fn all() -> [GiveawayWinnersType; 2usize] {
        [GiveawayWinnersType::Premium, GiveawayWinnersType::Star]
    }
}
impl From<GiveawayWinnersType> for Box<str> {
    fn from(val: GiveawayWinnersType) -> Self {
        Into::<&'static str>::into(val).into()
    }
}
impl From<GiveawayWinnersType> for String {
    fn from(val: GiveawayWinnersType) -> Self {
        val.as_ref().to_owned()
    }
}
impl<'a> PartialEq<&'a str> for GiveawayWinnersType {
    fn eq(&self, other: &&'a str) -> bool {
        self.as_ref() == *other
    }
}
impl<'a> From<&'a GiveawayWinners> for GiveawayWinnersType {
    fn from(val: &'a GiveawayWinners) -> Self {
        match val {
            GiveawayWinners::Premium(_) => GiveawayWinnersType::Premium,
            GiveawayWinners::Star(_) => GiveawayWinnersType::Star,
        }
    }
}
impl From<GiveawayWinners> for GiveawayWinnersType {
    fn from(val: GiveawayWinners) -> Self {
        GiveawayWinnersType::from(&val)
    }
}
