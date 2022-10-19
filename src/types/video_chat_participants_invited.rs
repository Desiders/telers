use super::User;

use serde::{Deserialize, Serialize};

/// This object represents a service message about new members invited to a video chat.
/// <https://core.telegram.org/bots/api#videochatparticipantsinvited>_
#[derive(Default, Clone, Debug, Eq, Hash, PartialEq, Serialize, Deserialize)]
pub struct VideoChatParticipantsInvited {
    /// New members that were invited to the video chat
    pub users: Vec<User>,
}
