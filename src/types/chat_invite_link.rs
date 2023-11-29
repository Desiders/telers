use super::User;

use serde::Deserialize;

/// Represents an invite link for a chat.
/// # Documentation
/// <https://core.telegram.org/bots/api#chatinvitelink>
#[derive(Debug, Clone, Hash, PartialEq, Eq, Deserialize)]
pub struct ChatInviteLink {
    /// The invite link. If the link was created by another chat administrator, then the second part of the link will be replaced with '…'.
    pub invite_link: Box<str>,
    /// Creator of the link
    pub creator: User,
    /// `true`, if users joining the chat via the link need to be approved by chat administrators
    pub creates_join_request: bool,
    /// `true`, if the link is primary
    pub is_primary: bool,
    /// `true`, if the link is revoked
    pub is_revoked: bool,
    /// Invite link name
    pub name: Option<Box<str>>,
    /// Point in time (Unix timestamp) when the link will expire or has been expired
    pub expire_date: Option<i64>,
    /// The maximum number of users that can be members of the chat simultaneously after joining the chat via this invite link; 1-99999
    pub member_limit: Option<i64>,
    /// Number of pending join requests created using this link
    pub pending_join_request_count: Option<i64>,
}
