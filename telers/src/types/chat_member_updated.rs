use serde::{Deserialize, Serialize};
/// This object represents changes in the status of a chat member.
/// # Documentation
/// <https://core.telegram.org/bots/api#chatmemberupdated>
#[derive(Clone, Debug, Serialize, Deserialize)]
pub struct ChatMemberUpdated {
    /// Chat the user belongs to
    pub chat: Box<crate::types::Chat>,
    /// Performer of the action, which resulted in the change
    pub from: Box<crate::types::User>,
    /// Date the change was done in Unix time
    pub date: i64,
    /// Previous information about the chat member
    pub old_chat_member: crate::types::ChatMember,
    /// New information about the chat member
    pub new_chat_member: crate::types::ChatMember,
    /// Chat invite link, which was used by the user to join the chat; for joining by invite link events only.
    #[serde(skip_serializing_if = "Option::is_none")]
    pub invite_link: Option<crate::types::ChatInviteLink>,
    /// `true`, if the user joined the chat after sending a direct join request without using an invite link and being approved by an administrator
    #[serde(skip_serializing_if = "Option::is_none")]
    pub via_join_request: Option<bool>,
    /// `true`, if the user joined the chat via a chat folder invite link
    #[serde(skip_serializing_if = "Option::is_none")]
    pub via_chat_folder_invite_link: Option<bool>,
}
impl ChatMemberUpdated {
    /// Creates a new `ChatMemberUpdated`.
    ///
    /// # Arguments
    /// * `chat` - Chat the user belongs to
    /// * `from` - Performer of the action, which resulted in the change
    /// * `date` - Date the change was done in Unix time
    /// * `old_chat_member` - Previous information about the chat member
    /// * `new_chat_member` - New information about the chat member
    ///
    /// # Notes
    /// Use builder methods to set optional fields.
    #[must_use]
    pub fn new<
        T0: Into<crate::types::Chat>,
        T1: Into<crate::types::User>,
        T2: Into<i64>,
        T3: Into<crate::types::ChatMember>,
        T4: Into<crate::types::ChatMember>,
    >(
        chat: T0,
        from: T1,
        date: T2,
        old_chat_member: T3,
        new_chat_member: T4,
    ) -> Self {
        Self {
            chat: Box::new(chat.into()),
            from: Box::new(from.into()),
            date: date.into(),
            old_chat_member: old_chat_member.into(),
            new_chat_member: new_chat_member.into(),
            invite_link: None,
            via_join_request: None,
            via_chat_folder_invite_link: None,
        }
    }

    /// Chat the user belongs to
    #[must_use]
    pub fn chat<T: Into<crate::types::Chat>>(self, val: T) -> Self {
        let mut this = self;
        this.chat = Box::new(val.into());
        this
    }

    /// Performer of the action, which resulted in the change
    #[must_use]
    pub fn from<T: Into<crate::types::User>>(self, val: T) -> Self {
        let mut this = self;
        this.from = Box::new(val.into());
        this
    }

    /// Date the change was done in Unix time
    #[must_use]
    pub fn date<T: Into<i64>>(self, val: T) -> Self {
        let mut this = self;
        this.date = val.into();
        this
    }

    /// Previous information about the chat member
    #[must_use]
    pub fn old_chat_member<T: Into<crate::types::ChatMember>>(self, val: T) -> Self {
        let mut this = self;
        this.old_chat_member = val.into();
        this
    }

    /// New information about the chat member
    #[must_use]
    pub fn new_chat_member<T: Into<crate::types::ChatMember>>(self, val: T) -> Self {
        let mut this = self;
        this.new_chat_member = val.into();
        this
    }

    /// Chat invite link, which was used by the user to join the chat; for joining by invite link events only.
    #[must_use]
    pub fn invite_link<T: Into<crate::types::ChatInviteLink>>(self, val: T) -> Self {
        let mut this = self;
        this.invite_link = Some(val.into());
        this
    }

    /// Chat invite link, which was used by the user to join the chat; for joining by invite link events only.
    #[must_use]
    pub fn invite_link_option<T: Into<crate::types::ChatInviteLink>>(self, val: Option<T>) -> Self {
        let mut this = self;
        this.invite_link = val.map(Into::into);
        this
    }

    /// `true`, if the user joined the chat after sending a direct join request without using an invite link and being approved by an administrator
    #[must_use]
    pub fn via_join_request<T: Into<bool>>(self, val: T) -> Self {
        let mut this = self;
        this.via_join_request = Some(val.into());
        this
    }

    /// `true`, if the user joined the chat after sending a direct join request without using an invite link and being approved by an administrator
    #[must_use]
    pub fn via_join_request_option<T: Into<bool>>(self, val: Option<T>) -> Self {
        let mut this = self;
        this.via_join_request = val.map(Into::into);
        this
    }

    /// `true`, if the user joined the chat via a chat folder invite link
    #[must_use]
    pub fn via_chat_folder_invite_link<T: Into<bool>>(self, val: T) -> Self {
        let mut this = self;
        this.via_chat_folder_invite_link = Some(val.into());
        this
    }

    /// `true`, if the user joined the chat via a chat folder invite link
    #[must_use]
    pub fn via_chat_folder_invite_link_option<T: Into<bool>>(self, val: Option<T>) -> Self {
        let mut this = self;
        this.via_chat_folder_invite_link = val.map(Into::into);
        this
    }
}
