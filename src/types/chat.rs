use super::{ChatLocation, ChatPermissions, ChatPhoto, Message};

use serde::Deserialize;

/// This object represents a chat.
/// # Documentation
/// <https://core.telegram.org/bots/api#chat>
#[derive(Default, Clone, Debug, PartialEq, Deserialize)]
pub struct Chat {
    /// Unique identifier for this chat. This number may have more than 32 significant bits and some programming languages may have difficulty/silent defects in i64erpreting it. But it has at most 52 significant bits, so a signed 64-bit i64eger or double-precision float type are safe for storing this identifier.
    pub id: i64,
    /// Type of chat, can be either 'private', 'group', 'supergroup' or 'channel'
    #[serde(rename = "type")]
    pub chat_type: String,
    /// *Optional*. Title, for supergroups, channels and group chats
    pub title: Option<String>,
    /// *Optional*. Username, for private chats, supergroups and channels if available
    pub username: Option<String>,
    /// *Optional*. First name of the other party in a private chat
    pub first_name: Option<String>,
    /// *Optional*. Last name of the other party in a private chat
    pub last_name: Option<String>,
    /// *Optional*. `True`, if the supergroup chat is a forum (has [`topics`](https://telegram.org/blog/topics-in-groups-collectible-usernames#topics-in-groups) enabled)
    pub is_forum: Option<bool>,
    /// *Optional*. Chat photo. Returned only in [`GetChat`](crate::methods::GetChat).
    pub photo: Option<ChatPhoto>,
    /// *Optional*. If non-empty, the list of all [`active chat usernames`](https://telegram.org/blog/topics-in-groups-collectible-usernames#collectible-usernames); for private chats, supergroups and channels. Returned only in [`GetChat`](crate::methods::GetChat).
    pub active_usernames: Option<Vec<String>>,
    /// *Optional*. Custom emoji identifier of emoji status of the other party in a private chat. Returned only in [`GetChat`](crate::methods::GetChat).
    pub emoji_status_custom_emoji_id: Option<String>,
    /// *Optional*. Bio of the other party in a private chat. Returned only in [`GetChat`](crate::methods::GetChat).
    pub bio: Option<String>,
    /// *Optional*. `True`, if privacy settings of the other party in the private chat allows to use `tg://user?id=<user_id>` links only in chats with the user. Returned only in [`GetChat`](crate::methods::GetChat).
    pub has_private_forwards: Option<bool>,
    /// *Optional*. `True`, if the privacy settings of the other party restrict sending voice and video note messages in the private chat. Returned only in [`GetChat`](crate::methods::GetChat).
    pub has_restricted_voice_and_video_messages: Option<bool>,
    /// *Optional*. `True`, if users need to join the supergroup before they can send messages. Returned only in [`GetChat`](crate::methods::GetChat).
    pub join_to_send_messages: Option<bool>,
    /// *Optional*. `True`, if all users directly joining the supergroup need to be approved by supergroup administrators. Returned only in [`GetChat`](crate::methods::GetChat).
    pub join_by_request: Option<bool>,
    /// *Optional*. Description, for groups, supergroups and channel chats. Returned only in [`GetChat`](crate::methods::GetChat).
    pub description: Option<String>,
    /// *Optional*. Primary invite link, for groups, supergroups and channel chats. Returned only in [`GetChat`](crate::methods::GetChat).
    pub invite_link: Option<String>,
    /// *Optional*. The most recent pinned message (by sending date). Returned only in [`GetChat`](crate::methods::GetChat).
    pub pinned_message: Option<Message>,
    /// *Optional*. Default chat member permissions, for groups and supergroups. Returned only in [`GetChat`](crate::methods::GetChat).
    pub permissions: Option<ChatPermissions>,
    /// *Optional*. For supergroups, the minimum allowed delay between consecutive messages sent by each unpriviledged user; in seconds. Returned only in [`GetChat`](crate::methods::GetChat).
    pub slow_mode_delay: Option<i64>,
    /// *Optional*. The time after which all messages sent to the chat will be automatically deleted; in seconds. Returned only in [`GetChat`](crate::methods::GetChat).
    pub message_auto_delete_time: Option<i64>,
    /// *Optional*. `True`, if aggressive anti-spam checks are enabled in the supergroup. The field is only available to chat administrators. Returned only in [`GetChat`](crate::methods::GetChat).
    pub has_aggressive_anti_spam_enabled: Option<bool>,
    /// *Optional*. `True`, if non-administrators can only get the list of bots and administrators in the chat. Returned only in [`GetChat`](crate::methods::GetChat).
    pub has_hidden_members: Option<bool>,
    /// *Optional*. `True`, if messages from the chat can't be forwarded to other chats. Returned only in [`GetChat`](crate::methods::GetChat).
    pub has_protected_content: Option<bool>,
    /// *Optional*. For supergroups, name of group sticker set. Returned only in [`GetChat`](crate::methods::GetChat).
    pub sticker_set_name: Option<String>,
    /// *Optional*. `True`, if the bot can change the group sticker set. Returned only in [`GetChat`](crate::methods::GetChat).
    pub can_set_sticker_set: Option<bool>,
    /// *Optional*. Unique identifier for the linked chat, i.e. the discussion group identifier for a channel and vice versa; for supergroups and channel chats. This identifier may be greater than 32 bits and some programming languages may have difficulty/silent defects in i64erpreting it. But it is smaller than 52 bits, so a signed 64 bit i64eger or double-precision float type are safe for storing this identifier. Returned only in [`GetChat`](crate::methods::GetChat).
    pub linked_chat_id: Option<i64>,
    /// *Optional*. For supergroups, the location to which the supergroup is connected. Returned only in [`GetChat`](crate::methods::GetChat).
    pub location: Option<ChatLocation>,
}
