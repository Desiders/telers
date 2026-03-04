use serde::{Deserialize, Serialize};
/// This object describes the state of a revenue withdrawal operation. Currently, it can be one of
/// - [`crate::types::RevenueWithdrawalStatePending`]
/// - [`crate::types::RevenueWithdrawalStateSucceeded`]
/// - [`crate::types::RevenueWithdrawalStateFailed`]
/// # Documentation
/// <https://core.telegram.org/bots/api#revenuewithdrawalstate>
#[derive(Clone, Debug, Serialize, Deserialize)]
#[serde(tag = "type", rename_all = "snake_case")]
pub enum RevenueWithdrawalState {
    Pending(crate::types::RevenueWithdrawalStatePending),
    Succeeded(crate::types::RevenueWithdrawalStateSucceeded),
    Failed(crate::types::RevenueWithdrawalStateFailed),
}
impl RevenueWithdrawalState {
    /// Helper method for field `date`.
    ///
    /// Date the withdrawal was completed in Unix time
    #[must_use]
    pub fn date(&self) -> Option<i64> {
        match self {
            Self::Succeeded(val) => Some(val.date),
            _ => None,
        }
    }

    /// Helper method for field `url`.
    ///
    /// An HTTPS URL that can be used to see transaction details
    #[must_use]
    pub fn url(&self) -> Option<&str> {
        match self {
            Self::Succeeded(val) => Some(val.url.as_ref()),
            _ => None,
        }
    }
}
impl From<crate::types::RevenueWithdrawalStatePending> for RevenueWithdrawalState {
    fn from(val: crate::types::RevenueWithdrawalStatePending) -> Self {
        Self::Pending(val)
    }
}
impl TryFrom<RevenueWithdrawalState> for crate::types::RevenueWithdrawalStatePending {
    type Error = crate::errors::ConvertToTypeError;

    fn try_from(val: RevenueWithdrawalState) -> Result<Self, Self::Error> {
        if let RevenueWithdrawalState::Pending(inner) = val {
            Ok(inner)
        } else {
            Err(Self::Error::new(
                stringify!(RevenueWithdrawalState),
                stringify!(RevenueWithdrawalStatePending),
            ))
        }
    }
}
impl From<crate::types::RevenueWithdrawalStateSucceeded> for RevenueWithdrawalState {
    fn from(val: crate::types::RevenueWithdrawalStateSucceeded) -> Self {
        Self::Succeeded(val)
    }
}
impl TryFrom<RevenueWithdrawalState> for crate::types::RevenueWithdrawalStateSucceeded {
    type Error = crate::errors::ConvertToTypeError;

    fn try_from(val: RevenueWithdrawalState) -> Result<Self, Self::Error> {
        if let RevenueWithdrawalState::Succeeded(inner) = val {
            Ok(inner)
        } else {
            Err(Self::Error::new(
                stringify!(RevenueWithdrawalState),
                stringify!(RevenueWithdrawalStateSucceeded),
            ))
        }
    }
}
impl From<crate::types::RevenueWithdrawalStateFailed> for RevenueWithdrawalState {
    fn from(val: crate::types::RevenueWithdrawalStateFailed) -> Self {
        Self::Failed(val)
    }
}
impl TryFrom<RevenueWithdrawalState> for crate::types::RevenueWithdrawalStateFailed {
    type Error = crate::errors::ConvertToTypeError;

    fn try_from(val: RevenueWithdrawalState) -> Result<Self, Self::Error> {
        if let RevenueWithdrawalState::Failed(inner) = val {
            Ok(inner)
        } else {
            Err(Self::Error::new(
                stringify!(RevenueWithdrawalState),
                stringify!(RevenueWithdrawalStateFailed),
            ))
        }
    }
}
