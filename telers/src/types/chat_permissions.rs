use serde::{Deserialize, Serialize};
/// Describes actions that a non-administrator user is allowed to take in a chat.
/// # Documentation
/// <https://core.telegram.org/bots/api#chatpermissions>
#[derive(Clone, Debug, Serialize, Deserialize)]
pub struct ChatPermissions {
    /// `true`, if the user is allowed to send text messages, contacts, giveaways, giveaway winners, invoices, locations and venues
    #[serde(skip_serializing_if = "Option::is_none")]
    pub can_send_messages: Option<bool>,
    /// `true`, if the user is allowed to send audios
    #[serde(skip_serializing_if = "Option::is_none")]
    pub can_send_audios: Option<bool>,
    /// `true`, if the user is allowed to send documents
    #[serde(skip_serializing_if = "Option::is_none")]
    pub can_send_documents: Option<bool>,
    /// `true`, if the user is allowed to send photos
    #[serde(skip_serializing_if = "Option::is_none")]
    pub can_send_photos: Option<bool>,
    /// `true`, if the user is allowed to send videos
    #[serde(skip_serializing_if = "Option::is_none")]
    pub can_send_videos: Option<bool>,
    /// `true`, if the user is allowed to send video notes
    #[serde(skip_serializing_if = "Option::is_none")]
    pub can_send_video_notes: Option<bool>,
    /// `true`, if the user is allowed to send voice notes
    #[serde(skip_serializing_if = "Option::is_none")]
    pub can_send_voice_notes: Option<bool>,
    /// `true`, if the user is allowed to send polls and checklists
    #[serde(skip_serializing_if = "Option::is_none")]
    pub can_send_polls: Option<bool>,
    /// `true`, if the user is allowed to send animations, games, stickers and use inline bots
    #[serde(skip_serializing_if = "Option::is_none")]
    pub can_send_other_messages: Option<bool>,
    /// `true`, if the user is allowed to add web page previews to their messages
    #[serde(skip_serializing_if = "Option::is_none")]
    pub can_add_web_page_previews: Option<bool>,
    /// `true`, if the user is allowed to edit their own tag
    #[serde(skip_serializing_if = "Option::is_none")]
    pub can_edit_tag: Option<bool>,
    /// `true`, if the user is allowed to change the chat title, photo and other settings. Ignored in public supergroups
    #[serde(skip_serializing_if = "Option::is_none")]
    pub can_change_info: Option<bool>,
    /// `true`, if the user is allowed to invite new users to the chat
    #[serde(skip_serializing_if = "Option::is_none")]
    pub can_invite_users: Option<bool>,
    /// `true`, if the user is allowed to pin messages. Ignored in public supergroups
    #[serde(skip_serializing_if = "Option::is_none")]
    pub can_pin_messages: Option<bool>,
    /// `true`, if the user is allowed to create forum topics. If omitted defaults to the value of `can_pin_messages`
    #[serde(skip_serializing_if = "Option::is_none")]
    pub can_manage_topics: Option<bool>,
}
impl ChatPermissions {
    /// Creates a new `ChatPermissions`.
    ///
    /// # Notes
    /// Use builder methods to set optional fields.
    #[must_use]
    pub fn new() -> Self {
        Self {
            can_send_messages: None,
            can_send_audios: None,
            can_send_documents: None,
            can_send_photos: None,
            can_send_videos: None,
            can_send_video_notes: None,
            can_send_voice_notes: None,
            can_send_polls: None,
            can_send_other_messages: None,
            can_add_web_page_previews: None,
            can_edit_tag: None,
            can_change_info: None,
            can_invite_users: None,
            can_pin_messages: None,
            can_manage_topics: None,
        }
    }

    /// `true`, if the user is allowed to send text messages, contacts, giveaways, giveaway winners, invoices, locations and venues
    #[must_use]
    pub fn can_send_messages<T: Into<bool>>(self, val: T) -> Self {
        let mut this = self;
        this.can_send_messages = Some(val.into());
        this
    }

