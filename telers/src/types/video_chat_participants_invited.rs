use super::User;

use serde::Deserialize;

/// This object represents a service message about new members invited to a video chat.
/// # Documentation
/// <https://core.telegram.org/bots/api#videochatparticipantsinvited>
#[derive(Debug, Clone, Hash, PartialEq, Eq, Deserialize)]
pub struct VideoChatParticipantsInvited {
    /// New members that were invited to the video chat
    pub users: Box<[User]>,
}
