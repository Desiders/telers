use super::{ChatLocation, ChatPermissions, ChatPhoto, Message};

use serde::Deserialize;

/// This object represents a chat.
/// # Documentation
/// <https://core.telegram.org/bots/api#chat>
#[derive(Debug, Clone, PartialEq, Deserialize)]
#[serde(tag = "type", rename_all = "snake_case")]
pub enum Chat {
    Private(Private),
    Group(Group),
    Supergroup(Supergroup),
    Channel(Channel),
}

#[derive(Debug, Default, Clone, PartialEq, Deserialize)]
pub struct Private {
    /// Unique identifier for this chat. This number may have more than 32 significant bits and some programming languages may have difficulty/silent defects in i64erpreting it. But it has at most 52 significant bits, so a signed 64-bit i64eger or double-precision float type are safe for storing this identifier.
    pub id: i64,
    /// Username
    pub username: Option<Box<str>>,
    /// First name of the other party
    pub first_name: Option<Box<str>>,
    /// Last name of the other party
    pub last_name: Option<Box<str>>,
    /// Chat photo. Returned only in [`GetChat`](crate::methods::GetChat).
    pub photo: Option<ChatPhoto>,
    /// If non-empty, the list of all [active chat usernames](https://telegram.org/blog/topics-in-groups-collectible-usernames/ru?ln=a#collectible-usernames). Returned only in [`GetChat`](crate::methods::GetChat).
    pub active_usernames: Option<Box<[Box<str>]>>,
    /// Identifier of the accent color for the chat name and backgrounds of the chat photo, reply header, and link preview. See [accent colors](https://core.telegram.org/bots/api#accent-colors) for more details. Returned only in [`GetChat`](crate::methods::GetChat). Always returned in [`GetChat`](crate::methods::GetChat).
    pub accent_color_id: Option<i64>,
    /// Custom emoji identifier of emoji chosen by the chat for the reply header and link preview background. Returned only in [`GetChat`](crate::methods::GetChat).
    pub background_custom_emoji_id: Option<Box<str>>,
    /// Identifier of the accent color for the chat's profile background. See [profile accent colors](https://core.telegram.org/bots/api#profile-accent-colors) for more details. Returned only in [`GetChat`](crate::methods::GetChat).
    pub profile_accent_color_id: Option<i64>,
    /// Custom emoji identifier of the emoji chosen by the chat for its profile background. Returned only in [`GetChat`](crate::methods::GetChat).
    pub profile_background_custom_emoji_id: Option<Box<str>>,
    /// Custom emoji identifier of emoji status of the other party. Returned only in [`GetChat`](crate::methods::GetChat).
    pub emoji_status_custom_emoji_id: Option<Box<str>>,
    /// Expiration date of the emoji status of the other party in Unix time, if any. Returned only in [`GetChat`](crate::methods::GetChat).
    pub emoji_status_expiration_date: Option<i64>,
    /// Bio of the other party. Returned only in [`GetChat`](crate::methods::GetChat).
    pub bio: Option<Box<str>>,
    /// `true`, if privacy settings of the other party allows to use `tg://user?id=<user_id>` links only in chats with the user. Returned only in [`GetChat`](crate::methods::GetChat).
    pub has_private_forwards: Option<bool>,
    /// `true`, if the privacy settings of the other party restrict sending voice and video note messages. Returned only in [`GetChat`](crate::methods::GetChat).
    pub has_restricted_voice_and_video_messages: Option<bool>,
    /// The most recent pinned message (by sending date). Returned only in [`GetChat`](crate::methods::GetChat).
    pub pinned_message: Option<Box<Message>>,
    /// The time after which all messages sent to the chat will be automatically deleted; in seconds. Returned only in [`GetChat`](crate::methods::GetChat).
    pub message_auto_delete_time: Option<i64>,
}