    /// `true`, if the user is allowed to send text messages, contacts, giveaways, giveaway winners, invoices, locations and venues
    #[must_use]
    pub fn can_send_messages_option<T: Into<bool>>(self, val: Option<T>) -> Self {
        let mut this = self;
        this.can_send_messages = val.map(Into::into);
        this
    }

    /// `true`, if the user is allowed to send audios
    #[must_use]
    pub fn can_send_audios<T: Into<bool>>(self, val: T) -> Self {
        let mut this = self;
        this.can_send_audios = Some(val.into());
        this
    }

    /// `true`, if the user is allowed to send audios
    #[must_use]
    pub fn can_send_audios_option<T: Into<bool>>(self, val: Option<T>) -> Self {
        let mut this = self;
        this.can_send_audios = val.map(Into::into);
        this
    }

    /// `true`, if the user is allowed to send documents
    #[must_use]
    pub fn can_send_documents<T: Into<bool>>(self, val: T) -> Self {
        let mut this = self;
        this.can_send_documents = Some(val.into());
        this
    }

    /// `true`, if the user is allowed to send documents
    #[must_use]
    pub fn can_send_documents_option<T: Into<bool>>(self, val: Option<T>) -> Self {
        let mut this = self;
        this.can_send_documents = val.map(Into::into);
        this
    }

    /// `true`, if the user is allowed to send photos
    #[must_use]
    pub fn can_send_photos<T: Into<bool>>(self, val: T) -> Self {
        let mut this = self;
        this.can_send_photos = Some(val.into());
        this
    }

    /// `true`, if the user is allowed to send photos
    #[must_use]
    pub fn can_send_photos_option<T: Into<bool>>(self, val: Option<T>) -> Self {
        let mut this = self;
        this.can_send_photos = val.map(Into::into);
        this
    }

    /// `true`, if the user is allowed to send videos
    #[must_use]
    pub fn can_send_videos<T: Into<bool>>(self, val: T) -> Self {
        let mut this = self;
        this.can_send_videos = Some(val.into());
        this
    }

    /// `true`, if the user is allowed to send videos
    #[must_use]
    pub fn can_send_videos_option<T: Into<bool>>(self, val: Option<T>) -> Self {
        let mut this = self;
        this.can_send_videos = val.map(Into::into);
        this
    }

    /// `true`, if the user is allowed to send video notes
    #[must_use]
    pub fn can_send_video_notes<T: Into<bool>>(self, val: T) -> Self {
        let mut this = self;
        this.can_send_video_notes = Some(val.into());
        this
    }

    /// `true`, if the user is allowed to send video notes
    #[must_use]
    pub fn can_send_video_notes_option<T: Into<bool>>(self, val: Option<T>) -> Self {
        let mut this = self;
        this.can_send_video_notes = val.map(Into::into);
        this
    }

    /// `true`, if the user is allowed to send voice notes
    #[must_use]
    pub fn can_send_voice_notes<T: Into<bool>>(self, val: T) -> Self {
        let mut this = self;
        this.can_send_voice_notes = Some(val.into());
        this
    }

    /// `true`, if the user is allowed to send voice notes
    #[must_use]
    pub fn can_send_voice_notes_option<T: Into<bool>>(self, val: Option<T>) -> Self {
        let mut this = self;
        this.can_send_voice_notes = val.map(Into::into);
        this
    }

    /// `true`, if the user is allowed to send polls and checklists
    #[must_use]
    pub fn can_send_polls<T: Into<bool>>(self, val: T) -> Self {
        let mut this = self;
        this.can_send_polls = Some(val.into());
        this
    }

    /// `true`, if the user is allowed to send polls and checklists
    #[must_use]
    pub fn can_send_polls_option<T: Into<bool>>(self, val: Option<T>) -> Self {
        let mut this = self;
        this.can_send_polls = val.map(Into::into);
        this
    }

