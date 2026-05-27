use serde::{Deserialize, Serialize};
/// Represents an invite link for a chat.
/// # Documentation
/// <https://core.telegram.org/bots/api#chatinvitelink>
#[derive(Clone, Debug, Serialize, Deserialize)]
pub struct ChatInviteLink {
    /// The invite link. If the link was created by another chat administrator, then the second part of the link will be replaced with `...`.
    pub invite_link: Box<str>,
    /// Creator of the link
    pub creator: Box<crate::types::User>,
    /// `true`, if users joining the chat via the link need to be approved by chat administrators
    pub creates_join_request: bool,
    /// `true`, if the link is primary
    pub is_primary: bool,
    /// `true`, if the link is revoked
    pub is_revoked: bool,
    /// Invite link name
    #[serde(skip_serializing_if = "Option::is_none")]
    pub name: Option<Box<str>>,
    /// Point in time (Unix timestamp) when the link will expire or has been expired
    #[serde(skip_serializing_if = "Option::is_none")]
    pub expire_date: Option<i64>,
    /// The maximum number of users that can be members of the chat simultaneously after joining the chat via this invite link; 1-99999
    #[serde(skip_serializing_if = "Option::is_none")]
    pub member_limit: Option<u32>,
    /// Number of pending join requests created using this link
    #[serde(skip_serializing_if = "Option::is_none")]
    pub pending_join_request_count: Option<i64>,
    /// The number of seconds the subscription will be active for before the next payment
    #[serde(skip_serializing_if = "Option::is_none")]
    pub subscription_period: Option<i64>,
    /// The amount of Telegram Stars a user must pay initially and after each subsequent subscription period to be a member of the chat using the link
    #[serde(skip_serializing_if = "Option::is_none")]
    pub subscription_price: Option<i64>,
}
impl ChatInviteLink {
    /// Creates a new `ChatInviteLink`.
    ///
    /// # Arguments
    /// * `invite_link` - The invite link. If the link was created by another chat administrator, then the second part of the link will be replaced with `...`.
    /// * `creator` - Creator of the link
    /// * `creates_join_request` - `true`, if users joining the chat via the link need to be approved by chat administrators
    /// * `is_primary` - `true`, if the link is primary
    /// * `is_revoked` - `true`, if the link is revoked
    ///
    /// # Notes
    /// Use builder methods to set optional fields.
    #[must_use]
    pub fn new<
        T0: Into<Box<str>>,
        T1: Into<crate::types::User>,
        T2: Into<bool>,
        T3: Into<bool>,
        T4: Into<bool>,
    >(
        invite_link: T0,
        creator: T1,
        creates_join_request: T2,
        is_primary: T3,
        is_revoked: T4,
    ) -> Self {
        Self {
            invite_link: invite_link.into(),
            creator: Box::new(creator.into()),
            creates_join_request: creates_join_request.into(),
            is_primary: is_primary.into(),
            is_revoked: is_revoked.into(),
            name: None,
            expire_date: None,
            member_limit: None,
            pending_join_request_count: None,
            subscription_period: None,
            subscription_price: None,
        }
    }

    /// The invite link. If the link was created by another chat administrator, then the second part of the link will be replaced with `...`.
    #[must_use]
    pub fn invite_link<T: Into<Box<str>>>(mut self, val: T) -> Self {
        self.invite_link = val.into();
        self
    }

    /// Creator of the link
    #[must_use]
    pub fn creator<T: Into<crate::types::User>>(mut self, val: T) -> Self {
        self.creator = Box::new(val.into());
        self
    }

    /// `true`, if users joining the chat via the link need to be approved by chat administrators
    #[must_use]
    pub fn creates_join_request<T: Into<bool>>(mut self, val: T) -> Self {
        self.creates_join_request = val.into();
        self
    }

    /// `true`, if the link is primary
    #[must_use]
    pub fn is_primary<T: Into<bool>>(mut self, val: T) -> Self {
        self.is_primary = val.into();
        self
    }

    /// `true`, if the link is revoked
    #[must_use]
    pub fn is_revoked<T: Into<bool>>(mut self, val: T) -> Self {
        self.is_revoked = val.into();
        self
    }

    /// Invite link name
    #[must_use]
    pub fn name<T: Into<Box<str>>>(mut self, val: T) -> Self {
        self.name = Some(val.into());
        self
    }

    /// Invite link name
    #[must_use]
    pub fn name_option<T: Into<Box<str>>>(mut self, val: Option<T>) -> Self {
        self.name = val.map(Into::into);
        self
    }

    /// Point in time (Unix timestamp) when the link will expire or has been expired
    #[must_use]
    pub fn expire_date<T: Into<i64>>(mut self, val: T) -> Self {
        self.expire_date = Some(val.into());
        self
    }

    /// Point in time (Unix timestamp) when the link will expire or has been expired
    #[must_use]
    pub fn expire_date_option<T: Into<i64>>(mut self, val: Option<T>) -> Self {
        self.expire_date = val.map(Into::into);
        self
    }

    /// The maximum number of users that can be members of the chat simultaneously after joining the chat via this invite link; 1-99999
    #[must_use]
    pub fn member_limit<T: Into<u32>>(mut self, val: T) -> Self {
        self.member_limit = Some(val.into());
        self
    }

    /// The maximum number of users that can be members of the chat simultaneously after joining the chat via this invite link; 1-99999
    #[must_use]
    pub fn member_limit_option<T: Into<u32>>(mut self, val: Option<T>) -> Self {
        self.member_limit = val.map(Into::into);
        self
    }

    /// Number of pending join requests created using this link
    #[must_use]
    pub fn pending_join_request_count<T: Into<i64>>(mut self, val: T) -> Self {
        self.pending_join_request_count = Some(val.into());
        self
    }

    /// Number of pending join requests created using this link
    #[must_use]
    pub fn pending_join_request_count_option<T: Into<i64>>(mut self, val: Option<T>) -> Self {
        self.pending_join_request_count = val.map(Into::into);
        self
    }

    /// The number of seconds the subscription will be active for before the next payment
    #[must_use]
    pub fn subscription_period<T: Into<i64>>(mut self, val: T) -> Self {
        self.subscription_period = Some(val.into());
        self
    }

    /// The number of seconds the subscription will be active for before the next payment
    #[must_use]
    pub fn subscription_period_option<T: Into<i64>>(mut self, val: Option<T>) -> Self {
        self.subscription_period = val.map(Into::into);
        self
    }

    /// The amount of Telegram Stars a user must pay initially and after each subsequent subscription period to be a member of the chat using the link
    #[must_use]
    pub fn subscription_price<T: Into<i64>>(mut self, val: T) -> Self {
        self.subscription_price = Some(val.into());
        self
    }

    /// The amount of Telegram Stars a user must pay initially and after each subsequent subscription period to be a member of the chat using the link
    #[must_use]
    pub fn subscription_price_option<T: Into<i64>>(mut self, val: Option<T>) -> Self {
        self.subscription_price = val.map(Into::into);
        self
    }
}
