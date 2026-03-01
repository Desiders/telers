use serde::{Deserialize, Serialize};
/// Represents a chat member that is under certain restrictions in the chat. Supergroups only.
/// # Documentation
/// <https://core.telegram.org/bots/api#chatmemberrestricted>
#[derive(Clone, Debug, Serialize, Deserialize)]
pub struct ChatMemberRestricted {
    /// Tag of the member
    #[serde(skip_serializing_if = "Option::is_none")]
    pub tag: Option<Box<str>>,
    /// Information about the user
    pub user: Box<crate::types::User>,
    /// `true`, if the user is a member of the chat at the moment of the request
    pub is_member: bool,
    /// `true`, if the user is allowed to send text messages, contacts, giveaways, giveaway winners, invoices, locations and venues
    pub can_send_messages: bool,
    /// `true`, if the user is allowed to send audios
    pub can_send_audios: bool,
    /// `true`, if the user is allowed to send documents
    pub can_send_documents: bool,
    /// `true`, if the user is allowed to send photos
    pub can_send_photos: bool,
    /// `true`, if the user is allowed to send videos
    pub can_send_videos: bool,
    /// `true`, if the user is allowed to send video notes
    pub can_send_video_notes: bool,
    /// `true`, if the user is allowed to send voice notes
    pub can_send_voice_notes: bool,
    /// `true`, if the user is allowed to send polls and checklists
    pub can_send_polls: bool,
    /// `true`, if the user is allowed to send animations, games, stickers and use inline bots
    pub can_send_other_messages: bool,
    /// `true`, if the user is allowed to add web page previews to their messages
    pub can_add_web_page_previews: bool,
    /// `true`, if the user is allowed to edit their own tag
    pub can_edit_tag: bool,
    /// `true`, if the user is allowed to change the chat title, photo and other settings
    pub can_change_info: bool,
    /// `true`, if the user is allowed to invite new users to the chat
    pub can_invite_users: bool,
    /// `true`, if the user is allowed to pin messages
    pub can_pin_messages: bool,
    /// `true`, if the user is allowed to create forum topics
    pub can_manage_topics: bool,
    /// Date when restrictions will be lifted for this user; Unix time. If 0, then the user is restricted forever
    pub until_date: i64,
}
impl ChatMemberRestricted {
    /// Creates a new `ChatMemberRestricted`.
    ///
    /// # Arguments
    /// * `user` - Information about the user
    /// * `is_member` - `true`, if the user is a member of the chat at the moment of the request
    /// * `can_send_messages` - `true`, if the user is allowed to send text messages, contacts, giveaways, giveaway winners, invoices, locations and venues
    /// * `can_send_audios` - `true`, if the user is allowed to send audios
    /// * `can_send_documents` - `true`, if the user is allowed to send documents
    /// * `can_send_photos` - `true`, if the user is allowed to send photos
    /// * `can_send_videos` - `true`, if the user is allowed to send videos
    /// * `can_send_video_notes` - `true`, if the user is allowed to send video notes
    /// * `can_send_voice_notes` - `true`, if the user is allowed to send voice notes
    /// * `can_send_polls` - `true`, if the user is allowed to send polls and checklists
    /// * `can_send_other_messages` - `true`, if the user is allowed to send animations, games, stickers and use inline bots
    /// * `can_add_web_page_previews` - `true`, if the user is allowed to add web page previews to their messages
    /// * `can_edit_tag` - `true`, if the user is allowed to edit their own tag
    /// * `can_change_info` - `true`, if the user is allowed to change the chat title, photo and other settings
    /// * `can_invite_users` - `true`, if the user is allowed to invite new users to the chat
    /// * `can_pin_messages` - `true`, if the user is allowed to pin messages
    /// * `can_manage_topics` - `true`, if the user is allowed to create forum topics
    /// * `until_date` - Date when restrictions will be lifted for this user; Unix time. If 0, then the user is restricted forever
    ///
    /// # Notes
    /// Use builder methods to set optional fields.
    #[must_use]
    pub fn new<
        T0: Into<crate::types::User>,
        T1: Into<bool>,
        T2: Into<bool>,
        T3: Into<bool>,
        T4: Into<bool>,
        T5: Into<bool>,
        T6: Into<bool>,
        T7: Into<bool>,
        T8: Into<bool>,
        T9: Into<bool>,
        T10: Into<bool>,
        T11: Into<bool>,
        T12: Into<bool>,
        T13: Into<bool>,
        T14: Into<bool>,
        T15: Into<bool>,
        T16: Into<bool>,
        T17: Into<i64>,
    >(
        user: T0,
        is_member: T1,
        can_send_messages: T2,
        can_send_audios: T3,
        can_send_documents: T4,
        can_send_photos: T5,
        can_send_videos: T6,
        can_send_video_notes: T7,
        can_send_voice_notes: T8,
        can_send_polls: T9,
        can_send_other_messages: T10,
        can_add_web_page_previews: T11,
        can_edit_tag: T12,
        can_change_info: T13,
        can_invite_users: T14,
        can_pin_messages: T15,
        can_manage_topics: T16,
        until_date: T17,
    ) -> Self {
        Self {
            tag: None,
            user: Box::new(user.into()),
            is_member: is_member.into(),
            can_send_messages: can_send_messages.into(),
            can_send_audios: can_send_audios.into(),
            can_send_documents: can_send_documents.into(),
            can_send_photos: can_send_photos.into(),
            can_send_videos: can_send_videos.into(),
            can_send_video_notes: can_send_video_notes.into(),
            can_send_voice_notes: can_send_voice_notes.into(),
            can_send_polls: can_send_polls.into(),
            can_send_other_messages: can_send_other_messages.into(),
            can_add_web_page_previews: can_add_web_page_previews.into(),
            can_edit_tag: can_edit_tag.into(),
            can_change_info: can_change_info.into(),
            can_invite_users: can_invite_users.into(),
            can_pin_messages: can_pin_messages.into(),
            can_manage_topics: can_manage_topics.into(),
            until_date: until_date.into(),
        }
    }