    /// `true`, if the user is allowed to send animations, games, stickers and use inline bots
    #[must_use]
    pub fn can_send_other_messages<T: Into<bool>>(self, val: T) -> Self {
        let mut this = self;
        this.can_send_other_messages = Some(val.into());
        this
    }

    /// `true`, if the user is allowed to send animations, games, stickers and use inline bots
    #[must_use]
    pub fn can_send_other_messages_option<T: Into<bool>>(self, val: Option<T>) -> Self {
        let mut this = self;
        this.can_send_other_messages = val.map(Into::into);
        this
    }

    /// `true`, if the user is allowed to add web page previews to their messages
    #[must_use]
    pub fn can_add_web_page_previews<T: Into<bool>>(self, val: T) -> Self {
        let mut this = self;
        this.can_add_web_page_previews = Some(val.into());
        this
    }

    /// `true`, if the user is allowed to add web page previews to their messages
    #[must_use]
    pub fn can_add_web_page_previews_option<T: Into<bool>>(self, val: Option<T>) -> Self {
        let mut this = self;
        this.can_add_web_page_previews = val.map(Into::into);
        this
    }

    /// `true`, if the user is allowed to edit their own tag
    #[must_use]
    pub fn can_edit_tag<T: Into<bool>>(self, val: T) -> Self {
        let mut this = self;
        this.can_edit_tag = Some(val.into());
        this
    }

    /// `true`, if the user is allowed to edit their own tag
    #[must_use]
    pub fn can_edit_tag_option<T: Into<bool>>(self, val: Option<T>) -> Self {
        let mut this = self;
        this.can_edit_tag = val.map(Into::into);
        this
    }

    /// `true`, if the user is allowed to change the chat title, photo and other settings. Ignored in public supergroups
    #[must_use]
    pub fn can_change_info<T: Into<bool>>(self, val: T) -> Self {
        let mut this = self;
        this.can_change_info = Some(val.into());
        this
    }

    /// `true`, if the user is allowed to change the chat title, photo and other settings. Ignored in public supergroups
    #[must_use]
    pub fn can_change_info_option<T: Into<bool>>(self, val: Option<T>) -> Self {
        let mut this = self;
        this.can_change_info = val.map(Into::into);
        this
    }

    /// `true`, if the user is allowed to invite new users to the chat
    #[must_use]
    pub fn can_invite_users<T: Into<bool>>(self, val: T) -> Self {
        let mut this = self;
        this.can_invite_users = Some(val.into());
        this
    }

    /// `true`, if the user is allowed to invite new users to the chat
    #[must_use]
    pub fn can_invite_users_option<T: Into<bool>>(self, val: Option<T>) -> Self {
        let mut this = self;
        this.can_invite_users = val.map(Into::into);
        this
    }

    /// `true`, if the user is allowed to pin messages. Ignored in public supergroups
    #[must_use]
    pub fn can_pin_messages<T: Into<bool>>(self, val: T) -> Self {
        let mut this = self;
        this.can_pin_messages = Some(val.into());
        this
    }

    /// `true`, if the user is allowed to pin messages. Ignored in public supergroups
    #[must_use]
    pub fn can_pin_messages_option<T: Into<bool>>(self, val: Option<T>) -> Self {
        let mut this = self;
        this.can_pin_messages = val.map(Into::into);
        this
    }

    /// `true`, if the user is allowed to create forum topics. If omitted defaults to the value of `can_pin_messages`
    #[must_use]
    pub fn can_manage_topics<T: Into<bool>>(self, val: T) -> Self {
        let mut this = self;
        this.can_manage_topics = Some(val.into());
        this
    }

    /// `true`, if the user is allowed to create forum topics. If omitted defaults to the value of `can_pin_messages`
    #[must_use]
    pub fn can_manage_topics_option<T: Into<bool>>(self, val: Option<T>) -> Self {
        let mut this = self;
        this.can_manage_topics = val.map(Into::into);
        this
    }
}
impl Default for ChatPermissions {
    fn default() -> Self {
        Self::new()
    }
}
