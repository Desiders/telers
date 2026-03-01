use serde::{Deserialize, Serialize};
/// This object contains full information about a chat.
/// Currently, it can be one of
/// - [`ChatFullInfoChannel`]
/// - [`ChatFullInfoGroup`]
/// - [`ChatFullInfoPrivate`]
/// - [`ChatFullInfoSupergroup`]
/// # Documentation
/// <https://core.telegram.org/bots/api#chatfullinfo>
#[derive(Clone, Debug, Serialize, Deserialize)]
#[serde(tag = "type", rename_all = "snake_case")]
pub enum ChatFullInfo {
    Private(crate::types::ChatFullInfoPrivate),
    Group(crate::types::ChatFullInfoGroup),
    Supergroup(crate::types::ChatFullInfoSupergroup),
    Channel(crate::types::ChatFullInfoChannel),
}
impl ChatFullInfo {
    /// Helper method for field `accent_color_id`.
    ///
    /// # Variants
    /// - `ChatFullInfoPrivate`. Identifier of the accent color for the chat name and backgrounds of the chat photo, reply header, and link preview. See accent colors for more details.
    /// - `ChatFullInfoGroup`. Identifier of the accent color for the chat name and backgrounds of the chat photo, reply header, and link preview. See accent colors for more details.
    /// - `ChatFullInfoSupergroup`. Identifier of the accent color for the chat name and backgrounds of the chat photo, reply header, and link preview. See accent colors for more details.
    /// - `ChatFullInfoChannel`. Identifier of the accent color for the chat name and backgrounds of the chat photo, reply header, and link preview. See accent colors for more details.
    #[must_use]
    pub fn accent_color_id(&self) -> i64 {
        match self {
            Self::Private(val) => val.accent_color_id,
            Self::Group(val) => val.accent_color_id,
            Self::Supergroup(val) => val.accent_color_id,
            Self::Channel(val) => val.accent_color_id,
        }
    }

    /// Helper method for field `accepted_gift_types`.
    ///
    /// # Variants
    /// - `ChatFullInfoPrivate`. Information about types of gifts that are accepted by the chat or by the corresponding user for private chats
    #[must_use]
    pub fn accepted_gift_types(&self) -> Option<&crate::types::AcceptedGiftTypes> {
        match self {
            Self::Private(val) => Some(&val.accepted_gift_types),
            _ => None,
        }
    }

    /// Helper method for field `active_usernames`.
    ///
    /// # Variants
    /// - `ChatFullInfoPrivate`. If non-empty, the list of all active chat usernames; for private chats, supergroups and channels
    /// - `ChatFullInfoSupergroup`. If non-empty, the list of all active chat usernames; for private chats, supergroups and channels
    /// - `ChatFullInfoChannel`. If non-empty, the list of all active chat usernames; for private chats, supergroups and channels
    #[must_use]
    pub fn active_usernames(&self) -> Option<&[Box<str>]> {
        match self {
            Self::Private(val) => val.active_usernames.as_deref(),
            Self::Supergroup(val) => val.active_usernames.as_deref(),
            Self::Channel(val) => val.active_usernames.as_deref(),
            Self::Group(_) => None,
        }
    }

    /// Helper method for field `available_reactions`.
    ///
    /// # Variants
    /// - `ChatFullInfoPrivate`. List of available reactions allowed in the chat. If omitted, then all emoji reactions are allowed.
    /// - `ChatFullInfoGroup`. List of available reactions allowed in the chat. If omitted, then all emoji reactions are allowed.
    /// - `ChatFullInfoSupergroup`. List of available reactions allowed in the chat. If omitted, then all emoji reactions are allowed.
    /// - `ChatFullInfoChannel`. List of available reactions allowed in the chat. If omitted, then all emoji reactions are allowed.
    #[must_use]
    pub fn available_reactions(&self) -> Option<&[crate::types::ReactionType]> {
        match self {
            Self::Private(val) => val.available_reactions.as_deref(),
            Self::Group(val) => val.available_reactions.as_deref(),
            Self::Supergroup(val) => val.available_reactions.as_deref(),
            Self::Channel(val) => val.available_reactions.as_deref(),
        }
    }

    /// Helper method for field `background_custom_emoji_id`.
    ///
    /// # Variants
    /// - `ChatFullInfoPrivate`. Custom emoji identifier of the emoji chosen by the chat for the reply header and link preview background
    /// - `ChatFullInfoGroup`. Custom emoji identifier of the emoji chosen by the chat for the reply header and link preview background
    /// - `ChatFullInfoSupergroup`. Custom emoji identifier of the emoji chosen by the chat for the reply header and link preview background
    /// - `ChatFullInfoChannel`. Custom emoji identifier of the emoji chosen by the chat for the reply header and link preview background
    #[must_use]
    pub fn background_custom_emoji_id(&self) -> Option<&str> {
        match self {
            Self::Private(val) => val.background_custom_emoji_id.as_deref(),
            Self::Group(val) => val.background_custom_emoji_id.as_deref(),
            Self::Supergroup(val) => val.background_custom_emoji_id.as_deref(),
            Self::Channel(val) => val.background_custom_emoji_id.as_deref(),
        }
    }

    /// Helper method for field `bio`.
    ///
    /// # Variants
    /// - `ChatFullInfoPrivate`. Bio of the other party in a private chat
    #[must_use]
    pub fn bio(&self) -> Option<&str> {
        match self {
            Self::Private(val) => val.bio.as_deref(),
            _ => None,
        }
    }

    /// Helper method for field `birthdate`.
    ///
    /// # Variants
    /// - `ChatFullInfoPrivate`. For private chats, the date of birth of the user
    #[must_use]
    pub fn birthdate(&self) -> Option<&crate::types::Birthdate> {
        match self {
            Self::Private(val) => val.birthdate.as_ref(),
            _ => None,
        }
    }

    /// Helper method for field `business_intro`.
    ///
    /// # Variants
    /// - `ChatFullInfoPrivate`. For private chats with business accounts, the intro of the business
    #[must_use]
    pub fn business_intro(&self) -> Option<&crate::types::BusinessIntro> {
        match self {
            Self::Private(val) => val.business_intro.as_ref(),
            _ => None,
        }
    }

    /// Helper method for field `business_location`.
    ///
    /// # Variants
    /// - `ChatFullInfoPrivate`. For private chats with business accounts, the location of the business
    #[must_use]
    pub fn business_location(&self) -> Option<&crate::types::BusinessLocation> {
        match self {
            Self::Private(val) => val.business_location.as_ref(),
            _ => None,
        }
    }

    /// Helper method for field `business_opening_hours`.
    ///
    /// # Variants
    /// - `ChatFullInfoPrivate`. For private chats with business accounts, the opening hours of the business
    #[must_use]
    pub fn business_opening_hours(&self) -> Option<&crate::types::BusinessOpeningHours> {
        match self {
            Self::Private(val) => val.business_opening_hours.as_ref(),
            _ => None,
        }
    }

    /// Helper method for field `can_send_paid_media`.
    ///
    /// # Variants
    /// - `ChatFullInfoChannel`. `true`, if paid media messages can be sent or forwarded to the channel chat. The field is available only for channel chats.
    #[must_use]
    pub fn can_send_paid_media(&self) -> Option<bool> {
        match self {
            Self::Channel(val) => val.can_send_paid_media,
            _ => None,
        }
    }

    /// Helper method for field `can_set_sticker_set`.
    ///
    /// # Variants
    /// - `ChatFullInfoGroup`. `true`, if the bot can change the group sticker set
    #[must_use]
    pub fn can_set_sticker_set(&self) -> Option<bool> {
        match self {
            Self::Group(val) => val.can_set_sticker_set,
            _ => None,
        }
    }

    /// Helper method for field `custom_emoji_sticker_set_name`.
    ///
    /// # Variants
    /// - `ChatFullInfoSupergroup`. For supergroups, the name of the group's custom emoji sticker set. Custom emoji from this set can be used by all users and bots in the group.
    #[must_use]
    pub fn custom_emoji_sticker_set_name(&self) -> Option<&str> {
        match self {
            Self::Supergroup(val) => val.custom_emoji_sticker_set_name.as_deref(),
            _ => None,
        }
    }

    /// Helper method for field `description`.
    ///
    /// # Variants
    /// - `ChatFullInfoSupergroup`. Description, for groups, supergroups and channel chats
    /// - `ChatFullInfoChannel`. Description, for groups, supergroups and channel chats
    #[must_use]
    pub fn description(&self) -> Option<&str> {
        match self {
            Self::Supergroup(val) => val.description.as_deref(),
            Self::Channel(val) => val.description.as_deref(),
            _ => None,
        }
    }

    /// Helper method for field `emoji_status_custom_emoji_id`.
    ///
    /// # Variants
    /// - `ChatFullInfoPrivate`. Custom emoji identifier of the emoji status of the chat or the other party in a private chat
    #[must_use]
    pub fn emoji_status_custom_emoji_id(&self) -> Option<&str> {
        match self {
            Self::Private(val) => val.emoji_status_custom_emoji_id.as_deref(),
            _ => None,
        }
    }

