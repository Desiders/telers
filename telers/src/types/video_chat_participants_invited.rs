use serde::{Deserialize, Serialize};
/// This object represents a service message about new members invited to a video chat.
/// # Documentation
/// <https://core.telegram.org/bots/api#videochatparticipantsinvited>
#[derive(Clone, Debug, Serialize, Deserialize)]
pub struct VideoChatParticipantsInvited {
    /// New members that were invited to the video chat
    pub users: Box<[crate::types::User]>,
}
impl VideoChatParticipantsInvited {
    /// Creates a new `VideoChatParticipantsInvited`.
    ///
    /// # Arguments
    /// * `users` - New members that were invited to the video chat
    #[must_use]
    pub fn new<T0Item: Into<crate::types::User>, T0: IntoIterator<Item = T0Item>>(
        users: T0,
    ) -> Self {
        Self {
            users: users.into_iter().map(Into::into).collect(),
        }
    }

    /// New members that were invited to the video chat
    ///
    /// # Notes
    /// Adds multiple elements.
    #[must_use]
    pub fn users<T: Into<Box<[crate::types::User]>>>(self, val: T) -> Self {
        let mut this = self;
        this.users = this
            .users
            .into_vec()
            .into_iter()
            .chain(val.into())
            .collect();
        this
    }

    /// New members that were invited to the video chat
    ///
    /// # Notes
    /// Adds a single element.
    #[must_use]
    pub fn user<T: Into<crate::types::User>>(self, val: T) -> Self {
        let mut this = self;
        this.users = this
            .users
            .into_vec()
            .into_iter()
            .chain(Some(val.into()))
            .collect();
        this
    }
}
