use serde::{Deserialize, Serialize};
/// Represents a join request sent to a chat.
/// # Documentation
/// <https://core.telegram.org/bots/api#chatjoinrequest>
#[derive(Clone, Debug, Serialize, Deserialize)]
pub struct ChatJoinRequest {
    /// Chat to which the request was sent
    pub chat: Box<crate::types::Chat>,
    /// User that sent the join request
    pub from: Box<crate::types::User>,
    /// Identifier of a private chat with the user who sent the join request. This number may have more than 32 significant bits and some programming languages may have difficulty/silent defects in interpreting it. But it has at most 52 significant bits, so a 64-bit integer or double-precision float type are safe for storing this identifier. The bot can use this identifier for 5 minutes to send messages until the join request is processed, assuming no other administrator contacted the user.
    pub user_chat_id: i64,
    /// Date the request was sent in Unix time
    pub date: i64,
    /// Bio of the user.
    #[serde(skip_serializing_if = "Option::is_none")]
    pub bio: Option<Box<str>>,
    /// Chat invite link that was used by the user to send the join request
    #[serde(skip_serializing_if = "Option::is_none")]
    pub invite_link: Option<crate::types::ChatInviteLink>,
}
impl ChatJoinRequest {
    /// Creates a new `ChatJoinRequest`.
    ///
    /// # Arguments
    /// * `chat` - Chat to which the request was sent
    /// * `from` - User that sent the join request
    /// * `user_chat_id` - Identifier of a private chat with the user who sent the join request. This number may have more than 32 significant bits and some programming languages may have difficulty/silent defects in interpreting it. But it has at most 52 significant bits, so a 64-bit integer or double-precision float type are safe for storing this identifier. The bot can use this identifier for 5 minutes to send messages until the join request is processed, assuming no other administrator contacted the user.
    /// * `date` - Date the request was sent in Unix time
    ///
    /// # Notes
    /// Use builder methods to set optional fields.
    #[must_use]
    pub fn new<
        T0: Into<crate::types::Chat>,
        T1: Into<crate::types::User>,
        T2: Into<i64>,
        T3: Into<i64>,
    >(
        chat: T0,
        from: T1,
        user_chat_id: T2,
        date: T3,
    ) -> Self {
        Self {
            chat: Box::new(chat.into()),
            from: Box::new(from.into()),
            user_chat_id: user_chat_id.into(),
            date: date.into(),
            bio: None,
            invite_link: None,
        }
    }

    /// Chat to which the request was sent
    #[must_use]
    pub fn chat<T: Into<crate::types::Chat>>(self, val: T) -> Self {
        let mut this = self;
        this.chat = Box::new(val.into());
        this
    }

    /// User that sent the join request
    #[must_use]
    pub fn from<T: Into<crate::types::User>>(self, val: T) -> Self {
        let mut this = self;
        this.from = Box::new(val.into());
        this
    }

    /// Identifier of a private chat with the user who sent the join request. This number may have more than 32 significant bits and some programming languages may have difficulty/silent defects in interpreting it. But it has at most 52 significant bits, so a 64-bit integer or double-precision float type are safe for storing this identifier. The bot can use this identifier for 5 minutes to send messages until the join request is processed, assuming no other administrator contacted the user.
    #[must_use]
    pub fn user_chat_id<T: Into<i64>>(self, val: T) -> Self {
        let mut this = self;
        this.user_chat_id = val.into();
        this
    }

    /// Date the request was sent in Unix time
    #[must_use]
    pub fn date<T: Into<i64>>(self, val: T) -> Self {
        let mut this = self;
        this.date = val.into();
        this
    }

    /// Bio of the user.
    #[must_use]
    pub fn bio<T: Into<Box<str>>>(self, val: T) -> Self {
        let mut this = self;
        this.bio = Some(val.into());
        this
    }

    /// Bio of the user.
    #[must_use]
    pub fn bio_option<T: Into<Box<str>>>(self, val: Option<T>) -> Self {
        let mut this = self;
        this.bio = val.map(Into::into);
        this
    }

    /// Chat invite link that was used by the user to send the join request
    #[must_use]
    pub fn invite_link<T: Into<crate::types::ChatInviteLink>>(self, val: T) -> Self {
        let mut this = self;
        this.invite_link = Some(val.into());
        this
    }

    /// Chat invite link that was used by the user to send the join request
    #[must_use]
    pub fn invite_link_option<T: Into<crate::types::ChatInviteLink>>(self, val: Option<T>) -> Self {
        let mut this = self;
        this.invite_link = val.map(Into::into);
        this
    }
}