    /// Helper method for field `emoji_status_expiration_date`.
    ///
    /// # Variants
    /// - `ChatFullInfoPrivate`. Expiration date of the emoji status of the chat or the other party in a private chat, in Unix time, if any
    #[must_use]
    pub fn emoji_status_expiration_date(&self) -> Option<i64> {
        match self {
            Self::Private(val) => val.emoji_status_expiration_date,
            _ => None,
        }
    }

    /// Helper method for field `first_name`.
    ///
    /// # Variants
    /// - `ChatFullInfoPrivate`. First name of the other party in a private chat
    #[must_use]
    pub fn first_name(&self) -> Option<&str> {
        match self {
            Self::Private(val) => val.first_name.as_deref(),
            _ => None,
        }
    }

    /// Helper method for field `first_profile_audio`.
    ///
    /// # Variants
    /// - `ChatFullInfoPrivate`. For private chats, the first audio added to the profile of the user
    #[must_use]
    pub fn first_profile_audio(&self) -> Option<&crate::types::Audio> {
        match self {
            Self::Private(val) => val.first_profile_audio.as_deref(),
            _ => None,
        }
    }

    /// Helper method for field `has_aggressive_anti_spam_enabled`.
    ///
    /// # Variants
    /// - `ChatFullInfoSupergroup`. `true`, if aggressive anti-spam checks are enabled in the supergroup. The field is only available to chat administrators.
    #[must_use]
    pub fn has_aggressive_anti_spam_enabled(&self) -> Option<bool> {
        match self {
            Self::Supergroup(val) => val.has_aggressive_anti_spam_enabled,
            _ => None,
        }
    }

    /// Helper method for field `has_hidden_members`.
    ///
    /// # Variants
    /// - `ChatFullInfoPrivate`. `true`, if non-administrators can only get the list of bots and administrators in the chat
    /// - `ChatFullInfoGroup`. `true`, if non-administrators can only get the list of bots and administrators in the chat
    /// - `ChatFullInfoSupergroup`. `true`, if non-administrators can only get the list of bots and administrators in the chat
    /// - `ChatFullInfoChannel`. `true`, if non-administrators can only get the list of bots and administrators in the chat
    #[must_use]
    pub fn has_hidden_members(&self) -> Option<bool> {
        match self {
            Self::Private(val) => val.has_hidden_members,
            Self::Group(val) => val.has_hidden_members,
            Self::Supergroup(val) => val.has_hidden_members,
            Self::Channel(val) => val.has_hidden_members,
        }
    }

    /// Helper method for field `has_private_forwards`.
    ///
    /// # Variants
    /// - `ChatFullInfoPrivate`. `true`, if privacy settings of the other party in the private chat allows to use ``tg://user?id=<user_id>`` links only in chats with the user
    #[must_use]
    pub fn has_private_forwards(&self) -> Option<bool> {
        match self {
            Self::Private(val) => val.has_private_forwards,
            _ => None,
        }
    }

    /// Helper method for field `has_protected_content`.
    ///
    /// # Variants
    /// - `ChatFullInfoPrivate`. `true`, if messages from the chat can't be forwarded to other chats
    /// - `ChatFullInfoGroup`. `true`, if messages from the chat can't be forwarded to other chats
    /// - `ChatFullInfoSupergroup`. `true`, if messages from the chat can't be forwarded to other chats
    /// - `ChatFullInfoChannel`. `true`, if messages from the chat can't be forwarded to other chats
    #[must_use]
    pub fn has_protected_content(&self) -> Option<bool> {
        match self {
            Self::Private(val) => val.has_protected_content,
            Self::Group(val) => val.has_protected_content,
            Self::Supergroup(val) => val.has_protected_content,
            Self::Channel(val) => val.has_protected_content,
        }
    }

    /// Helper method for field `has_restricted_voice_and_video_messages`.
    ///
    /// # Variants
    /// - `ChatFullInfoPrivate`. `true`, if the privacy settings of the other party restrict sending voice and video note messages in the private chat
    #[must_use]
    pub fn has_restricted_voice_and_video_messages(&self) -> Option<bool> {
        match self {
            Self::Private(val) => val.has_restricted_voice_and_video_messages,
            _ => None,
        }
    }

    /// Helper method for field `has_visible_history`.
    ///
    /// # Variants
    /// - `ChatFullInfoPrivate`. `true`, if new chat members will have access to old messages; available only to chat administrators
    /// - `ChatFullInfoGroup`. `true`, if new chat members will have access to old messages; available only to chat administrators
    /// - `ChatFullInfoSupergroup`. `true`, if new chat members will have access to old messages; available only to chat administrators
    /// - `ChatFullInfoChannel`. `true`, if new chat members will have access to old messages; available only to chat administrators
    #[must_use]
    pub fn has_visible_history(&self) -> Option<bool> {
        match self {
            Self::Private(val) => val.has_visible_history,
            Self::Group(val) => val.has_visible_history,
            Self::Supergroup(val) => val.has_visible_history,
            Self::Channel(val) => val.has_visible_history,
        }
    }

    /// Helper method for field `id`.
    ///
    /// # Variants
    /// - `ChatFullInfoPrivate`. Unique identifier for this chat. This number may have more than 32 significant bits and some programming languages may have difficulty/silent defects in interpreting it. But it has at most 52 significant bits, so a signed 64-bit integer or double-precision float type are safe for storing this identifier.
    /// - `ChatFullInfoGroup`. Unique identifier for this chat. This number may have more than 32 significant bits and some programming languages may have difficulty/silent defects in interpreting it. But it has at most 52 significant bits, so a signed 64-bit integer or double-precision float type are safe for storing this identifier.
    /// - `ChatFullInfoSupergroup`. Unique identifier for this chat. This number may have more than 32 significant bits and some programming languages may have difficulty/silent defects in interpreting it. But it has at most 52 significant bits, so a signed 64-bit integer or double-precision float type are safe for storing this identifier.
    /// - `ChatFullInfoChannel`. Unique identifier for this chat. This number may have more than 32 significant bits and some programming languages may have difficulty/silent defects in interpreting it. But it has at most 52 significant bits, so a signed 64-bit integer or double-precision float type are safe for storing this identifier.
    #[must_use]
    pub fn id(&self) -> i64 {
        match self {
            Self::Private(val) => val.id,
            Self::Group(val) => val.id,
            Self::Supergroup(val) => val.id,
            Self::Channel(val) => val.id,
        }
    }

    /// Helper method for field `invite_link`.
    ///
    /// # Variants
    /// - `ChatFullInfoSupergroup`. Primary invite link, for groups, supergroups and channel chats
    /// - `ChatFullInfoChannel`. Primary invite link, for groups, supergroups and channel chats
    #[must_use]
    pub fn invite_link(&self) -> Option<&str> {
        match self {
            Self::Supergroup(val) => val.invite_link.as_deref(),
            Self::Channel(val) => val.invite_link.as_deref(),
            _ => None,
        }
    }

    /// Helper method for field `is_direct_messages`.
    ///
    /// # Variants
    /// - `ChatFullInfoPrivate`. `true`, if the chat is the direct messages chat of a channel
    /// - `ChatFullInfoGroup`. `true`, if the chat is the direct messages chat of a channel
    /// - `ChatFullInfoSupergroup`. `true`, if the chat is the direct messages chat of a channel
    /// - `ChatFullInfoChannel`. `true`, if the chat is the direct messages chat of a channel
    #[must_use]
    pub fn is_direct_messages(&self) -> Option<bool> {
        match self {
            Self::Private(val) => val.is_direct_messages,
            Self::Group(val) => val.is_direct_messages,
            Self::Supergroup(val) => val.is_direct_messages,
            Self::Channel(val) => val.is_direct_messages,
        }
    }

    /// Helper method for field `is_forum`.
    ///
    /// # Variants
    /// - `ChatFullInfoSupergroup`. `true`, if the supergroup chat is a forum (has topics enabled)
    #[must_use]
    pub fn is_forum(&self) -> Option<bool> {
        match self {
            Self::Supergroup(val) => val.is_forum,
            _ => None,
        }
    }

    /// Helper method for field `join_by_request`.
    ///
    /// # Variants
    /// - `ChatFullInfoSupergroup`. `true`, if all users directly joining the supergroup without using an invite link need to be approved by supergroup administrators
    #[must_use]
    pub fn join_by_request(&self) -> Option<bool> {
        match self {
            Self::Supergroup(val) => val.join_by_request,
            _ => None,
        }
    }

    /// Helper method for field `join_to_send_messages`.
    ///
    /// # Variants
    /// - `ChatFullInfoSupergroup`. `true`, if users need to join the supergroup before they can send messages
    #[must_use]
    pub fn join_to_send_messages(&self) -> Option<bool> {
        match self {
            Self::Supergroup(val) => val.join_to_send_messages,
            _ => None,
        }
    }

