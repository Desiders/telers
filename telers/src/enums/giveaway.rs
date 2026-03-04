use crate::types::Giveaway;
use strum_macros::{AsRefStr, Display, EnumString, IntoStaticStr};
/// This object represents a message about a scheduled giveaway.
/// Currently, it can be one of
/// - [`crate::types::GiveawayPremium`]
/// - [`crate::types::GiveawayStar`]
/// # Documentation
/// <https://core.telegram.org/bots/api#giveaway>
#[derive(Debug, Display, Clone, Copy, PartialEq, Eq, Hash, EnumString, AsRefStr, IntoStaticStr)]
pub enum GiveawayType {
    #[strum(serialize = "premium")]
    Premium,
    #[strum(serialize = "star")]
    Star,
}
impl GiveawayType {
    #[must_use]
    pub const fn all() -> [GiveawayType; 2usize] {
        [GiveawayType::Premium, GiveawayType::Star]
    }
}
impl From<GiveawayType> for Box<str> {
    fn from(val: GiveawayType) -> Self {
        Into::<&'static str>::into(val).into()
    }
}
impl From<GiveawayType> for String {
    fn from(val: GiveawayType) -> Self {
        val.as_ref().to_owned()
    }
}
impl<'a> PartialEq<&'a str> for GiveawayType {
    fn eq(&self, other: &&'a str) -> bool {
        self.as_ref() == *other
    }
}
impl<'a> From<&'a Giveaway> for GiveawayType {
    fn from(val: &'a Giveaway) -> Self {
        match val {
            Giveaway::Premium(_) => GiveawayType::Premium,
            Giveaway::Star(_) => GiveawayType::Star,
        }
    }
}
impl From<Giveaway> for GiveawayType {
    fn from(val: Giveaway) -> Self {
        GiveawayType::from(&val)
    }
}