#[derive(Debug, Clone, PartialEq, Deserialize)]
pub struct Group {
    /// Unique identifier for this chat. This number may have more than 32 significant bits and some programming languages may have difficulty/silent defects in i64erpreting it. But it has at most 52 significant bits, so a signed 64-bit i64eger or double-precision float type are safe for storing this identifier.
    pub id: i64,
    /// Title
    pub title: Box<str>,
    /// Chat photo. Returned only in [`GetChat`](crate::methods::GetChat).
    pub photo: Option<ChatPhoto>,
    /// Description. Returned only in [`GetChat`](crate::methods::GetChat).
    pub description: Option<Box<str>>,
    /// Primary invite link. Returned only in [`GetChat`](crate::methods::GetChat).
    pub invite_link: Option<Box<str>>,
    /// The most recent pinned message (by sending date). Returned only in [`GetChat`](crate::methods::GetChat).
    pub pinned_message: Option<Box<Message>>,
    /// Default chat member permissions. Returned only in [`GetChat`](crate::methods::GetChat).
    pub permissions: Option<ChatPermissions>,
    /// The time after which all messages sent to the chat will be automatically deleted; in seconds. Returned only in [`GetChat`](crate::methods::GetChat).
    pub message_auto_delete_time: Option<i64>,
    /// `true`, if non-administrators can only get the list of bots and administrators in the chat. Returned only in [`GetChat`](crate::methods::GetChat).
    pub has_hidden_members: Option<bool>,
    /// `true`, if messages from the chat can't be forwarded to other chats. Returned only in [`GetChat`](crate::methods::GetChat).
    pub has_protected_content: Option<bool>,
    /// `true`, if new chat members will have access to old messages; available only to chat administrators. Returned only in [`GetChat`](crate::methods::GetChat).
    pub has_visible_history: Option<bool>,
}