    /// Helper method for field `last_name`.
    ///
    /// # Variants
    /// - `ChatFullInfoPrivate`. Last name of the other party in a private chat
    #[must_use]
    pub fn last_name(&self) -> Option<&str> {
        match self {
            Self::Private(val) => val.last_name.as_deref(),
            _ => None,
        }
    }

    /// Helper method for field `linked_chat_id`.
    ///
    /// # Variants
    /// - `ChatFullInfoSupergroup`. Unique identifier for the linked chat, i.e. the discussion group identifier for a channel and vice versa; for supergroups and channel chats. This identifier may be greater than 32 bits and some programming languages may have difficulty/silent defects in interpreting it. But it is smaller than 52 bits, so a signed 64 bit integer or double-precision float type are safe for storing this identifier.
    /// - `ChatFullInfoChannel`. Unique identifier for the linked chat, i.e. the discussion group identifier for a channel and vice versa; for supergroups and channel chats. This identifier may be greater than 32 bits and some programming languages may have difficulty/silent defects in interpreting it. But it is smaller than 52 bits, so a signed 64 bit integer or double-precision float type are safe for storing this identifier.
    #[must_use]
    pub fn linked_chat_id(&self) -> Option<i64> {
        match self {
            Self::Supergroup(val) => val.linked_chat_id,
            Self::Channel(val) => val.linked_chat_id,
            _ => None,
        }
    }

    /// Helper method for field `location`.
    ///
    /// # Variants
    /// - `ChatFullInfoSupergroup`. For supergroups, the location to which the supergroup is connected
    #[must_use]
    pub fn location(&self) -> Option<&crate::types::ChatLocation> {
        match self {
            Self::Supergroup(val) => val.location.as_ref(),
            _ => None,
        }
    }

    /// Helper method for field `max_reaction_count`.
    ///
    /// # Variants
    /// - `ChatFullInfoPrivate`. The maximum number of reactions that can be set on a message in the chat
    /// - `ChatFullInfoGroup`. The maximum number of reactions that can be set on a message in the chat
    /// - `ChatFullInfoSupergroup`. The maximum number of reactions that can be set on a message in the chat
    /// - `ChatFullInfoChannel`. The maximum number of reactions that can be set on a message in the chat
    #[must_use]
    pub fn max_reaction_count(&self) -> i64 {
        match self {
            Self::Private(val) => val.max_reaction_count,
            Self::Group(val) => val.max_reaction_count,
            Self::Supergroup(val) => val.max_reaction_count,
            Self::Channel(val) => val.max_reaction_count,
        }
    }

    /// Helper method for field `message_auto_delete_time`.
    ///
    /// # Variants
    /// - `ChatFullInfoPrivate`. The time after which all messages sent to the chat will be automatically deleted; in seconds
    /// - `ChatFullInfoGroup`. The time after which all messages sent to the chat will be automatically deleted; in seconds
    /// - `ChatFullInfoSupergroup`. The time after which all messages sent to the chat will be automatically deleted; in seconds
    /// - `ChatFullInfoChannel`. The time after which all messages sent to the chat will be automatically deleted; in seconds
    #[must_use]
    pub fn message_auto_delete_time(&self) -> Option<i64> {
        match self {
            Self::Private(val) => val.message_auto_delete_time,
            Self::Group(val) => val.message_auto_delete_time,
            Self::Supergroup(val) => val.message_auto_delete_time,
            Self::Channel(val) => val.message_auto_delete_time,
        }
    }

    /// Helper method for field `paid_message_star_count`.
    ///
    /// # Variants
    /// - `ChatFullInfoPrivate`. The number of Telegram Stars a general user have to pay to send a message to the chat
    /// - `ChatFullInfoGroup`. The number of Telegram Stars a general user have to pay to send a message to the chat
    /// - `ChatFullInfoSupergroup`. The number of Telegram Stars a general user have to pay to send a message to the chat
    /// - `ChatFullInfoChannel`. The number of Telegram Stars a general user have to pay to send a message to the chat
    #[must_use]
    pub fn paid_message_star_count(&self) -> Option<i64> {
        match self {
            Self::Private(val) => val.paid_message_star_count,
            Self::Group(val) => val.paid_message_star_count,
            Self::Supergroup(val) => val.paid_message_star_count,
            Self::Channel(val) => val.paid_message_star_count,
        }
    }

    /// Helper method for field `parent_chat`.
    ///
    /// # Variants
    /// - `ChatFullInfoChannel`. Information about the corresponding channel chat; for direct messages chats only
    #[must_use]
    pub fn parent_chat(&self) -> Option<&crate::types::Chat> {
        match self {
            Self::Channel(val) => val.parent_chat.as_deref(),
            _ => None,
        }
    }

    /// Helper method for field `permissions`.
    ///
    /// # Variants
    /// - `ChatFullInfoSupergroup`. Default chat member permissions, for groups and supergroups
    #[must_use]
    pub fn permissions(&self) -> Option<&crate::types::ChatPermissions> {
        match self {
            Self::Supergroup(val) => val.permissions.as_ref(),
            _ => None,
        }
    }

    /// Helper method for field `personal_chat`.
    ///
    /// # Variants
    /// - `ChatFullInfoPrivate`. For private chats, the personal channel of the user
    /// - `ChatFullInfoChannel`. For private chats, the personal channel of the user
    #[must_use]
    pub fn personal_chat(&self) -> Option<&crate::types::Chat> {
        match self {
            Self::Private(val) => val.personal_chat.as_deref(),
            Self::Channel(val) => val.personal_chat.as_deref(),
            _ => None,
        }
    }

    /// Helper method for field `photo`.
    ///
    /// # Variants
    /// - `ChatFullInfoPrivate`. Chat photo
    /// - `ChatFullInfoGroup`. Chat photo
    /// - `ChatFullInfoSupergroup`. Chat photo
    /// - `ChatFullInfoChannel`. Chat photo
    #[must_use]
    pub fn photo(&self) -> Option<&crate::types::ChatPhoto> {
        match self {
            Self::Private(val) => val.photo.as_ref(),
            Self::Group(val) => val.photo.as_ref(),
            Self::Supergroup(val) => val.photo.as_ref(),
            Self::Channel(val) => val.photo.as_ref(),
        }
    }

    /// Helper method for field `pinned_message`.
    ///
    /// # Variants
    /// - `ChatFullInfoPrivate`. The most recent pinned message (by sending date)
    /// - `ChatFullInfoGroup`. The most recent pinned message (by sending date)
    /// - `ChatFullInfoSupergroup`. The most recent pinned message (by sending date)
    /// - `ChatFullInfoChannel`. The most recent pinned message (by sending date)
    #[must_use]
    pub fn pinned_message(&self) -> Option<&crate::types::Message> {
        match self {
            Self::Private(val) => val.pinned_message.as_deref(),
            Self::Group(val) => val.pinned_message.as_deref(),
            Self::Supergroup(val) => val.pinned_message.as_deref(),
            Self::Channel(val) => val.pinned_message.as_deref(),
        }
    }

    /// Helper method for field `profile_accent_color_id`.
    ///
    /// # Variants
    /// - `ChatFullInfoPrivate`. Identifier of the accent color for the chat's profile background. See profile accent colors for more details.
    /// - `ChatFullInfoGroup`. Identifier of the accent color for the chat's profile background. See profile accent colors for more details.
    /// - `ChatFullInfoSupergroup`. Identifier of the accent color for the chat's profile background. See profile accent colors for more details.
    /// - `ChatFullInfoChannel`. Identifier of the accent color for the chat's profile background. See profile accent colors for more details.
    #[must_use]
    pub fn profile_accent_color_id(&self) -> Option<i64> {
        match self {
            Self::Private(val) => val.profile_accent_color_id,
            Self::Group(val) => val.profile_accent_color_id,
            Self::Supergroup(val) => val.profile_accent_color_id,
            Self::Channel(val) => val.profile_accent_color_id,
        }
    }

    /// Helper method for field `profile_background_custom_emoji_id`.
    ///
    /// # Variants
    /// - `ChatFullInfoPrivate`. Custom emoji identifier of the emoji chosen by the chat for its profile background
    /// - `ChatFullInfoGroup`. Custom emoji identifier of the emoji chosen by the chat for its profile background
    /// - `ChatFullInfoSupergroup`. Custom emoji identifier of the emoji chosen by the chat for its profile background
    /// - `ChatFullInfoChannel`. Custom emoji identifier of the emoji chosen by the chat for its profile background
    #[must_use]
    pub fn profile_background_custom_emoji_id(&self) -> Option<&str> {
        match self {
            Self::Private(val) => val.profile_background_custom_emoji_id.as_deref(),
            Self::Group(val) => val.profile_background_custom_emoji_id.as_deref(),
            Self::Supergroup(val) => val.profile_background_custom_emoji_id.as_deref(),
            Self::Channel(val) => val.profile_background_custom_emoji_id.as_deref(),
        }
    }

