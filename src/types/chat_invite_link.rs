use super::User;

use serde::{Deserialize, Serialize};

/// Represents an invite link for a chat.
/// <https://core.telegram.org/bots/api#chatinvitelink>
#[derive(Default, Clone, Debug, Eq, Hash, PartialEq, Serialize, Deserialize)]
pub struct ChatInviteLink {
    /// The invite link. If the link was created by another chat administrator, then the second part of the link will be replaced with '…'.
    pub invite_link: String,
    /// Creator of the link
    pub creator: User,
    /// :code:`True`, if users joining the chat via the link need to be approved by chat administrators
    pub creates_join_request: bool,
    /// :code:`True`, if the link is primary
    pub is_primary: bool,
    /// :code:`True`, if the link is revoked
    pub is_revoked: bool,
    /// *Optional*. Invite link name
    pub name: Option<String>,
    /// *Optional*. Point in time (Unix timestamp) when the link will expire or has been expired
    pub expire_date: Option<i64>,
    /// *Optional*. The maximum number of users that can be members of the chat simultaneously after joining the chat via this invite link; 1-99999
    pub member_limit: Option<i64>,
    /// *Optional*. Number of pending join requests created using this link
    pub pending_join_request_count: Option<i64>,
}