#[derive(Debug, Clone, PartialEq, Deserialize)]
pub struct Supergroup {
    /// Unique identifier for this chat. This number may have more than 32 significant bits and some programming languages may have difficulty/silent defects in i64erpreting it. But it has at most 52 significant bits, so a signed 64-bit i64eger or double-precision float type are safe for storing this identifier.
    pub id: i64,
    /// Title
    pub title: Box<str>,
    /// Username
    pub username: Option<Box<str>>,
    /// `true`, if the chat is a forum (has [`topics`](https://telegram.org/blog/topics-in-groups-collectible-usernames#topics-in-groups) enabled)
    pub is_forum: Option<bool>,
    /// Chat photo. Returned only in [`GetChat`](crate::methods::GetChat).
    pub photo: Option<ChatPhoto>,
    /// If non-empty, the list of all [active chat usernames](https://telegram.org/blog/topics-in-groups-collectible-usernames/ru?ln=a#collectible-usernames). Returned only in [`GetChat`](crate::methods::GetChat).
    pub active_usernames: Option<Box<[Box<str>]>>,
    /// List of available reactions allowed in the chat. If omitted, then all [emoji reactions](https://core.telegram.org/bots/api#reactiontypeemoji) are allowed. Returned only in [`GetChat`](crate::methods::GetChat).
    pub available_reactions: Option<Box<[Box<str>]>>,
    /// Identifier of the accent color for the chat name and backgrounds of the chat photo, reply header, and link preview. See [accent colors](https://core.telegram.org/bots/api#accent-colors) for more details. Returned only in [`GetChat`](crate::methods::GetChat). Always returned in [`GetChat`](crate::methods::GetChat).
    pub accent_color_id: Option<i64>,
    /// Custom emoji identifier of emoji chosen by the chat for the reply header and link preview background. Returned only in [`GetChat`](crate::methods::GetChat).
    pub background_custom_emoji_id: Option<Box<str>>,
    /// Identifier of the accent color for the chat's profile background. See [profile accent colors](https://core.telegram.org/bots/api#profile-accent-colors) for more details. Returned only in [`GetChat`](crate::methods::GetChat).
    pub profile_accent_color_id: Option<i64>,
    /// Custom emoji identifier of the emoji chosen by the chat for its profile background. Returned only in [`GetChat`](crate::methods::GetChat).
    pub profile_background_custom_emoji_id: Option<Box<str>>,
    /// Custom emoji identifier of the emoji status. Returned only in [`GetChat`](crate::methods::GetChat).
    pub emoji_status_custom_emoji_id: Option<Box<str>>,
    /// Expiration date of the emoji status in Unix time, if any. Returned only in [`GetChat`](crate::methods::GetChat).
    pub emoji_status_expiration_date: Option<i64>,
    /// `true`, if users need to join the supergroup before they can send messages. Returned only in [`GetChat`](crate::methods::GetChat).
    pub join_to_send_messages: Option<bool>,
    /// `true`, if all users directly joining the supergroup need to be approved by supergroup administrators. Returned only in [`GetChat`](crate::methods::GetChat).
    pub join_by_request: Option<bool>,
    /// Description. Returned only in [`GetChat`](crate::methods::GetChat).
    pub description: Option<Box<str>>,
    /// Primary invite link. Returned only in [`GetChat`](crate::methods::GetChat).
    pub invite_link: Option<Box<str>>,
    /// The most recent pinned message (by sending date). Returned only in [`GetChat`](crate::methods::GetChat).
    pub pinned_message: Option<Box<Message>>,
    /// Default chat member permissions. Returned only in [`GetChat`](crate::methods::GetChat).
    pub permissions: Option<ChatPermissions>,
    /// The minimum allowed delay between consecutive messages sent by each unpriviledged user; in seconds. Returned only in [`GetChat`](crate::methods::GetChat).
    pub slow_mode_delay: Option<i64>,
    /// The time after which all messages sent to the chat will be automatically deleted; in seconds. Returned only in [`GetChat`](crate::methods::GetChat).
    pub message_auto_delete_time: Option<i64>,
    /// `true`, if aggressive anti-spam checks are enabled in the supergroup. The field is only available to chat administrators. Returned only in [`GetChat`](crate::methods::GetChat).
    pub has_aggressive_anti_spam_enabled: Option<bool>,
    /// `true`, if non-administrators can only get the list of bots and administrators in the chat. Returned only in [`GetChat`](crate::methods::GetChat).
    pub has_hidden_members: Option<bool>,
    /// `true`, if messages from the chat can't be forwarded to other chats. Returned only in [`GetChat`](crate::methods::GetChat).
    pub has_protected_content: Option<bool>,
    /// `true`, if new chat members will have access to old messages; available only to chat administrators. Returned only in [`GetChat`](crate::methods::GetChat).
    pub has_visible_history: Option<bool>,
    /// Name of group sticker set. Returned only in [`GetChat`](crate::methods::GetChat).
    pub sticker_set_name: Option<Box<str>>,
    /// `true`, if the bot can change the group sticker set. Returned only in [`GetChat`](crate::methods::GetChat).
    pub can_set_sticker_set: Option<bool>,
    /// Unique identifier for the linked chat, i.e. the discussion group identifier for a channel and vice versa. This identifier may be greater than 32 bits and some programming languages may have difficulty/silent defects in interpreting it. But it is smaller than 52 bits, so a signed 64 bit integer or double-precision float type are safe for storing this identifier. Returned only in [`GetChat`](crate::methods::GetChat).
    pub linked_chat_id: Option<i64>,
    /// The location to which the supergroup is connected. Returned only in [`GetChat`](crate::methods::GetChat).
    pub location: Option<ChatLocation>,
}