    /// Helper method for field `rating`.
    ///
    /// # Variants
    /// - `ChatFullInfoPrivate`. For private chats, the rating of the user if any
    #[must_use]
    pub fn rating(&self) -> Option<&crate::types::UserRating> {
        match self {
            Self::Private(val) => val.rating.as_ref(),
            _ => None,
        }
    }

    /// Helper method for field `slow_mode_delay`.
    ///
    /// # Variants
    /// - `ChatFullInfoSupergroup`. For supergroups, the minimum allowed delay between consecutive messages sent by each unprivileged user; in seconds
    #[must_use]
    pub fn slow_mode_delay(&self) -> Option<i64> {
        match self {
            Self::Supergroup(val) => val.slow_mode_delay,
            _ => None,
        }
    }

    /// Helper method for field `sticker_set_name`.
    ///
    /// # Variants
    /// - `ChatFullInfoSupergroup`. For supergroups, name of the group sticker set
    #[must_use]
    pub fn sticker_set_name(&self) -> Option<&str> {
        match self {
            Self::Supergroup(val) => val.sticker_set_name.as_deref(),
            _ => None,
        }
    }

    /// Helper method for field `title`.
    ///
    /// # Variants
    /// - `ChatFullInfoSupergroup`. Title, for supergroups, channels and group chats
    /// - `ChatFullInfoChannel`. Title, for supergroups, channels and group chats
    #[must_use]
    pub fn title(&self) -> Option<&str> {
        match self {
            Self::Supergroup(val) => val.title.as_deref(),
            Self::Channel(val) => val.title.as_deref(),
            _ => None,
        }
    }

    /// Helper method for field `unique_gift_colors`.
    ///
    /// # Variants
    /// - `ChatFullInfoPrivate`. The color scheme based on a unique gift that must be used for the chat's name, message replies and link previews
    /// - `ChatFullInfoGroup`. The color scheme based on a unique gift that must be used for the chat's name, message replies and link previews
    /// - `ChatFullInfoSupergroup`. The color scheme based on a unique gift that must be used for the chat's name, message replies and link previews
    /// - `ChatFullInfoChannel`. The color scheme based on a unique gift that must be used for the chat's name, message replies and link previews
    #[must_use]
    pub fn unique_gift_colors(&self) -> Option<&crate::types::UniqueGiftColors> {
        match self {
            Self::Private(val) => val.unique_gift_colors.as_ref(),
            Self::Group(val) => val.unique_gift_colors.as_ref(),
            Self::Supergroup(val) => val.unique_gift_colors.as_ref(),
            Self::Channel(val) => val.unique_gift_colors.as_ref(),
        }
    }

    /// Helper method for field `unrestrict_boost_count`.
    ///
    /// # Variants
    /// - `ChatFullInfoSupergroup`. For supergroups, the minimum number of boosts that a non-administrator user needs to add in order to ignore slow mode and chat permissions
    #[must_use]
    pub fn unrestrict_boost_count(&self) -> Option<i64> {
        match self {
            Self::Supergroup(val) => val.unrestrict_boost_count,
            _ => None,
        }
    }

    /// Helper method for field `username`.
    ///
    /// # Variants
    /// - `ChatFullInfoPrivate`. Username, for private chats, supergroups and channels if available
    /// - `ChatFullInfoSupergroup`. Username, for private chats, supergroups and channels if available
    /// - `ChatFullInfoChannel`. Username, for private chats, supergroups and channels if available
    #[must_use]
    pub fn username(&self) -> Option<&str> {
        match self {
            Self::Private(val) => val.username.as_deref(),
            Self::Supergroup(val) => val.username.as_deref(),
            Self::Channel(val) => val.username.as_deref(),
            Self::Group(_) => None,
        }
    }

    /// Helper method for nested field `address`.
    #[must_use]
    pub fn address(&self) -> Option<&str> {
        match self {
            Self::Private(val) => val
                .business_location
                .as_ref()
                .map(|inner| inner.address.as_ref()),
            Self::Supergroup(val) => val.location.as_ref().map(|inner| inner.address.as_ref()),
            _ => None,
        }
    }

    /// Helper method for nested field `animation`.
    #[must_use]
    pub fn animation(&self) -> Option<&crate::types::Animation> {
        self.pinned_message()
            .and_then(crate::types::Message::animation)
    }

    /// Helper method for nested field `audio`.
    #[must_use]
    pub fn audio(&self) -> Option<&crate::types::Audio> {
        self.pinned_message().and_then(crate::types::Message::audio)
    }

    /// Helper method for nested field `author_signature`.
    #[must_use]
    pub fn author_signature(&self) -> Option<&str> {
        self.pinned_message()
            .and_then(crate::types::Message::author_signature)
    }

    /// Helper method for nested field `big_file_id`.
    #[must_use]
    pub fn big_file_id(&self) -> Option<&str> {
        self.photo().map(|inner| inner.big_file_id.as_ref())
    }

    /// Helper method for nested field `big_file_unique_id`.
    #[must_use]
    pub fn big_file_unique_id(&self) -> Option<&str> {
        self.photo().map(|inner| inner.big_file_unique_id.as_ref())
    }

    /// Helper method for nested field `boost_added`.
    #[must_use]
    pub fn boost_added(&self) -> Option<&crate::types::ChatBoostAdded> {
        self.pinned_message()
            .and_then(crate::types::Message::boost_added)
    }

    /// Helper method for nested field `business_connection_id`.
    #[must_use]
    pub fn business_connection_id(&self) -> Option<&str> {
        self.pinned_message()
            .and_then(crate::types::Message::business_connection_id)
    }

    /// Helper method for nested field `can_add_web_page_previews`.
    #[must_use]
    pub fn can_add_web_page_previews(&self) -> Option<bool> {
        match self {
            Self::Supergroup(val) => val
                .permissions
                .as_ref()
                .and_then(|inner| inner.can_add_web_page_previews),
            _ => None,
        }
    }

    /// Helper method for nested field `can_change_info`.
    #[must_use]
    pub fn can_change_info(&self) -> Option<bool> {
        match self {
            Self::Supergroup(val) => val
                .permissions
                .as_ref()
                .and_then(|inner| inner.can_change_info),
            _ => None,
        }
    }

    /// Helper method for nested field `can_edit_tag`.
    #[must_use]
    pub fn can_edit_tag(&self) -> Option<bool> {
        match self {
            Self::Supergroup(val) => val
                .permissions
                .as_ref()
                .and_then(|inner| inner.can_edit_tag),
            _ => None,
        }
    }

    /// Helper method for nested field `can_invite_users`.
    #[must_use]
    pub fn can_invite_users(&self) -> Option<bool> {
        match self {
            Self::Supergroup(val) => val
                .permissions
                .as_ref()
                .and_then(|inner| inner.can_invite_users),
            _ => None,
        }
    }

    /// Helper method for nested field `can_manage_topics`.
    #[must_use]
    pub fn can_manage_topics(&self) -> Option<bool> {
        match self {
            Self::Supergroup(val) => val
                .permissions
                .as_ref()
                .and_then(|inner| inner.can_manage_topics),
            _ => None,
        }
    }

    /// Helper method for nested field `can_pin_messages`.
    #[must_use]
    pub fn can_pin_messages(&self) -> Option<bool> {
        match self {
            Self::Supergroup(val) => val
                .permissions
                .as_ref()
                .and_then(|inner| inner.can_pin_messages),
            _ => None,
        }
    }

    /// Helper method for nested field `can_send_audios`.
    #[must_use]
    pub fn can_send_audios(&self) -> Option<bool> {
        match self {
            Self::Supergroup(val) => val
                .permissions
                .as_ref()
                .and_then(|inner| inner.can_send_audios),
            _ => None,
        }
    }

    /// Helper method for nested field `can_send_documents`.
    #[must_use]
    pub fn can_send_documents(&self) -> Option<bool> {
        match self {
            Self::Supergroup(val) => val
                .permissions
                .as_ref()
                .and_then(|inner| inner.can_send_documents),
            _ => None,
        }
    }

    /// Helper method for nested field `can_send_messages`.
    #[must_use]
    pub fn can_send_messages(&self) -> Option<bool> {
        match self {
            Self::Supergroup(val) => val
                .permissions
                .as_ref()
                .and_then(|inner| inner.can_send_messages),
            _ => None,
        }
    }

    /// Helper method for nested field `can_send_other_messages`.
    #[must_use]
    pub fn can_send_other_messages(&self) -> Option<bool> {
        match self {
            Self::Supergroup(val) => val
                .permissions
                .as_ref()
                .and_then(|inner| inner.can_send_other_messages),
            _ => None,
        }
    }

