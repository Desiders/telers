use serde::{Deserialize, Serialize};
use serde_with::skip_serializing_none;

/// Describes actions that a non-administrator user is allowed to take in a chat.
/// # Documentation
/// <https://core.telegram.org/bots/api#chatpermissions>
#[skip_serializing_none]
#[derive(Debug, Default, Clone, Hash, PartialEq, Eq, Deserialize, Serialize)]
pub struct ChatPermissions {
    /// `true`, if the user is allowed to send text messages, contacts, locations and venues
    pub can_send_messages: Option<bool>,
    /// `true`, if the user is allowed to send audios
    pub can_send_audios: Option<bool>,
    /// `true`, if the user is allowed to send documents
    pub can_send_documents: Option<bool>,
    /// `true`, if the user is allowed to send photos
    pub can_send_photos: Option<bool>,
    /// `true`, if the user is allowed to send videos
    pub can_send_videos: Option<bool>,
    /// `true`, if the user is allowed to send video notes
    pub can_send_video_notes: Option<bool>,
    /// `true`, if the user is allowed to send voice notes
    pub can_send_voice_notes: Option<bool>,
    /// `true`, if the user is allowed to send polls
    pub can_send_polls: Option<bool>,
    /// `true`, if the user is allowed to send animations, games, stickers and use inline bots
    pub can_send_other_messages: Option<bool>,
    /// `true`, if the user is allowed to add web page previews to their messages, implies `can_send_media_messages`
    pub can_add_web_page_previews: Option<bool>,
    /// `true`, if the user is allowed to change the chat title, photo and other settings. Ignored in public supergroups
    pub can_change_info: Option<bool>,
    /// `true`, if the user is allowed to invite new users to the chat
    pub can_invite_users: Option<bool>,
    /// `true`, if the user is allowed to pin messages. Ignored in public supergroups
    pub can_pin_messages: Option<bool>,
    /// `true`, if the user is allowed to create forum topics. If omitted defaults to the value of `can_pin_messages`
    pub can_manage_topics: Option<bool>,
}

impl ChatPermissions {
    #[must_use]
    pub fn new() -> Self {
        Self::default()
    }

    #[must_use]
    pub const fn all() -> Self {
        Self {
            can_send_messages: Some(true),
            can_send_audios: Some(true),
            can_send_documents: Some(true),
            can_send_photos: Some(true),
            can_send_videos: Some(true),
            can_send_video_notes: Some(true),
            can_send_voice_notes: Some(true),
            can_send_polls: Some(true),
            can_send_other_messages: Some(true),
            can_add_web_page_previews: Some(true),
            can_change_info: Some(true),
            can_invite_users: Some(true),
            can_pin_messages: Some(true),
            can_manage_topics: Some(true),
        }
    }

    #[must_use]
    pub fn can_send_messages(self, val: bool) -> Self {
        Self {
            can_send_messages: Some(val),
            ..self
        }
    }

    #[must_use]
    pub fn can_send_audios(self, val: bool) -> Self {
        Self {
            can_send_audios: Some(val),
            ..self
        }
    }

    #[must_use]
    pub fn can_send_documents(self, val: bool) -> Self {
        Self {
            can_send_documents: Some(val),
            ..self
        }
    }

    #[must_use]
    pub fn can_send_photos(self, val: bool) -> Self {
        Self {
            can_send_photos: Some(val),
            ..self
        }
    }

    #[must_use]
    pub fn can_send_videos(self, val: bool) -> Self {
        Self {
            can_send_videos: Some(val),
            ..self
        }
    }

    #[must_use]
    pub fn can_send_video_notes(self, val: bool) -> Self {
        Self {
            can_send_video_notes: Some(val),
            ..self
        }
    }

    #[must_use]
    pub fn can_send_voice_notes(self, val: bool) -> Self {
        Self {
            can_send_voice_notes: Some(val),
            ..self
        }
    }

    #[must_use]
    pub fn can_send_polls(self, val: bool) -> Self {
        Self {
            can_send_polls: Some(val),
            ..self
        }
    }

    #[must_use]
    pub fn can_send_other_messages(self, val: bool) -> Self {
        Self {
            can_send_other_messages: Some(val),
            ..self
        }
    }

    #[must_use]
    pub fn can_add_web_page_previews(self, val: bool) -> Self {
        Self {
            can_add_web_page_previews: Some(val),
            ..self
        }
    }

    #[must_use]
    pub fn can_change_info(self, val: bool) -> Self {
        Self {
            can_change_info: Some(val),
            ..self
        }
    }

    #[must_use]
    pub fn can_invite_users(self, val: bool) -> Self {
        Self {
            can_invite_users: Some(val),
            ..self
        }
    }

    #[must_use]
    pub fn can_pin_messages(self, val: bool) -> Self {
        Self {
            can_pin_messages: Some(val),
            ..self
        }
    }

    #[must_use]
    pub fn can_manage_topics(self, val: bool) -> Self {
        Self {
            can_manage_topics: Some(val),
            ..self
        }
    }
}

impl ChatPermissions {
    #[must_use]
    pub fn can_send_message_option(self, val: Option<bool>) -> Self {
        Self {
            can_send_messages: val,
            ..self
        }
    }

    #[must_use]
    pub fn can_send_audios_option(self, val: Option<bool>) -> Self {
        Self {
            can_send_audios: val,
            ..self
        }
    }

    #[must_use]
    pub fn can_send_documents_option(self, val: Option<bool>) -> Self {
        Self {
            can_send_documents: val,
            ..self
        }
    }

    #[must_use]
    pub fn can_send_photos_option(self, val: Option<bool>) -> Self {
        Self {
            can_send_photos: val,
            ..self
        }
    }

    #[must_use]
    pub fn can_send_videos_option(self, val: Option<bool>) -> Self {
        Self {
            can_send_videos: val,
            ..self
        }
    }

    #[must_use]
    pub fn can_send_video_notes_option(self, val: Option<bool>) -> Self {
        Self {
            can_send_video_notes: val,
            ..self
        }
    }

    #[must_use]
    pub fn can_send_voice_notes_option(self, val: Option<bool>) -> Self {
        Self {
            can_send_voice_notes: val,
            ..self
        }
    }

    #[must_use]
    pub fn can_send_polls_option(self, val: Option<bool>) -> Self {
        Self {
            can_send_polls: val,
            ..self
        }
    }

    #[must_use]
    pub fn can_send_other_messages_option(self, val: Option<bool>) -> Self {
        Self {
            can_send_other_messages: val,
            ..self
        }
    }

    #[must_use]
    pub fn can_add_web_page_previews_option(self, val: Option<bool>) -> Self {
        Self {
            can_add_web_page_previews: val,
            ..self
        }
    }

    #[must_use]
    pub fn can_change_info_option(self, val: Option<bool>) -> Self {
        Self {
            can_change_info: val,
            ..self
        }
    }

    #[must_use]
    pub fn can_invite_users_option(self, val: Option<bool>) -> Self {
        Self {
            can_invite_users: val,
            ..self
        }
    }

    #[must_use]
    pub fn can_pin_messages_option(self, val: Option<bool>) -> Self {
        Self {
            can_pin_messages: val,
            ..self
        }
    }

    #[must_use]
    pub fn can_manage_topics_option(self, val: Option<bool>) -> Self {
        Self {
            can_manage_topics: val,
            ..self
        }
    }
}