#[derive(Debug, Clone, PartialEq, Deserialize)]
pub struct Channel {
    /// Unique identifier for this chat. This number may have more than 32 significant bits and some programming languages may have difficulty/silent defects in i64erpreting it. But it has at most 52 significant bits, so a signed 64-bit i64eger or double-precision float type are safe for storing this identifier.
    pub id: i64,
    /// Title
    pub title: Box<str>,
    /// Username
    pub username: Option<Box<str>>,
    /// Chat photo. Returned only in [`GetChat`](crate::methods::GetChat).
    pub photo: Option<ChatPhoto>,
    /// If non-empty, the list of all [active chat usernames](https://telegram.org/blog/topics-in-groups-collectible-usernames/ru?ln=a#collectible-usernames). Returned only in [`GetChat`](crate::methods::GetChat).
    pub active_usernames: Option<Box<[Box<str>]>>,
    /// List of available reactions allowed in the chat. If omitted, then all [emoji reactions](https://core.telegram.org/bots/api#reactiontypeemoji) are allowed. Returned only in [`GetChat`](crate::methods::GetChat).
    pub available_reactions: Option<Box<[Box<str>]>>,
    /// Identifier of the accent color for the chat name and backgrounds of the chat photo, reply header, and link preview. See [accent colors](https://core.telegram.org/bots/api#accent-colors) for more details. Returned only in [`GetChat`](crate::methods::GetChat). Always returned in [`GetChat`](crate::methods::GetChat).
    pub accent_color_id: Option<i64>,
    /// Custom emoji identifier of emoji chosen by the chat for the reply header and link preview background. Returned only in [`GetChat`](crate::methods::GetChat).
    pub background_custom_emoji_id: Option<Box<str>>,
    /// Identifier of the accent color for the chat's profile background. See [profile accent colors](https://core.telegram.org/bots/api#profile-accent-colors) for more details. Returned only in [`GetChat`](crate::methods::GetChat).
    pub profile_accent_color_id: Option<i64>,
    /// Custom emoji identifier of the emoji chosen by the chat for its profile background. Returned only in [`GetChat`](crate::methods::GetChat).
    pub profile_background_custom_emoji_id: Option<Box<str>>,
    /// Custom emoji identifier of the emoji status. Returned only in [`GetChat`](crate::methods::GetChat).
    pub emoji_status_custom_emoji_id: Option<Box<str>>,
    /// Expiration date of the emoji status in Unix time, if any. Returned only in [`GetChat`](crate::methods::GetChat).
    pub emoji_status_expiration_date: Option<i64>,
    /// Description. Returned only in [`GetChat`](crate::methods::GetChat).
    pub description: Option<Box<str>>,
    /// Primary invite link. Returned only in [`GetChat`](crate::methods::GetChat).
    pub invite_link: Option<Box<str>>,
    /// The most recent pinned message (by sending date). Returned only in [`GetChat`](crate::methods::GetChat).
    pub pinned_message: Option<Box<Message>>,
    /// The time after which all messages sent to the chat will be automatically deleted; in seconds. Returned only in [`GetChat`](crate::methods::GetChat).
    pub message_auto_delete_time: Option<i64>,
    /// `true`, if non-administrators can only get the list of bots and administrators in the chat. Returned only in [`GetChat`](crate::methods::GetChat).
    pub has_hidden_members: Option<bool>,
    /// `true`, if messages from the chat can't be forwarded to other chats. Returned only in [`GetChat`](crate::methods::GetChat).
    pub has_protected_content: Option<bool>,
    /// Unique identifier for the linked chat, i.e. the discussion group identifier for a channel and vice versa. This identifier may be greater than 32 bits and some programming languages may have difficulty/silent defects in interpreting it. But it is smaller than 52 bits, so a signed 64 bit integer or double-precision float type are safe for storing this identifier. Returned only in [`GetChat`](crate::methods::GetChat).
    pub linked_chat_id: Option<i64>,
}

impl Chat {
    #[must_use]
    pub const fn id(&self) -> i64 {
        match self {
            Self::Private(Private { id, .. })
            | Self::Group(Group { id, .. })
            | Self::Supergroup(Supergroup { id, .. })
            | Self::Channel(Channel { id, .. }) => *id,
        }
    }

    #[must_use]
    pub const fn username(&self) -> Option<&str> {
        match self {
            Self::Group(_) => None,
            Self::Private(Private { username, .. })
            | Self::Supergroup(Supergroup { username, .. })
            | Self::Channel(Channel { username, .. }) => match username {
                Some(username) => Some(username),
                None => None,
            },
        }
    }

    #[must_use]
    pub const fn photo(&self) -> Option<&ChatPhoto> {
        match self {
            Self::Private(Private { photo, .. })
            | Self::Group(Group { photo, .. })
            | Self::Supergroup(Supergroup { photo, .. })
            | Self::Channel(Channel { photo, .. }) => photo.as_ref(),
        }
    }

    #[must_use]
    pub const fn title(&self) -> Option<&str> {
        match self {
            Self::Private(_) => None,
            Self::Group(Group { title, .. })
            | Self::Supergroup(Supergroup { title, .. })
            | Self::Channel(Channel { title, .. }) => Some(title),
        }
    }

    #[must_use]
    pub const fn active_usernames(&self) -> Option<&[Box<str>]> {
        match self {
            Self::Group(_) => None,
            Self::Private(Private {
                active_usernames, ..
            })
            | Self::Supergroup(Supergroup {
                active_usernames, ..
            })
            | Self::Channel(Channel {
                active_usernames, ..
            }) => match active_usernames {
                Some(active_usernames) => Some(active_usernames),
                None => None,
            },
        }
    }