    /// Helper method for nested field `can_send_photos`.
    #[must_use]
    pub fn can_send_photos(&self) -> Option<bool> {
        match self {
            Self::Supergroup(val) => val
                .permissions
                .as_ref()
                .and_then(|inner| inner.can_send_photos),
            _ => None,
        }
    }

    /// Helper method for nested field `can_send_polls`.
    #[must_use]
    pub fn can_send_polls(&self) -> Option<bool> {
        match self {
            Self::Supergroup(val) => val
                .permissions
                .as_ref()
                .and_then(|inner| inner.can_send_polls),
            _ => None,
        }
    }

    /// Helper method for nested field `can_send_video_notes`.
    #[must_use]
    pub fn can_send_video_notes(&self) -> Option<bool> {
        match self {
            Self::Supergroup(val) => val
                .permissions
                .as_ref()
                .and_then(|inner| inner.can_send_video_notes),
            _ => None,
        }
    }

    /// Helper method for nested field `can_send_videos`.
    #[must_use]
    pub fn can_send_videos(&self) -> Option<bool> {
        match self {
            Self::Supergroup(val) => val
                .permissions
                .as_ref()
                .and_then(|inner| inner.can_send_videos),
            _ => None,
        }
    }

    /// Helper method for nested field `can_send_voice_notes`.
    #[must_use]
    pub fn can_send_voice_notes(&self) -> Option<bool> {
        match self {
            Self::Supergroup(val) => val
                .permissions
                .as_ref()
                .and_then(|inner| inner.can_send_voice_notes),
            _ => None,
        }
    }

    /// Helper method for nested field `caption`.
    #[must_use]
    pub fn caption(&self) -> Option<&str> {
        self.pinned_message()
            .and_then(crate::types::Message::caption)
    }

    /// Helper method for nested field `caption_entities`.
    #[must_use]
    pub fn caption_entities(&self) -> Option<&[crate::types::MessageEntity]> {
        self.pinned_message()
            .and_then(crate::types::Message::caption_entities)
    }

    /// Helper method for nested field `channel_chat_created`.
    #[must_use]
    pub fn channel_chat_created(&self) -> Option<bool> {
        self.pinned_message()
            .and_then(crate::types::Message::channel_chat_created)
    }

    /// Helper method for nested field `chat`.
    #[must_use]
    pub fn chat(&self) -> Option<&crate::types::Chat> {
        self.pinned_message().map(crate::types::Message::chat)
    }

    /// Helper method for nested field `chat_background_set`.
    #[must_use]
    pub fn chat_background_set(&self) -> Option<&crate::types::ChatBackground> {
        self.pinned_message()
            .and_then(crate::types::Message::chat_background_set)
    }

    /// Helper method for nested field `chat_owner_changed`.
    #[must_use]
    pub fn chat_owner_changed(&self) -> Option<&crate::types::ChatOwnerChanged> {
        self.pinned_message()
            .and_then(crate::types::Message::chat_owner_changed)
    }

    /// Helper method for nested field `chat_owner_left`.
    #[must_use]
    pub fn chat_owner_left(&self) -> Option<&crate::types::ChatOwnerLeft> {
        self.pinned_message()
            .and_then(crate::types::Message::chat_owner_left)
    }

    /// Helper method for nested field `chat_shared`.
    #[must_use]
    pub fn chat_shared(&self) -> Option<&crate::types::ChatShared> {
        self.pinned_message()
            .and_then(crate::types::Message::chat_shared)
    }

    /// Helper method for nested field `checklist`.
    #[must_use]
    pub fn checklist(&self) -> Option<&crate::types::Checklist> {
        self.pinned_message()
            .and_then(crate::types::Message::checklist)
    }

    /// Helper method for nested field `checklist_tasks_added`.
    #[must_use]
    pub fn checklist_tasks_added(&self) -> Option<&crate::types::ChecklistTasksAdded> {
        self.pinned_message()
            .and_then(crate::types::Message::checklist_tasks_added)
    }

    /// Helper method for nested field `checklist_tasks_done`.
    #[must_use]
    pub fn checklist_tasks_done(&self) -> Option<&crate::types::ChecklistTasksDone> {
        self.pinned_message()
            .and_then(crate::types::Message::checklist_tasks_done)
    }

    /// Helper method for nested field `connected_website`.
    #[must_use]
    pub fn connected_website(&self) -> Option<&str> {
        self.pinned_message()
            .and_then(crate::types::Message::connected_website)
    }

    /// Helper method for nested field `contact`.
    #[must_use]
    pub fn contact(&self) -> Option<&crate::types::Contact> {
        self.pinned_message()
            .and_then(crate::types::Message::contact)
    }

    /// Helper method for nested field `current_level_rating`.
    #[must_use]
    pub fn current_level_rating(&self) -> Option<i64> {
        match self {
            Self::Private(val) => val.rating.as_ref().map(|inner| inner.current_level_rating),
            _ => None,
        }
    }

    /// Helper method for nested field `dark_theme_main_color`.
    #[must_use]
    pub fn dark_theme_main_color(&self) -> Option<i64> {
        self.unique_gift_colors()
            .map(|inner| inner.dark_theme_main_color)
    }

    /// Helper method for nested field `dark_theme_other_colors`.
    #[must_use]
    pub fn dark_theme_other_colors(&self) -> Option<&[u8]> {
        self.unique_gift_colors()
            .map(|inner| inner.dark_theme_other_colors.as_ref())
    }

    /// Helper method for nested field `date`.
    #[must_use]
    pub fn date(&self) -> Option<i64> {
        self.pinned_message().map(crate::types::Message::date)
    }

    /// Helper method for nested field `day`.
    #[must_use]
    pub fn day(&self) -> Option<u8> {
        match self {
            Self::Private(val) => val.birthdate.as_ref().map(|inner| inner.day),
            _ => None,
        }
    }

    /// Helper method for nested field `delete_chat_photo`.
    #[must_use]
    pub fn delete_chat_photo(&self) -> Option<bool> {
        self.pinned_message()
            .and_then(crate::types::Message::delete_chat_photo)
    }

    /// Helper method for nested field `dice`.
    #[must_use]
    pub fn dice(&self) -> Option<&crate::types::Dice> {
        self.pinned_message().and_then(crate::types::Message::dice)
    }

    /// Helper method for nested field `direct_message_price_changed`.
    #[must_use]
    pub fn direct_message_price_changed(&self) -> Option<&crate::types::DirectMessagePriceChanged> {
        self.pinned_message()
            .and_then(crate::types::Message::direct_message_price_changed)
    }

    /// Helper method for nested field `direct_messages_topic`.
    #[must_use]
    pub fn direct_messages_topic(&self) -> Option<&crate::types::DirectMessagesTopic> {
        self.pinned_message()
            .and_then(crate::types::Message::direct_messages_topic)
    }

    /// Helper method for nested field `document`.
    #[must_use]
    pub fn document(&self) -> Option<&crate::types::Document> {
        self.pinned_message()
            .and_then(crate::types::Message::document)
    }

    /// Helper method for nested field `duration`.
    #[must_use]
    pub fn duration(&self) -> Option<i64> {
        match self {
            Self::Private(val) => val
                .first_profile_audio
                .as_deref()
                .map(|inner| inner.duration),
            _ => None,
        }
    }

    /// Helper method for nested field `edit_date`.
    #[must_use]
    pub fn edit_date(&self) -> Option<i64> {
        self.pinned_message()
            .and_then(crate::types::Message::edit_date)
    }

    /// Helper method for nested field `effect_id`.
    #[must_use]
    pub fn effect_id(&self) -> Option<&str> {
        self.pinned_message()
            .and_then(crate::types::Message::effect_id)
    }

    /// Helper method for nested field `entities`.
    #[must_use]
    pub fn entities(&self) -> Option<&[crate::types::MessageEntity]> {
        self.pinned_message()
            .and_then(crate::types::Message::entities)
    }

    /// Helper method for nested field `external_reply`.
    #[must_use]
    pub fn external_reply(&self) -> Option<&crate::types::ExternalReplyInfo> {
        self.pinned_message()
            .and_then(crate::types::Message::external_reply)
    }

    /// Helper method for nested field `file_id`.
    #[must_use]
    pub fn file_id(&self) -> Option<&str> {
        match self {
            Self::Private(val) => val
                .first_profile_audio
                .as_deref()
                .map(|inner| inner.file_id.as_ref()),
            _ => None,
        }
    }

    /// Helper method for nested field `file_name`.
    #[must_use]
    pub fn file_name(&self) -> Option<&str> {
        match self {
            Self::Private(val) => val
                .first_profile_audio
                .as_deref()
                .and_then(|inner| inner.file_name.as_deref()),
            _ => None,
        }
    }

    /// Helper method for nested field `file_size`.
    #[must_use]
    pub fn file_size(&self) -> Option<i64> {
        match self {
            Self::Private(val) => val
                .first_profile_audio
                .as_deref()
                .and_then(|inner| inner.file_size),
            _ => None,
        }
    }