    /// Tag of the member
    #[must_use]
    pub fn tag<T: Into<Box<str>>>(self, val: T) -> Self {
        let mut this = self;
        this.tag = Some(val.into());
        this
    }

    /// Tag of the member
    #[must_use]
    pub fn tag_option<T: Into<Box<str>>>(self, val: Option<T>) -> Self {
        let mut this = self;
        this.tag = val.map(Into::into);
        this
    }

    /// Information about the user
    #[must_use]
    pub fn user<T: Into<crate::types::User>>(self, val: T) -> Self {
        let mut this = self;
        this.user = Box::new(val.into());
        this
    }

    /// `true`, if the user is a member of the chat at the moment of the request
    #[must_use]
    pub fn is_member<T: Into<bool>>(self, val: T) -> Self {
        let mut this = self;
        this.is_member = val.into();
        this
    }

    /// `true`, if the user is allowed to send text messages, contacts, giveaways, giveaway winners, invoices, locations and venues
    #[must_use]
    pub fn can_send_messages<T: Into<bool>>(self, val: T) -> Self {
        let mut this = self;
        this.can_send_messages = val.into();
        this
    }

    /// `true`, if the user is allowed to send audios
    #[must_use]
    pub fn can_send_audios<T: Into<bool>>(self, val: T) -> Self {
        let mut this = self;
        this.can_send_audios = val.into();
        this
    }

    /// `true`, if the user is allowed to send documents
    #[must_use]
    pub fn can_send_documents<T: Into<bool>>(self, val: T) -> Self {
        let mut this = self;
        this.can_send_documents = val.into();
        this
    }

    /// `true`, if the user is allowed to send photos
    #[must_use]
    pub fn can_send_photos<T: Into<bool>>(self, val: T) -> Self {
        let mut this = self;
        this.can_send_photos = val.into();
        this
    }

    /// `true`, if the user is allowed to send videos
    #[must_use]
    pub fn can_send_videos<T: Into<bool>>(self, val: T) -> Self {
        let mut this = self;
        this.can_send_videos = val.into();
        this
    }

    /// `true`, if the user is allowed to send video notes
    #[must_use]
    pub fn can_send_video_notes<T: Into<bool>>(self, val: T) -> Self {
        let mut this = self;
        this.can_send_video_notes = val.into();
        this
    }

    /// `true`, if the user is allowed to send voice notes
    #[must_use]
    pub fn can_send_voice_notes<T: Into<bool>>(self, val: T) -> Self {
        let mut this = self;
        this.can_send_voice_notes = val.into();
        this
    }

    /// `true`, if the user is allowed to send polls and checklists
    #[must_use]
    pub fn can_send_polls<T: Into<bool>>(self, val: T) -> Self {
        let mut this = self;
        this.can_send_polls = val.into();
        this
    }

    /// `true`, if the user is allowed to send animations, games, stickers and use inline bots
    #[must_use]
    pub fn can_send_other_messages<T: Into<bool>>(self, val: T) -> Self {
        let mut this = self;
        this.can_send_other_messages = val.into();
        this
    }

    /// `true`, if the user is allowed to add web page previews to their messages
    #[must_use]
    pub fn can_add_web_page_previews<T: Into<bool>>(self, val: T) -> Self {
        let mut this = self;
        this.can_add_web_page_previews = val.into();
        this
    }

    /// `true`, if the user is allowed to edit their own tag
    #[must_use]
    pub fn can_edit_tag<T: Into<bool>>(self, val: T) -> Self {
        let mut this = self;
        this.can_edit_tag = val.into();
        this
    }

    /// `true`, if the user is allowed to change the chat title, photo and other settings
    #[must_use]
    pub fn can_change_info<T: Into<bool>>(self, val: T) -> Self {
        let mut this = self;
        this.can_change_info = val.into();
        this
    }

    /// `true`, if the user is allowed to invite new users to the chat
    #[must_use]
    pub fn can_invite_users<T: Into<bool>>(self, val: T) -> Self {
        let mut this = self;
        this.can_invite_users = val.into();
        this
    }

    /// `true`, if the user is allowed to pin messages
    #[must_use]
    pub fn can_pin_messages<T: Into<bool>>(self, val: T) -> Self {
        let mut this = self;
        this.can_pin_messages = val.into();
        this
    }

    /// `true`, if the user is allowed to create forum topics
    #[must_use]
    pub fn can_manage_topics<T: Into<bool>>(self, val: T) -> Self {
        let mut this = self;
        this.can_manage_topics = val.into();
        this
    }

    /// Date when restrictions will be lifted for this user; Unix time. If 0, then the user is restricted forever
    #[must_use]
    pub fn until_date<T: Into<i64>>(self, val: T) -> Self {
        let mut this = self;
        this.until_date = val.into();
        this
    }
}