    #[must_use]
    pub const fn available_reactions(&self) -> Option<&[Box<str>]> {
        match self {
            Self::Group(_) | Self::Private(_) => None,
            Self::Supergroup(Supergroup {
                available_reactions,
                ..
            })
            | Self::Channel(Channel {
                available_reactions,
                ..
            }) => match available_reactions {
                Some(available_reactions) => Some(available_reactions),
                None => None,
            },
        }
    }

    #[must_use]
    pub const fn accent_color_id(&self) -> Option<i64> {
        match self {
            Self::Group(_) | Self::Private(_) => None,
            Self::Supergroup(Supergroup {
                accent_color_id, ..
            })
            | Self::Channel(Channel {
                accent_color_id, ..
            }) => *accent_color_id,
        }
    }

    #[must_use]
    pub const fn background_custom_emoji_id(&self) -> Option<&str> {
        match self {
            Self::Group(_) | Self::Private(_) => None,
            Self::Supergroup(Supergroup {
                background_custom_emoji_id,
                ..
            })
            | Self::Channel(Channel {
                background_custom_emoji_id,
                ..
            }) => match background_custom_emoji_id {
                Some(background_custom_emoji_id) => Some(background_custom_emoji_id),
                None => None,
            },
        }
    }

    #[must_use]
    pub const fn profile_accent_color_id(&self) -> Option<i64> {
        match self {
            Self::Group(_) | Self::Private(_) => None,
            Self::Supergroup(Supergroup {
                profile_accent_color_id,
                ..
            })
            | Self::Channel(Channel {
                profile_accent_color_id,
                ..
            }) => *profile_accent_color_id,
        }
    }

    #[must_use]
    pub const fn profile_background_custom_emoji_id(&self) -> Option<&str> {
        match self {
            Self::Group(_) | Self::Private(_) => None,
            Self::Supergroup(Supergroup {
                profile_background_custom_emoji_id,
                ..
            })
            | Self::Channel(Channel {
                profile_background_custom_emoji_id,
                ..
            }) => match profile_background_custom_emoji_id {
                Some(profile_background_custom_emoji_id) => {
                    Some(profile_background_custom_emoji_id)
                }
                None => None,
            },
        }
    }

    #[must_use]
    pub const fn has_visible_history(&self) -> Option<bool> {
        match self {
            Self::Private(_) | Self::Channel(_) => None,
            Self::Group(Group {
                has_visible_history,
                ..
            })
            | Self::Supergroup(Supergroup {
                has_visible_history,
                ..
            }) => *has_visible_history,
        }
    }

    #[must_use]
    pub const fn description(&self) -> Option<&str> {
        match self {
            Self::Private(_) => None,
            Self::Group(Group { description, .. })
            | Self::Supergroup(Supergroup { description, .. })
            | Self::Channel(Channel { description, .. }) => match description {
                Some(description) => Some(description),
                None => None,
            },
        }
    }

    #[must_use]
    pub const fn invite_link(&self) -> Option<&str> {
        match self {
            Self::Private(_) => None,
            Self::Group(Group { invite_link, .. })
            | Self::Supergroup(Supergroup { invite_link, .. })
            | Self::Channel(Channel { invite_link, .. }) => match invite_link {
                Some(invite_link) => Some(invite_link),
                None => None,
            },
        }
    }

    #[must_use]
    pub const fn has_protected_content(&self) -> Option<bool> {
        match self {
            Self::Private(_) => None,
            Self::Group(Group {
                has_protected_content,
                ..
            })
            | Self::Supergroup(Supergroup {
                has_protected_content,
                ..
            })
            | Self::Channel(Channel {
                has_protected_content,
                ..
            }) => *has_protected_content,
        }
    }

    #[must_use]
    pub const fn linked_chat_id(&self) -> Option<i64> {
        match self {
            Self::Private(_) | Self::Group(_) => None,
            Self::Supergroup(Supergroup { linked_chat_id, .. })
            | Self::Channel(Channel { linked_chat_id, .. }) => *linked_chat_id,
        }
    }
}

impl Default for Chat {
    #[must_use]
    fn default() -> Self {
        Self::Private(Private::default())
    }
}