    /// Helper method for nested field `file_unique_id`.
    #[must_use]
    pub fn file_unique_id(&self) -> Option<&str> {
        match self {
            Self::Private(val) => val
                .first_profile_audio
                .as_deref()
                .map(|inner| inner.file_unique_id.as_ref()),
            _ => None,
        }
    }

    /// Helper method for nested field `forum_topic_closed`.
    #[must_use]
    pub fn forum_topic_closed(&self) -> Option<&crate::types::ForumTopicClosed> {
        self.pinned_message()
            .and_then(crate::types::Message::forum_topic_closed)
    }

    /// Helper method for nested field `forum_topic_created`.
    #[must_use]
    pub fn forum_topic_created(&self) -> Option<&crate::types::ForumTopicCreated> {
        self.pinned_message()
            .and_then(crate::types::Message::forum_topic_created)
    }

    /// Helper method for nested field `forum_topic_edited`.
    #[must_use]
    pub fn forum_topic_edited(&self) -> Option<&crate::types::ForumTopicEdited> {
        self.pinned_message()
            .and_then(crate::types::Message::forum_topic_edited)
    }

    /// Helper method for nested field `forum_topic_reopened`.
    #[must_use]
    pub fn forum_topic_reopened(&self) -> Option<&crate::types::ForumTopicReopened> {
        self.pinned_message()
            .and_then(crate::types::Message::forum_topic_reopened)
    }

    /// Helper method for nested field `forward_origin`.
    #[must_use]
    pub fn forward_origin(&self) -> Option<&crate::types::MessageOrigin> {
        self.pinned_message()
            .and_then(crate::types::Message::forward_origin)
    }

    /// Helper method for nested field `from`.
    #[must_use]
    pub fn from(&self) -> Option<&crate::types::User> {
        self.pinned_message().and_then(crate::types::Message::from)
    }

    /// Helper method for nested field `game`.
    #[must_use]
    pub fn game(&self) -> Option<&crate::types::Game> {
        self.pinned_message().and_then(crate::types::Message::game)
    }

    /// Helper method for nested field `general_forum_topic_hidden`.
    #[must_use]
    pub fn general_forum_topic_hidden(&self) -> Option<&crate::types::GeneralForumTopicHidden> {
        self.pinned_message()
            .and_then(crate::types::Message::general_forum_topic_hidden)
    }

    /// Helper method for nested field `general_forum_topic_unhidden`.
    #[must_use]
    pub fn general_forum_topic_unhidden(&self) -> Option<&crate::types::GeneralForumTopicUnhidden> {
        self.pinned_message()
            .and_then(crate::types::Message::general_forum_topic_unhidden)
    }

    /// Helper method for nested field `gift`.
    #[must_use]
    pub fn gift(&self) -> Option<&crate::types::GiftInfo> {
        self.pinned_message().and_then(crate::types::Message::gift)
    }

    /// Helper method for nested field `gift_upgrade_sent`.
    #[must_use]
    pub fn gift_upgrade_sent(&self) -> Option<&crate::types::GiftInfo> {
        self.pinned_message()
            .and_then(crate::types::Message::gift_upgrade_sent)
    }

    /// Helper method for nested field `gifts_from_channels`.
    #[must_use]
    pub fn gifts_from_channels(&self) -> Option<bool> {
        match self {
            Self::Private(val) => {
                let inner = &val.accepted_gift_types;
                Some(inner.gifts_from_channels)
            }
            _ => None,
        }
    }

    /// Helper method for nested field `giveaway`.
    #[must_use]
    pub fn giveaway(&self) -> Option<&crate::types::Giveaway> {
        self.pinned_message()
            .and_then(crate::types::Message::giveaway)
    }

    /// Helper method for nested field `giveaway_completed`.
    #[must_use]
    pub fn giveaway_completed(&self) -> Option<&crate::types::GiveawayCompleted> {
        self.pinned_message()
            .and_then(crate::types::Message::giveaway_completed)
    }

    /// Helper method for nested field `giveaway_created`.
    #[must_use]
    pub fn giveaway_created(&self) -> Option<&crate::types::GiveawayCreated> {
        self.pinned_message()
            .and_then(crate::types::Message::giveaway_created)
    }

    /// Helper method for nested field `giveaway_winners`.
    #[must_use]
    pub fn giveaway_winners(&self) -> Option<&crate::types::GiveawayWinners> {
        self.pinned_message()
            .and_then(crate::types::Message::giveaway_winners)
    }

    /// Helper method for nested field `group_chat_created`.
    #[must_use]
    pub fn group_chat_created(&self) -> Option<bool> {
        self.pinned_message()
            .and_then(crate::types::Message::group_chat_created)
    }

    /// Helper method for nested field `has_media_spoiler`.
    #[must_use]
    pub fn has_media_spoiler(&self) -> Option<bool> {
        self.pinned_message()
            .and_then(crate::types::Message::has_media_spoiler)
    }

    /// Helper method for nested field `invoice`.
    #[must_use]
    pub fn invoice(&self) -> Option<&crate::types::Invoice> {
        self.pinned_message()
            .and_then(crate::types::Message::invoice)
    }

    /// Helper method for nested field `is_automatic_forward`.
    #[must_use]
    pub fn is_automatic_forward(&self) -> Option<bool> {
        self.pinned_message()
            .and_then(crate::types::Message::is_automatic_forward)
    }

    /// Helper method for nested field `is_from_offline`.
    #[must_use]
    pub fn is_from_offline(&self) -> Option<bool> {
        self.pinned_message()
            .and_then(crate::types::Message::is_from_offline)
    }

    /// Helper method for nested field `is_paid_post`.
    #[must_use]
    pub fn is_paid_post(&self) -> Option<bool> {
        self.pinned_message()
            .and_then(crate::types::Message::is_paid_post)
    }

    /// Helper method for nested field `is_topic_message`.
    #[must_use]
    pub fn is_topic_message(&self) -> Option<bool> {
        self.pinned_message()
            .and_then(crate::types::Message::is_topic_message)
    }

    /// Helper method for nested field `left_chat_member`.
    #[must_use]
    pub fn left_chat_member(&self) -> Option<&crate::types::User> {
        self.pinned_message()
            .and_then(crate::types::Message::left_chat_member)
    }

    /// Helper method for nested field `level`.
    #[must_use]
    pub fn level(&self) -> Option<i64> {
        match self {
            Self::Private(val) => val.rating.as_ref().map(|inner| inner.level),
            _ => None,
        }
    }

    /// Helper method for nested field `light_theme_main_color`.
    #[must_use]
    pub fn light_theme_main_color(&self) -> Option<i64> {
        self.unique_gift_colors()
            .map(|inner| inner.light_theme_main_color)
    }

    /// Helper method for nested field `light_theme_other_colors`.
    #[must_use]
    pub fn light_theme_other_colors(&self) -> Option<&[u8]> {
        self.unique_gift_colors()
            .map(|inner| inner.light_theme_other_colors.as_ref())
    }

    /// Helper method for nested field `limited_gifts`.
    #[must_use]
    pub fn limited_gifts(&self) -> Option<bool> {
        match self {
            Self::Private(val) => {
                let inner = &val.accepted_gift_types;
                Some(inner.limited_gifts)
            }
            _ => None,
        }
    }

    /// Helper method for nested field `link_preview_options`.
    #[must_use]
    pub fn link_preview_options(&self) -> Option<&crate::types::LinkPreviewOptions> {
        self.pinned_message()
            .and_then(crate::types::Message::link_preview_options)
    }

    /// Helper method for nested field `media_group_id`.
    #[must_use]
    pub fn media_group_id(&self) -> Option<&str> {
        self.pinned_message()
            .and_then(crate::types::Message::media_group_id)
    }

    /// Helper method for nested field `message`.
    #[must_use]
    pub fn message(&self) -> Option<&str> {
        match self {
            Self::Private(val) => val
                .business_intro
                .as_ref()
                .and_then(|inner| inner.message.as_deref()),
            _ => None,
        }
    }

    /// Helper method for nested field `message_auto_delete_timer_changed`.
    #[must_use]
    pub fn message_auto_delete_timer_changed(
        &self,
    ) -> Option<&crate::types::MessageAutoDeleteTimerChanged> {
        self.pinned_message()
            .and_then(crate::types::Message::message_auto_delete_timer_changed)
    }

    /// Helper method for nested field `message_id`.
    #[must_use]
    pub fn message_id(&self) -> Option<i64> {
        self.pinned_message().map(crate::types::Message::message_id)
    }

    /// Helper method for nested field `message_thread_id`.
    #[must_use]
    pub fn message_thread_id(&self) -> Option<i64> {
        self.pinned_message()
            .and_then(crate::types::Message::message_thread_id)
    }

