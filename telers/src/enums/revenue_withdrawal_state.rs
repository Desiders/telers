use crate::types::RevenueWithdrawalState;
use serde::{Deserialize, Serialize};
use strum_macros::{AsRefStr, Display, EnumString, IntoStaticStr};
/// This object describes the state of a revenue withdrawal operation. Currently, it can be one of
/// - [`crate::types::RevenueWithdrawalStatePending`]
/// - [`crate::types::RevenueWithdrawalStateSucceeded`]
/// - [`crate::types::RevenueWithdrawalStateFailed`]
/// # Documentation
/// <https://core.telegram.org/bots/api#revenuewithdrawalstate>
#[derive(
    Debug,
    Display,
    Clone,
    Copy,
    PartialEq,
    Eq,
    Hash,
    EnumString,
    AsRefStr,
    IntoStaticStr,
    Deserialize,
    Serialize,
)]
pub enum RevenueWithdrawalStateType {
    #[strum(serialize = "pending")]
    Pending,
    #[strum(serialize = "succeeded")]
    Succeeded,
    #[strum(serialize = "failed")]
    Failed,
    #[strum(serialize = "unknown")]
    Unknown,
}
impl RevenueWithdrawalStateType {
    #[must_use]
    pub const fn all() -> [RevenueWithdrawalStateType; 4usize] {
        [
            RevenueWithdrawalStateType::Pending,
            RevenueWithdrawalStateType::Succeeded,
            RevenueWithdrawalStateType::Failed,
            RevenueWithdrawalStateType::Unknown,
        ]
    }
}
impl From<RevenueWithdrawalStateType> for Box<str> {
    fn from(val: RevenueWithdrawalStateType) -> Self {
        Into::<&'static str>::into(val).into()
    }
}
impl From<RevenueWithdrawalStateType> for String {
    fn from(val: RevenueWithdrawalStateType) -> Self {
        val.as_ref().to_owned()
    }
}
impl<'a> PartialEq<&'a str> for RevenueWithdrawalStateType {
    fn eq(&self, other: &&'a str) -> bool {
        self.as_ref() == *other
    }
}
impl<'a> From<&'a RevenueWithdrawalState> for RevenueWithdrawalStateType {
    fn from(val: &'a RevenueWithdrawalState) -> Self {
        match val {
            RevenueWithdrawalState::Pending(_) => RevenueWithdrawalStateType::Pending,
            RevenueWithdrawalState::Succeeded(_) => RevenueWithdrawalStateType::Succeeded,
            RevenueWithdrawalState::Failed(_) => RevenueWithdrawalStateType::Failed,
            RevenueWithdrawalState::Unknown(_) => RevenueWithdrawalStateType::Unknown,
        }
    }
}
impl From<RevenueWithdrawalState> for RevenueWithdrawalStateType {
    fn from(val: RevenueWithdrawalState) -> Self {
        RevenueWithdrawalStateType::from(&val)
    }
}