    /// Helper method for nested field `migrate_from_chat_id`.
    #[must_use]
    pub fn migrate_from_chat_id(&self) -> Option<i64> {
        self.pinned_message()
            .and_then(crate::types::Message::migrate_from_chat_id)
    }

    /// Helper method for nested field `migrate_to_chat_id`.
    #[must_use]
    pub fn migrate_to_chat_id(&self) -> Option<i64> {
        self.pinned_message()
            .and_then(crate::types::Message::migrate_to_chat_id)
    }

    /// Helper method for nested field `mime_type`.
    #[must_use]
    pub fn mime_type(&self) -> Option<&str> {
        match self {
            Self::Private(val) => val
                .first_profile_audio
                .as_deref()
                .and_then(|inner| inner.mime_type.as_deref()),
            _ => None,
        }
    }

    /// Helper method for nested field `model_custom_emoji_id`.
    #[must_use]
    pub fn model_custom_emoji_id(&self) -> Option<&str> {
        self.unique_gift_colors()
            .map(|inner| inner.model_custom_emoji_id.as_ref())
    }

    /// Helper method for nested field `month`.
    #[must_use]
    pub fn month(&self) -> Option<u8> {
        match self {
            Self::Private(val) => val.birthdate.as_ref().map(|inner| inner.month),
            _ => None,
        }
    }

    /// Helper method for nested field `new_chat_members`.
    #[must_use]
    pub fn new_chat_members(&self) -> Option<&[crate::types::User]> {
        self.pinned_message()
            .and_then(crate::types::Message::new_chat_members)
    }

    /// Helper method for nested field `new_chat_photo`.
    #[must_use]
    pub fn new_chat_photo(&self) -> Option<&[crate::types::PhotoSize]> {
        self.pinned_message()
            .and_then(crate::types::Message::new_chat_photo)
    }

    /// Helper method for nested field `new_chat_title`.
    #[must_use]
    pub fn new_chat_title(&self) -> Option<&str> {
        self.pinned_message()
            .and_then(crate::types::Message::new_chat_title)
    }

    /// Helper method for nested field `next_level_rating`.
    #[must_use]
    pub fn next_level_rating(&self) -> Option<i64> {
        match self {
            Self::Private(val) => val
                .rating
                .as_ref()
                .and_then(|inner| inner.next_level_rating),
            _ => None,
        }
    }

    /// Helper method for nested field `opening_hours`.
    #[must_use]
    pub fn opening_hours(&self) -> Option<&[crate::types::BusinessOpeningHoursInterval]> {
        match self {
            Self::Private(val) => val
                .business_opening_hours
                .as_ref()
                .map(|inner| inner.opening_hours.as_ref()),
            _ => None,
        }
    }

    /// Helper method for nested field `paid_media`.
    #[must_use]
    pub fn paid_media(&self) -> Option<&crate::types::PaidMediaInfo> {
        self.pinned_message()
            .and_then(crate::types::Message::paid_media)
    }

    /// Helper method for nested field `paid_message_price_changed`.
    #[must_use]
    pub fn paid_message_price_changed(&self) -> Option<&crate::types::PaidMessagePriceChanged> {
        self.pinned_message()
            .and_then(crate::types::Message::paid_message_price_changed)
    }

    /// Helper method for nested field `paid_star_count`.
    #[must_use]
    pub fn paid_star_count(&self) -> Option<i64> {
        self.pinned_message()
            .and_then(crate::types::Message::paid_star_count)
    }

    /// Helper method for nested field `passport_data`.
    #[must_use]
    pub fn passport_data(&self) -> Option<&crate::types::PassportData> {
        self.pinned_message()
            .and_then(crate::types::Message::passport_data)
    }

    /// Helper method for nested field `performer`.
    #[must_use]
    pub fn performer(&self) -> Option<&str> {
        match self {
            Self::Private(val) => val
                .first_profile_audio
                .as_deref()
                .and_then(|inner| inner.performer.as_deref()),
            _ => None,
        }
    }

    /// Helper method for nested field `poll`.
    #[must_use]
    pub fn poll(&self) -> Option<&crate::types::Poll> {
        self.pinned_message().and_then(crate::types::Message::poll)
    }

    /// Helper method for nested field `premium_subscription`.
    #[must_use]
    pub fn premium_subscription(&self) -> Option<bool> {
        match self {
            Self::Private(val) => {
                let inner = &val.accepted_gift_types;
                Some(inner.premium_subscription)
            }
            _ => None,
        }
    }

    /// Helper method for nested field `proximity_alert_triggered`.
    #[must_use]
    pub fn proximity_alert_triggered(&self) -> Option<&crate::types::ProximityAlertTriggered> {
        self.pinned_message()
            .and_then(crate::types::Message::proximity_alert_triggered)
    }

    /// Helper method for nested field `quote`.
    #[must_use]
    pub fn quote(&self) -> Option<&crate::types::TextQuote> {
        self.pinned_message().and_then(crate::types::Message::quote)
    }

    /// Helper method for nested field `refunded_payment`.
    #[must_use]
    pub fn refunded_payment(&self) -> Option<&crate::types::RefundedPayment> {
        self.pinned_message()
            .and_then(crate::types::Message::refunded_payment)
    }

    /// Helper method for nested field `reply_markup`.
    #[must_use]
    pub fn reply_markup(&self) -> Option<&crate::types::InlineKeyboardMarkup> {
        self.pinned_message()
            .and_then(crate::types::Message::reply_markup)
    }

    /// Helper method for nested field `reply_to_checklist_task_id`.
    #[must_use]
    pub fn reply_to_checklist_task_id(&self) -> Option<i64> {
        self.pinned_message()
            .and_then(crate::types::Message::reply_to_checklist_task_id)
    }

    /// Helper method for nested field `reply_to_message`.
    #[must_use]
    pub fn reply_to_message(&self) -> Option<&crate::types::Message> {
        self.pinned_message()
            .and_then(crate::types::Message::reply_to_message)
    }

    /// Helper method for nested field `reply_to_story`.
    #[must_use]
    pub fn reply_to_story(&self) -> Option<&crate::types::Story> {
        self.pinned_message()
            .and_then(crate::types::Message::reply_to_story)
    }

    /// Helper method for nested field `sender_boost_count`.
    #[must_use]
    pub fn sender_boost_count(&self) -> Option<i64> {
        self.pinned_message()
            .and_then(crate::types::Message::sender_boost_count)
    }

    /// Helper method for nested field `sender_business_bot`.
    #[must_use]
    pub fn sender_business_bot(&self) -> Option<&crate::types::User> {
        self.pinned_message()
            .and_then(crate::types::Message::sender_business_bot)
    }

    /// Helper method for nested field `sender_chat`.
    #[must_use]
    pub fn sender_chat(&self) -> Option<&crate::types::Chat> {
        self.pinned_message()
            .and_then(crate::types::Message::sender_chat)
    }

    /// Helper method for nested field `sender_tag`.
    #[must_use]
    pub fn sender_tag(&self) -> Option<&str> {
        self.pinned_message()
            .and_then(crate::types::Message::sender_tag)
    }

    /// Helper method for nested field `show_caption_above_media`.
    #[must_use]
    pub fn show_caption_above_media(&self) -> Option<bool> {
        self.pinned_message()
            .and_then(crate::types::Message::show_caption_above_media)
    }

    /// Helper method for nested field `small_file_id`.
    #[must_use]
    pub fn small_file_id(&self) -> Option<&str> {
        self.photo().map(|inner| inner.small_file_id.as_ref())
    }

    /// Helper method for nested field `small_file_unique_id`.
    #[must_use]
    pub fn small_file_unique_id(&self) -> Option<&str> {
        self.photo()
            .map(|inner| inner.small_file_unique_id.as_ref())
    }

    /// Helper method for nested field `story`.
    #[must_use]
    pub fn story(&self) -> Option<&crate::types::Story> {
        self.pinned_message().and_then(crate::types::Message::story)
    }

    /// Helper method for nested field `successful_payment`.
    #[must_use]
    pub fn successful_payment(&self) -> Option<&crate::types::SuccessfulPayment> {
        self.pinned_message()
            .and_then(crate::types::Message::successful_payment)
    }

    /// Helper method for nested field `suggested_post_approval_failed`.
    #[must_use]
    pub fn suggested_post_approval_failed(
        &self,
    ) -> Option<&crate::types::SuggestedPostApprovalFailed> {
        self.pinned_message()
            .and_then(crate::types::Message::suggested_post_approval_failed)
    }

    /// Helper method for nested field `suggested_post_approved`.
    #[must_use]
    pub fn suggested_post_approved(&self) -> Option<&crate::types::SuggestedPostApproved> {
        self.pinned_message()
            .and_then(crate::types::Message::suggested_post_approved)
    }

    /// Helper method for nested field `suggested_post_declined`.
    #[must_use]
    pub fn suggested_post_declined(&self) -> Option<&crate::types::SuggestedPostDeclined> {
        self.pinned_message()
            .and_then(crate::types::Message::suggested_post_declined)
    }

    /// Helper method for nested field `suggested_post_info`.
    #[must_use]
    pub fn suggested_post_info(&self) -> Option<&crate::types::SuggestedPostInfo> {
        self.pinned_message()
            .and_then(crate::types::Message::suggested_post_info)
    }

    /// Helper method for nested field `suggested_post_paid`.
    #[must_use]
    pub fn suggested_post_paid(&self) -> Option<&crate::types::SuggestedPostPaid> {
        self.pinned_message()
            .and_then(crate::types::Message::suggested_post_paid)
    }

    /// Helper method for nested field `suggested_post_refunded`.
    #[must_use]
    pub fn suggested_post_refunded(&self) -> Option<&crate::types::SuggestedPostRefunded> {
        self.pinned_message()
            .and_then(crate::types::Message::suggested_post_refunded)
    }

    /// Helper method for nested field `supergroup_chat_created`.
    #[must_use]
    pub fn supergroup_chat_created(&self) -> Option<bool> {
        self.pinned_message()
            .and_then(crate::types::Message::supergroup_chat_created)
    }

    /// Helper method for nested field `symbol_custom_emoji_id`.
    #[must_use]
    pub fn symbol_custom_emoji_id(&self) -> Option<&str> {
        self.unique_gift_colors()
            .map(|inner| inner.symbol_custom_emoji_id.as_ref())
    }

    /// Helper method for nested field `text`.
    #[must_use]
    pub fn text(&self) -> Option<&str> {
        self.pinned_message().and_then(crate::types::Message::text)
    }

    /// Helper method for nested field `thumbnail`.
    #[must_use]
    pub fn thumbnail(&self) -> Option<&crate::types::PhotoSize> {
        match self {
            Self::Private(val) => val
                .first_profile_audio
                .as_deref()
                .and_then(|inner| inner.thumbnail.as_ref()),
            _ => None,
        }
    }

    /// Helper method for nested field `time_zone_name`.
    #[must_use]
    pub fn time_zone_name(&self) -> Option<&str> {
        match self {
            Self::Private(val) => val
                .business_opening_hours
                .as_ref()
                .map(|inner| inner.time_zone_name.as_ref()),
            _ => None,
        }
    }

    /// Helper method for nested field `unique_gift`.
    #[must_use]
    pub fn unique_gift(&self) -> Option<&crate::types::UniqueGiftInfo> {
        self.pinned_message()
            .and_then(crate::types::Message::unique_gift)
    }

    /// Helper method for nested field `unique_gifts`.
    #[must_use]
    pub fn unique_gifts(&self) -> Option<bool> {
        match self {
            Self::Private(val) => {
                let inner = &val.accepted_gift_types;
                Some(inner.unique_gifts)
            }
            _ => None,
        }
    }

    /// Helper method for nested field `unlimited_gifts`.
    #[must_use]
    pub fn unlimited_gifts(&self) -> Option<bool> {
        match self {
            Self::Private(val) => {
                let inner = &val.accepted_gift_types;
                Some(inner.unlimited_gifts)
            }
            _ => None,
        }
    }

    /// Helper method for nested field `users_shared`.
    #[must_use]
    pub fn users_shared(&self) -> Option<&crate::types::UsersShared> {
        self.pinned_message()
            .and_then(crate::types::Message::users_shared)
    }

    /// Helper method for nested field `venue`.
    #[must_use]
    pub fn venue(&self) -> Option<&crate::types::Venue> {
        self.pinned_message().and_then(crate::types::Message::venue)
    }

    /// Helper method for nested field `via_bot`.
    #[must_use]
    pub fn via_bot(&self) -> Option<&crate::types::User> {
        self.pinned_message()
            .and_then(crate::types::Message::via_bot)
    }

    /// Helper method for nested field `video`.
    #[must_use]
    pub fn video(&self) -> Option<&crate::types::Video> {
        self.pinned_message().and_then(crate::types::Message::video)
    }

    /// Helper method for nested field `video_chat_ended`.
    #[must_use]
    pub fn video_chat_ended(&self) -> Option<&crate::types::VideoChatEnded> {
        self.pinned_message()
            .and_then(crate::types::Message::video_chat_ended)
    }

    /// Helper method for nested field `video_chat_participants_invited`.
    #[must_use]
    pub fn video_chat_participants_invited(
        &self,
    ) -> Option<&crate::types::VideoChatParticipantsInvited> {
        self.pinned_message()
            .and_then(crate::types::Message::video_chat_participants_invited)
    }

    /// Helper method for nested field `video_chat_scheduled`.
    #[must_use]
    pub fn video_chat_scheduled(&self) -> Option<&crate::types::VideoChatScheduled> {
        self.pinned_message()
            .and_then(crate::types::Message::video_chat_scheduled)
    }

    /// Helper method for nested field `video_chat_started`.
    #[must_use]
    pub fn video_chat_started(&self) -> Option<&crate::types::VideoChatStarted> {
        self.pinned_message()
            .and_then(crate::types::Message::video_chat_started)
    }

    /// Helper method for nested field `video_note`.
    #[must_use]
    pub fn video_note(&self) -> Option<&crate::types::VideoNote> {
        self.pinned_message()
            .and_then(crate::types::Message::video_note)
    }

    /// Helper method for nested field `voice`.
    #[must_use]
    pub fn voice(&self) -> Option<&crate::types::Voice> {
        self.pinned_message().and_then(crate::types::Message::voice)
    }

    /// Helper method for nested field `web_app_data`.
    #[must_use]
    pub fn web_app_data(&self) -> Option<&crate::types::WebAppData> {
        self.pinned_message()
            .and_then(crate::types::Message::web_app_data)
    }

    /// Helper method for nested field `write_access_allowed`.
    #[must_use]
    pub fn write_access_allowed(&self) -> Option<&crate::types::WriteAccessAllowed> {
        self.pinned_message()
            .and_then(crate::types::Message::write_access_allowed)
    }

    /// Helper method for nested field `year`.
    #[must_use]
    pub fn year(&self) -> Option<i64> {
        match self {
            Self::Private(val) => val.birthdate.as_ref().and_then(|inner| inner.year),
            _ => None,
        }
    }
}
impl From<crate::types::ChatFullInfoPrivate> for ChatFullInfo {
    fn from(val: crate::types::ChatFullInfoPrivate) -> Self {
        Self::Private(val)
    }
}
impl TryFrom<ChatFullInfo> for crate::types::ChatFullInfoPrivate {
    type Error = crate::errors::ConvertToTypeError;

    fn try_from(val: ChatFullInfo) -> Result<Self, Self::Error> {
        if let ChatFullInfo::Private(inner) = val {
            Ok(inner)
        } else {
            Err(Self::Error::new(
                stringify!(ChatFullInfo),
                stringify!(ChatFullInfoPrivate),
            ))
        }
    }
}
impl From<crate::types::ChatFullInfoGroup> for ChatFullInfo {
    fn from(val: crate::types::ChatFullInfoGroup) -> Self {
        Self::Group(val)
    }
}
impl TryFrom<ChatFullInfo> for crate::types::ChatFullInfoGroup {
    type Error = crate::errors::ConvertToTypeError;

    fn try_from(val: ChatFullInfo) -> Result<Self, Self::Error> {
        if let ChatFullInfo::Group(inner) = val {
            Ok(inner)
        } else {
            Err(Self::Error::new(
                stringify!(ChatFullInfo),
                stringify!(ChatFullInfoGroup),
            ))
        }
    }
}
impl From<crate::types::ChatFullInfoSupergroup> for ChatFullInfo {
    fn from(val: crate::types::ChatFullInfoSupergroup) -> Self {
        Self::Supergroup(val)
    }
}
impl TryFrom<ChatFullInfo> for crate::types::ChatFullInfoSupergroup {
    type Error = crate::errors::ConvertToTypeError;

    fn try_from(val: ChatFullInfo) -> Result<Self, Self::Error> {
        if let ChatFullInfo::Supergroup(inner) = val {
            Ok(inner)
        } else {
            Err(Self::Error::new(
                stringify!(ChatFullInfo),
                stringify!(ChatFullInfoSupergroup),
            ))
        }
    }
}
impl From<crate::types::ChatFullInfoChannel> for ChatFullInfo {
    fn from(val: crate::types::ChatFullInfoChannel) -> Self {
        Self::Channel(val)
    }
}
impl TryFrom<ChatFullInfo> for crate::types::ChatFullInfoChannel {
    type Error = crate::errors::ConvertToTypeError;

    fn try_from(val: ChatFullInfo) -> Result<Self, Self::Error> {
        if let ChatFullInfo::Channel(inner) = val {
            Ok(inner)
        } else {
            Err(Self::Error::new(
                stringify!(ChatFullInfo),
                stringify!(ChatFullInfoChannel),
            ))
        }
    }
}
