use serde::{Deserialize, Serialize};
/// This object contains full information about a chat.
/// Currently, it can be one of
/// - [`crate::types::ChatFullInfoChannel`]
/// - [`crate::types::ChatFullInfoGroup`]
/// - [`crate::types::ChatFullInfoPrivate`]
/// - [`crate::types::ChatFullInfoSupergroup`]
/// # Documentation
/// <https://core.telegram.org/bots/api#chatfullinfo>
#[derive(Clone, Debug, Serialize, Deserialize)]
#[serde(tag = "type", rename_all = "snake_case")]
pub enum ChatFullInfo {
    Private(crate::types::ChatFullInfoPrivate),
    Group(crate::types::ChatFullInfoGroup),
    Supergroup(crate::types::ChatFullInfoSupergroup),
    Channel(crate::types::ChatFullInfoChannel),
    /// Content unknown to this version of the library
    #[serde(untagged)]
    Unknown(crate::types::ChatFullInfoUnknown),
}
impl ChatFullInfo {
    /// Helper method for field `accent_color_id`.
    ///
    /// Identifier of the accent color for the chat name and backgrounds of the chat photo, reply header, and link preview. See accent colors for more details.
    #[must_use]
    pub fn accent_color_id(&self) -> i64 {
        match self {
            Self::Private(val) => val.accent_color_id,
            Self::Group(val) => val.accent_color_id,
            Self::Supergroup(val) => val.accent_color_id,
            Self::Channel(val) => val.accent_color_id,
            Self::Unknown(val) => val.accent_color_id,
        }
    }

    /// Helper method for field `accepted_gift_types`.
    ///
    /// Information about types of gifts that are accepted by the chat or by the corresponding user for private chats
    #[must_use]
    pub fn accepted_gift_types(&self) -> &crate::types::AcceptedGiftTypes {
        match self {
            Self::Private(val) => &val.accepted_gift_types,
            Self::Group(val) => &val.accepted_gift_types,
            Self::Supergroup(val) => &val.accepted_gift_types,
            Self::Channel(val) => &val.accepted_gift_types,
            Self::Unknown(val) => &val.accepted_gift_types,
        }
    }

    /// Helper method for field `active_usernames`.
    ///
    /// If non-empty, the list of all active chat usernames; for private chats, supergroups and channels
    #[must_use]
    pub fn active_usernames(&self) -> Option<&[Box<str>]> {
        match self {
            Self::Private(val) => val.active_usernames.as_deref(),
            Self::Supergroup(val) => val.active_usernames.as_deref(),
            Self::Channel(val) => val.active_usernames.as_deref(),
            _ => None,
        }
    }

    /// Helper method for field `available_reactions`.
    ///
    /// List of available reactions allowed in the chat. If omitted, then all emoji reactions are allowed.
    #[must_use]
    pub fn available_reactions(&self) -> Option<&[crate::types::ReactionType]> {
        match self {
            Self::Private(val) => val.available_reactions.as_deref(),
            Self::Group(val) => val.available_reactions.as_deref(),
            Self::Supergroup(val) => val.available_reactions.as_deref(),
            Self::Channel(val) => val.available_reactions.as_deref(),
            Self::Unknown(_) => None,
        }
    }

    /// Helper method for field `background_custom_emoji_id`.
    ///
    /// Custom emoji identifier of the emoji chosen by the chat for the reply header and link preview background
    #[must_use]
    pub fn background_custom_emoji_id(&self) -> Option<&str> {
        match self {
            Self::Private(val) => val.background_custom_emoji_id.as_deref(),
            Self::Group(val) => val.background_custom_emoji_id.as_deref(),
            Self::Supergroup(val) => val.background_custom_emoji_id.as_deref(),
            Self::Channel(val) => val.background_custom_emoji_id.as_deref(),
            Self::Unknown(_) => None,
        }
    }

    /// Helper method for field `bio`.
    ///
    /// Bio of the other party in a private chat
    #[must_use]
    pub fn bio(&self) -> Option<&str> {
        match self {
            Self::Private(val) => val.bio.as_deref(),
            _ => None,
        }
    }

    /// Helper method for field `birthdate`.
    ///
    /// For private chats, the date of birth of the user
    #[must_use]
    pub fn birthdate(&self) -> Option<&crate::types::Birthdate> {
        match self {
            Self::Private(val) => val.birthdate.as_ref(),
            _ => None,
        }
    }

    /// Helper method for field `business_intro`.
    ///
    /// For private chats with business accounts, the intro of the business
    #[must_use]
    pub fn business_intro(&self) -> Option<&crate::types::BusinessIntro> {
        match self {
            Self::Private(val) => val.business_intro.as_ref(),
            _ => None,
        }
    }

    /// Helper method for field `business_location`.
    ///
    /// For private chats with business accounts, the location of the business
    #[must_use]
    pub fn business_location(&self) -> Option<&crate::types::BusinessLocation> {
        match self {
            Self::Private(val) => val.business_location.as_ref(),
            _ => None,
        }
    }

    /// Helper method for field `business_opening_hours`.
    ///
    /// For private chats with business accounts, the opening hours of the business
    #[must_use]
    pub fn business_opening_hours(&self) -> Option<&crate::types::BusinessOpeningHours> {
        match self {
            Self::Private(val) => val.business_opening_hours.as_ref(),
            _ => None,
        }
    }

    /// Helper method for field `can_send_paid_media`.
    ///
    /// `true`, if paid media messages can be sent or forwarded to the channel chat. The field is available only for channel chats.
    #[must_use]
    pub fn can_send_paid_media(&self) -> Option<bool> {
        match self {
            Self::Channel(val) => val.can_send_paid_media,
            _ => None,
        }
    }

    /// Helper method for field `can_set_sticker_set`.
    ///
    /// `true`, if the bot can change the group sticker set
    #[must_use]
    pub fn can_set_sticker_set(&self) -> Option<bool> {
        match self {
            Self::Supergroup(val) => val.can_set_sticker_set,
            _ => None,
        }
    }

    /// Helper method for field `community`.
    ///
    /// The Community to which the chat belongs
    #[must_use]
    pub fn community(&self) -> Option<&crate::types::Community> {
        match self {
            Self::Private(val) => val.community.as_ref(),
            Self::Group(val) => val.community.as_ref(),
            Self::Supergroup(val) => val.community.as_ref(),
            Self::Channel(val) => val.community.as_ref(),
            Self::Unknown(_) => None,
        }
    }

    /// Helper method for field `custom_emoji_sticker_set_name`.
    ///
    /// For supergroups, the name of the group's custom emoji sticker set. Custom emoji from this set can be used by all users and bots in the group.
    #[must_use]
    pub fn custom_emoji_sticker_set_name(&self) -> Option<&str> {
        match self {
            Self::Supergroup(val) => val.custom_emoji_sticker_set_name.as_deref(),
            _ => None,
        }
    }

    /// Helper method for field `description`.
    ///
    /// Description, for groups, supergroups and channel chats
    #[must_use]
    pub fn description(&self) -> Option<&str> {
        match self {
            Self::Group(val) => val.description.as_deref(),
            Self::Supergroup(val) => val.description.as_deref(),
            Self::Channel(val) => val.description.as_deref(),
            _ => None,
        }
    }

    /// Helper method for field `emoji_status_custom_emoji_id`.
    ///
    /// Custom emoji identifier of the emoji status of the chat or the other party in a private chat
    #[must_use]
    pub fn emoji_status_custom_emoji_id(&self) -> Option<&str> {
        match self {
            Self::Private(val) => val.emoji_status_custom_emoji_id.as_deref(),
            _ => None,
        }
    }

    /// Helper method for field `emoji_status_expiration_date`.
    ///
    /// Expiration date of the emoji status of the chat or the other party in a private chat, in Unix time, if any
    #[must_use]
    pub fn emoji_status_expiration_date(&self) -> Option<i64> {
        match self {
            Self::Private(val) => val.emoji_status_expiration_date,
            _ => None,
        }
    }

    /// Helper method for field `first_name`.
    ///
    /// First name of the other party in a private chat
    #[must_use]
    pub fn first_name(&self) -> Option<&str> {
        match self {
            Self::Private(val) => val.first_name.as_deref(),
            _ => None,
        }
    }

    /// Helper method for field `first_profile_audio`.
    ///
    /// For private chats, the first audio added to the profile of the user
    #[must_use]
    pub fn first_profile_audio(&self) -> Option<&crate::types::Audio> {
        match self {
            Self::Private(val) => val.first_profile_audio.as_deref(),
            _ => None,
        }
    }

    /// Helper method for field `guard_bot`.
    ///
    /// The bot that processes join request queries in the chat. The field is only available to chat administrators.
    #[must_use]
    pub fn guard_bot(&self) -> Option<&crate::types::User> {
        match self {
            Self::Private(val) => val.guard_bot.as_deref(),
            Self::Group(val) => val.guard_bot.as_deref(),
            Self::Supergroup(val) => val.guard_bot.as_deref(),
            Self::Channel(val) => val.guard_bot.as_deref(),
            Self::Unknown(_) => None,
        }
    }

    /// Helper method for field `has_aggressive_anti_spam_enabled`.
    ///
    /// `true`, if aggressive anti-spam checks are enabled in the supergroup. The field is only available to chat administrators.
    #[must_use]
    pub fn has_aggressive_anti_spam_enabled(&self) -> Option<bool> {
        match self {
            Self::Supergroup(val) => val.has_aggressive_anti_spam_enabled,
            _ => None,
        }
    }

    /// Helper method for field `has_hidden_members`.
    ///
    /// `true`, if non-administrators can only get the list of bots and administrators in the chat
    #[must_use]
    pub fn has_hidden_members(&self) -> Option<bool> {
        match self {
            Self::Private(val) => val.has_hidden_members,
            Self::Group(val) => val.has_hidden_members,
            Self::Supergroup(val) => val.has_hidden_members,
            Self::Channel(val) => val.has_hidden_members,
            Self::Unknown(_) => None,
        }
    }

    /// Helper method for field `has_private_forwards`.
    ///
    /// `true`, if privacy settings of the other party in the private chat allows to use ``tg://user?id=<user_id>`` links only in chats with the user
    #[must_use]
    pub fn has_private_forwards(&self) -> Option<bool> {
        match self {
            Self::Private(val) => val.has_private_forwards,
            _ => None,
        }
    }

    /// Helper method for field `has_protected_content`.
    ///
    /// `true`, if messages from the chat can't be forwarded to other chats
    #[must_use]
    pub fn has_protected_content(&self) -> Option<bool> {
        match self {
            Self::Private(val) => val.has_protected_content,
            Self::Group(val) => val.has_protected_content,
            Self::Supergroup(val) => val.has_protected_content,
            Self::Channel(val) => val.has_protected_content,
            Self::Unknown(_) => None,
        }
    }

    /// Helper method for field `has_restricted_voice_and_video_messages`.
    ///
    /// `true`, if the privacy settings of the other party restrict sending voice and video note messages in the private chat
    #[must_use]
    pub fn has_restricted_voice_and_video_messages(&self) -> Option<bool> {
        match self {
            Self::Private(val) => val.has_restricted_voice_and_video_messages,
            _ => None,
        }
    }

    /// Helper method for field `has_visible_history`.
    ///
    /// `true`, if new chat members will have access to old messages; available only to chat administrators
    #[must_use]
    pub fn has_visible_history(&self) -> Option<bool> {
        match self {
            Self::Private(val) => val.has_visible_history,
            Self::Group(val) => val.has_visible_history,
            Self::Supergroup(val) => val.has_visible_history,
            Self::Channel(val) => val.has_visible_history,
            Self::Unknown(_) => None,
        }
    }

    /// Helper method for field `id`.
    ///
    /// Unique identifier for this chat. This number may have more than 32 significant bits and some programming languages may have difficulty/silent defects in interpreting it. But it has at most 52 significant bits, so a signed 64-bit integer or double-precision float type are safe for storing this identifier.
    #[must_use]
    pub fn id(&self) -> i64 {
        match self {
            Self::Private(val) => val.id,
            Self::Group(val) => val.id,
            Self::Supergroup(val) => val.id,
            Self::Channel(val) => val.id,
            Self::Unknown(val) => val.id,
        }
    }

    /// Helper method for field `invite_link`.
    ///
    /// Primary invite link, for groups, supergroups and channel chats
    #[must_use]
    pub fn invite_link(&self) -> Option<&str> {
        match self {
            Self::Group(val) => val.invite_link.as_deref(),
            Self::Supergroup(val) => val.invite_link.as_deref(),
            Self::Channel(val) => val.invite_link.as_deref(),
            _ => None,
        }
    }

    /// Helper method for field `is_direct_messages`.
    ///
    /// `true`, if the chat is the direct messages chat of a channel
    #[must_use]
    pub fn is_direct_messages(&self) -> Option<bool> {
        match self {
            Self::Private(val) => val.is_direct_messages,
            Self::Group(val) => val.is_direct_messages,
            Self::Supergroup(val) => val.is_direct_messages,
            Self::Channel(val) => val.is_direct_messages,
            Self::Unknown(_) => None,
        }
    }

    /// Helper method for field `is_forum`.
    ///
    /// `true`, if the supergroup chat is a forum (has topics enabled)
    #[must_use]
    pub fn is_forum(&self) -> Option<bool> {
        match self {
            Self::Supergroup(val) => val.is_forum,
            _ => None,
        }
    }

    /// Helper method for field `join_by_request`.
    ///
    /// `true`, if all users directly joining the supergroup without using an invite link need to be approved by supergroup administrators
    #[must_use]
    pub fn join_by_request(&self) -> Option<bool> {
        match self {
            Self::Supergroup(val) => val.join_by_request,
            _ => None,
        }
    }

    /// Helper method for field `join_to_send_messages`.
    ///
    /// `true`, if users need to join the supergroup before they can send messages
    #[must_use]
    pub fn join_to_send_messages(&self) -> Option<bool> {
        match self {
            Self::Supergroup(val) => val.join_to_send_messages,
            _ => None,
        }
    }

    /// Helper method for field `last_name`.
    ///
    /// Last name of the other party in a private chat
    #[must_use]
    pub fn last_name(&self) -> Option<&str> {
        match self {
            Self::Private(val) => val.last_name.as_deref(),
            _ => None,
        }
    }

    /// Helper method for field `linked_chat_id`.
    ///
    /// Unique identifier for the linked chat, i.e. the discussion group identifier for a channel and vice versa; for supergroups and channel chats. This identifier may be greater than 32 bits and some programming languages may have difficulty/silent defects in interpreting it. But it is smaller than 52 bits, so a signed 64 bit integer or double-precision float type are safe for storing this identifier.
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
    /// For supergroups, the location to which the supergroup is connected
    #[must_use]
    pub fn location(&self) -> Option<&crate::types::ChatLocation> {
        match self {
            Self::Supergroup(val) => val.location.as_ref(),
            _ => None,
        }
    }

    /// Helper method for field `max_reaction_count`.
    ///
    /// The maximum number of reactions that can be set on a message in the chat
    #[must_use]
    pub fn max_reaction_count(&self) -> i64 {
        match self {
            Self::Private(val) => val.max_reaction_count,
            Self::Group(val) => val.max_reaction_count,
            Self::Supergroup(val) => val.max_reaction_count,
            Self::Channel(val) => val.max_reaction_count,
            Self::Unknown(val) => val.max_reaction_count,
        }
    }

    /// Helper method for field `message_auto_delete_time`.
    ///
    /// The time after which all messages sent to the chat will be automatically deleted; in seconds
    #[must_use]
    pub fn message_auto_delete_time(&self) -> Option<i64> {
        match self {
            Self::Private(val) => val.message_auto_delete_time,
            Self::Group(val) => val.message_auto_delete_time,
            Self::Supergroup(val) => val.message_auto_delete_time,
            Self::Channel(val) => val.message_auto_delete_time,
            Self::Unknown(_) => None,
        }
    }

    /// Helper method for field `paid_message_star_count`.
    ///
    /// The number of Telegram Stars a general user has to pay to send a message to the chat
    #[must_use]
    pub fn paid_message_star_count(&self) -> Option<i64> {
        match self {
            Self::Private(val) => val.paid_message_star_count,
            Self::Group(val) => val.paid_message_star_count,
            Self::Supergroup(val) => val.paid_message_star_count,
            Self::Channel(val) => val.paid_message_star_count,
            Self::Unknown(_) => None,
        }
    }

    /// Helper method for field `parent_chat`.
    ///
    /// Information about the corresponding channel chat; for direct messages chats only
    #[must_use]
    pub fn parent_chat(&self) -> Option<&crate::types::Chat> {
        match self {
            Self::Supergroup(val) => val.parent_chat.as_deref(),
            _ => None,
        }
    }

    /// Helper method for field `permissions`.
    ///
    /// Default chat member permissions, for groups and supergroups
    #[must_use]
    pub fn permissions(&self) -> Option<&crate::types::ChatPermissions> {
        match self {
            Self::Group(val) => val.permissions.as_ref(),
            Self::Supergroup(val) => val.permissions.as_ref(),
            _ => None,
        }
    }

    /// Helper method for field `personal_chat`.
    ///
    /// For private chats, the personal channel of the user
    #[must_use]
    pub fn personal_chat(&self) -> Option<&crate::types::Chat> {
        match self {
            Self::Private(val) => val.personal_chat.as_deref(),
            _ => None,
        }
    }

    /// Helper method for field `photo`.
    ///
    /// Chat photo
    #[must_use]
    pub fn photo(&self) -> Option<&crate::types::ChatPhoto> {
        match self {
            Self::Private(val) => val.photo.as_ref(),
            Self::Group(val) => val.photo.as_ref(),
            Self::Supergroup(val) => val.photo.as_ref(),
            Self::Channel(val) => val.photo.as_ref(),
            Self::Unknown(_) => None,
        }
    }

    /// Helper method for field `pinned_message`.
    ///
    /// The most recent pinned message (by sending date)
    #[must_use]
    pub fn pinned_message(&self) -> Option<&crate::types::Message> {
        match self {
            Self::Private(val) => val.pinned_message.as_deref(),
            Self::Group(val) => val.pinned_message.as_deref(),
            Self::Supergroup(val) => val.pinned_message.as_deref(),
            Self::Channel(val) => val.pinned_message.as_deref(),
            Self::Unknown(_) => None,
        }
    }

    /// Helper method for field `profile_accent_color_id`.
    ///
    /// Identifier of the accent color for the chat's profile background. See profile accent colors for more details.
    #[must_use]
    pub fn profile_accent_color_id(&self) -> Option<i64> {
        match self {
            Self::Private(val) => val.profile_accent_color_id,
            Self::Group(val) => val.profile_accent_color_id,
            Self::Supergroup(val) => val.profile_accent_color_id,
            Self::Channel(val) => val.profile_accent_color_id,
            Self::Unknown(_) => None,
        }
    }

    /// Helper method for field `profile_background_custom_emoji_id`.
    ///
    /// Custom emoji identifier of the emoji chosen by the chat for its profile background
    #[must_use]
    pub fn profile_background_custom_emoji_id(&self) -> Option<&str> {
        match self {
            Self::Private(val) => val.profile_background_custom_emoji_id.as_deref(),
            Self::Group(val) => val.profile_background_custom_emoji_id.as_deref(),
            Self::Supergroup(val) => val.profile_background_custom_emoji_id.as_deref(),
            Self::Channel(val) => val.profile_background_custom_emoji_id.as_deref(),
            Self::Unknown(_) => None,
        }
    }

    /// Helper method for field `rating`.
    ///
    /// For private chats, the rating of the user if any
    #[must_use]
    pub fn rating(&self) -> Option<&crate::types::UserRating> {
        match self {
            Self::Private(val) => val.rating.as_ref(),
            _ => None,
        }
    }

    /// Helper method for field `slow_mode_delay`.
    ///
    /// For supergroups, the minimum allowed delay between consecutive messages sent by each unprivileged user; in seconds
    #[must_use]
    pub fn slow_mode_delay(&self) -> Option<i64> {
        match self {
            Self::Supergroup(val) => val.slow_mode_delay,
            _ => None,
        }
    }

    /// Helper method for field `sticker_set_name`.
    ///
    /// For supergroups, name of the group sticker set
    #[must_use]
    pub fn sticker_set_name(&self) -> Option<&str> {
        match self {
            Self::Supergroup(val) => val.sticker_set_name.as_deref(),
            _ => None,
        }
    }

    /// Helper method for field `title`.
    ///
    /// Title, for supergroups, channels and group chats
    #[must_use]
    pub fn title(&self) -> Option<&str> {
        match self {
            Self::Group(val) => val.title.as_deref(),
            Self::Supergroup(val) => val.title.as_deref(),
            Self::Channel(val) => val.title.as_deref(),
            _ => None,
        }
    }

    /// Helper method for field `unique_gift_colors`.
    ///
    /// The color scheme based on a unique gift that must be used for the chat's name, message replies and link previews
    #[must_use]
    pub fn unique_gift_colors(&self) -> Option<&crate::types::UniqueGiftColors> {
        match self {
            Self::Private(val) => val.unique_gift_colors.as_ref(),
            Self::Group(val) => val.unique_gift_colors.as_ref(),
            Self::Supergroup(val) => val.unique_gift_colors.as_ref(),
            Self::Channel(val) => val.unique_gift_colors.as_ref(),
            Self::Unknown(_) => None,
        }
    }

    /// Helper method for field `unrestrict_boost_count`.
    ///
    /// For supergroups, the minimum number of boosts that a non-administrator user needs to add in order to ignore slow mode and chat permissions
    #[must_use]
    pub fn unrestrict_boost_count(&self) -> Option<i64> {
        match self {
            Self::Supergroup(val) => val.unrestrict_boost_count,
            _ => None,
        }
    }

    /// Helper method for field `username`.
    ///
    /// Username, for private chats, supergroups and channels if available
    #[must_use]
    pub fn username(&self) -> Option<&str> {
        match self {
            Self::Private(val) => val.username.as_deref(),
            Self::Supergroup(val) => val.username.as_deref(),
            Self::Channel(val) => val.username.as_deref(),
            _ => None,
        }
    }

    /// Helper method for nested field `added_to_attachment_menu`.
    #[must_use]
    pub fn added_to_attachment_menu(&self) -> Option<bool> {
        match self {
            Self::Private(val) => val
                .guard_bot
                .as_deref()
                .and_then(|inner| inner.added_to_attachment_menu),
            Self::Group(val) => val
                .guard_bot
                .as_deref()
                .and_then(|inner| inner.added_to_attachment_menu),
            Self::Supergroup(val) => val
                .guard_bot
                .as_deref()
                .and_then(|inner| inner.added_to_attachment_menu),
            Self::Channel(val) => val
                .guard_bot
                .as_deref()
                .and_then(|inner| inner.added_to_attachment_menu),
            Self::Unknown(_) => None,
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

    /// Helper method for nested field `allows_users_to_create_topics`.
    #[must_use]
    pub fn allows_users_to_create_topics(&self) -> Option<bool> {
        match self {
            Self::Private(val) => val
                .guard_bot
                .as_deref()
                .and_then(|inner| inner.allows_users_to_create_topics),
            Self::Group(val) => val
                .guard_bot
                .as_deref()
                .and_then(|inner| inner.allows_users_to_create_topics),
            Self::Supergroup(val) => val
                .guard_bot
                .as_deref()
                .and_then(|inner| inner.allows_users_to_create_topics),
            Self::Channel(val) => val
                .guard_bot
                .as_deref()
                .and_then(|inner| inner.allows_users_to_create_topics),
            Self::Unknown(_) => None,
        }
    }

    /// Helper method for nested field `animation`.
    #[must_use]
    pub fn animation(&self) -> Option<&crate::types::Animation> {
        match self {
            Self::Private(val) => val
                .pinned_message
                .as_deref()
                .and_then(crate::types::Message::animation),
            Self::Group(val) => val
                .pinned_message
                .as_deref()
                .and_then(crate::types::Message::animation),
            Self::Supergroup(val) => val
                .pinned_message
                .as_deref()
                .and_then(crate::types::Message::animation),
            Self::Channel(val) => val
                .pinned_message
                .as_deref()
                .and_then(crate::types::Message::animation),
            Self::Unknown(_) => None,
        }
    }

    /// Helper method for nested field `audio`.
    #[must_use]
    pub fn audio(&self) -> Option<&crate::types::Audio> {
        match self {
            Self::Private(val) => val
                .pinned_message
                .as_deref()
                .and_then(crate::types::Message::audio),
            Self::Group(val) => val
                .pinned_message
                .as_deref()
                .and_then(crate::types::Message::audio),
            Self::Supergroup(val) => val
                .pinned_message
                .as_deref()
                .and_then(crate::types::Message::audio),
            Self::Channel(val) => val
                .pinned_message
                .as_deref()
                .and_then(crate::types::Message::audio),
            Self::Unknown(_) => None,
        }
    }

    /// Helper method for nested field `author_signature`.
    #[must_use]
    pub fn author_signature(&self) -> Option<&str> {
        match self {
            Self::Private(val) => val
                .pinned_message
                .as_deref()
                .and_then(crate::types::Message::author_signature),
            Self::Group(val) => val
                .pinned_message
                .as_deref()
                .and_then(crate::types::Message::author_signature),
            Self::Supergroup(val) => val
                .pinned_message
                .as_deref()
                .and_then(crate::types::Message::author_signature),
            Self::Channel(val) => val
                .pinned_message
                .as_deref()
                .and_then(crate::types::Message::author_signature),
            Self::Unknown(_) => None,
        }
    }

    /// Helper method for nested field `big_file_id`.
    #[must_use]
    pub fn big_file_id(&self) -> Option<&str> {
        match self {
            Self::Private(val) => val.photo.as_ref().map(|inner| inner.big_file_id.as_ref()),
            Self::Group(val) => val.photo.as_ref().map(|inner| inner.big_file_id.as_ref()),
            Self::Supergroup(val) => val.photo.as_ref().map(|inner| inner.big_file_id.as_ref()),
            Self::Channel(val) => val.photo.as_ref().map(|inner| inner.big_file_id.as_ref()),
            Self::Unknown(_) => None,
        }
    }

    /// Helper method for nested field `big_file_unique_id`.
    #[must_use]
    pub fn big_file_unique_id(&self) -> Option<&str> {
        match self {
            Self::Private(val) => val
                .photo
                .as_ref()
                .map(|inner| inner.big_file_unique_id.as_ref()),
            Self::Group(val) => val
                .photo
                .as_ref()
                .map(|inner| inner.big_file_unique_id.as_ref()),
            Self::Supergroup(val) => val
                .photo
                .as_ref()
                .map(|inner| inner.big_file_unique_id.as_ref()),
            Self::Channel(val) => val
                .photo
                .as_ref()
                .map(|inner| inner.big_file_unique_id.as_ref()),
            Self::Unknown(_) => None,
        }
    }

    /// Helper method for nested field `boost_added`.
    #[must_use]
    pub fn boost_added(&self) -> Option<&crate::types::ChatBoostAdded> {
        match self {
            Self::Private(val) => val
                .pinned_message
                .as_deref()
                .and_then(crate::types::Message::boost_added),
            Self::Group(val) => val
                .pinned_message
                .as_deref()
                .and_then(crate::types::Message::boost_added),
            Self::Supergroup(val) => val
                .pinned_message
                .as_deref()
                .and_then(crate::types::Message::boost_added),
            Self::Channel(val) => val
                .pinned_message
                .as_deref()
                .and_then(crate::types::Message::boost_added),
            Self::Unknown(_) => None,
        }
    }

    /// Helper method for nested field `business_connection_id`.
    #[must_use]
    pub fn business_connection_id(&self) -> Option<&str> {
        match self {
            Self::Private(val) => val
                .pinned_message
                .as_deref()
                .and_then(crate::types::Message::business_connection_id),
            Self::Group(val) => val
                .pinned_message
                .as_deref()
                .and_then(crate::types::Message::business_connection_id),
            Self::Supergroup(val) => val
                .pinned_message
                .as_deref()
                .and_then(crate::types::Message::business_connection_id),
            Self::Channel(val) => val
                .pinned_message
                .as_deref()
                .and_then(crate::types::Message::business_connection_id),
            Self::Unknown(_) => None,
        }
    }

    /// Helper method for nested field `can_add_web_page_previews`.
    #[must_use]
    pub fn can_add_web_page_previews(&self) -> Option<bool> {
        match self {
            Self::Group(val) => val
                .permissions
                .as_ref()
                .and_then(|inner| inner.can_add_web_page_previews),
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
            Self::Group(val) => val
                .permissions
                .as_ref()
                .and_then(|inner| inner.can_change_info),
            Self::Supergroup(val) => val
                .permissions
                .as_ref()
                .and_then(|inner| inner.can_change_info),
            _ => None,
        }
    }

    /// Helper method for nested field `can_connect_to_business`.
    #[must_use]
    pub fn can_connect_to_business(&self) -> Option<bool> {
        match self {
            Self::Private(val) => val
                .guard_bot
                .as_deref()
                .and_then(|inner| inner.can_connect_to_business),
            Self::Group(val) => val
                .guard_bot
                .as_deref()
                .and_then(|inner| inner.can_connect_to_business),
            Self::Supergroup(val) => val
                .guard_bot
                .as_deref()
                .and_then(|inner| inner.can_connect_to_business),
            Self::Channel(val) => val
                .guard_bot
                .as_deref()
                .and_then(|inner| inner.can_connect_to_business),
            Self::Unknown(_) => None,
        }
    }

    /// Helper method for nested field `can_edit_tag`.
    #[must_use]
    pub fn can_edit_tag(&self) -> Option<bool> {
        match self {
            Self::Group(val) => val
                .permissions
                .as_ref()
                .and_then(|inner| inner.can_edit_tag),
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
            Self::Group(val) => val
                .permissions
                .as_ref()
                .and_then(|inner| inner.can_invite_users),
            Self::Supergroup(val) => val
                .permissions
                .as_ref()
                .and_then(|inner| inner.can_invite_users),
            _ => None,
        }
    }

    /// Helper method for nested field `can_join_groups`.
    #[must_use]
    pub fn can_join_groups(&self) -> Option<bool> {
        match self {
            Self::Private(val) => val
                .guard_bot
                .as_deref()
                .and_then(|inner| inner.can_join_groups),
            Self::Group(val) => val
                .guard_bot
                .as_deref()
                .and_then(|inner| inner.can_join_groups),
            Self::Supergroup(val) => val
                .guard_bot
                .as_deref()
                .and_then(|inner| inner.can_join_groups),
            Self::Channel(val) => val
                .guard_bot
                .as_deref()
                .and_then(|inner| inner.can_join_groups),
            Self::Unknown(_) => None,
        }
    }

    /// Helper method for nested field `can_manage_bots`.
    #[must_use]
    pub fn can_manage_bots(&self) -> Option<bool> {
        match self {
            Self::Private(val) => val
                .guard_bot
                .as_deref()
                .and_then(|inner| inner.can_manage_bots),
            Self::Group(val) => val
                .guard_bot
                .as_deref()
                .and_then(|inner| inner.can_manage_bots),
            Self::Supergroup(val) => val
                .guard_bot
                .as_deref()
                .and_then(|inner| inner.can_manage_bots),
            Self::Channel(val) => val
                .guard_bot
                .as_deref()
                .and_then(|inner| inner.can_manage_bots),
            Self::Unknown(_) => None,
        }
    }

    /// Helper method for nested field `can_manage_topics`.
    #[must_use]
    pub fn can_manage_topics(&self) -> Option<bool> {
        match self {
            Self::Group(val) => val
                .permissions
                .as_ref()
                .and_then(|inner| inner.can_manage_topics),
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
            Self::Group(val) => val
                .permissions
                .as_ref()
                .and_then(|inner| inner.can_pin_messages),
            Self::Supergroup(val) => val
                .permissions
                .as_ref()
                .and_then(|inner| inner.can_pin_messages),
            _ => None,
        }
    }

    /// Helper method for nested field `can_react_to_messages`.
    #[must_use]
    pub fn can_react_to_messages(&self) -> Option<bool> {
        match self {
            Self::Group(val) => val
                .permissions
                .as_ref()
                .and_then(|inner| inner.can_react_to_messages),
            Self::Supergroup(val) => val
                .permissions
                .as_ref()
                .and_then(|inner| inner.can_react_to_messages),
            _ => None,
        }
    }

    /// Helper method for nested field `can_read_all_group_messages`.
    #[must_use]
    pub fn can_read_all_group_messages(&self) -> Option<bool> {
        match self {
            Self::Private(val) => val
                .guard_bot
                .as_deref()
                .and_then(|inner| inner.can_read_all_group_messages),
            Self::Group(val) => val
                .guard_bot
                .as_deref()
                .and_then(|inner| inner.can_read_all_group_messages),
            Self::Supergroup(val) => val
                .guard_bot
                .as_deref()
                .and_then(|inner| inner.can_read_all_group_messages),
            Self::Channel(val) => val
                .guard_bot
                .as_deref()
                .and_then(|inner| inner.can_read_all_group_messages),
            Self::Unknown(_) => None,
        }
    }

    /// Helper method for nested field `can_send_audios`.
    #[must_use]
    pub fn can_send_audios(&self) -> Option<bool> {
        match self {
            Self::Group(val) => val
                .permissions
                .as_ref()
                .and_then(|inner| inner.can_send_audios),
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
            Self::Group(val) => val
                .permissions
                .as_ref()
                .and_then(|inner| inner.can_send_documents),
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
            Self::Group(val) => val
                .permissions
                .as_ref()
                .and_then(|inner| inner.can_send_messages),
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
            Self::Group(val) => val
                .permissions
                .as_ref()
                .and_then(|inner| inner.can_send_other_messages),
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
            Self::Group(val) => val
                .permissions
                .as_ref()
                .and_then(|inner| inner.can_send_photos),
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
            Self::Group(val) => val
                .permissions
                .as_ref()
                .and_then(|inner| inner.can_send_polls),
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
            Self::Group(val) => val
                .permissions
                .as_ref()
                .and_then(|inner| inner.can_send_video_notes),
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
            Self::Group(val) => val
                .permissions
                .as_ref()
                .and_then(|inner| inner.can_send_videos),
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
            Self::Group(val) => val
                .permissions
                .as_ref()
                .and_then(|inner| inner.can_send_voice_notes),
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
        match self {
            Self::Private(val) => val
                .pinned_message
                .as_deref()
                .and_then(crate::types::Message::caption),
            Self::Group(val) => val
                .pinned_message
                .as_deref()
                .and_then(crate::types::Message::caption),
            Self::Supergroup(val) => val
                .pinned_message
                .as_deref()
                .and_then(crate::types::Message::caption),
            Self::Channel(val) => val
                .pinned_message
                .as_deref()
                .and_then(crate::types::Message::caption),
            Self::Unknown(_) => None,
        }
    }

    /// Helper method for nested field `caption_entities`.
    #[must_use]
    pub fn caption_entities(&self) -> Option<&[crate::types::MessageEntity]> {
        match self {
            Self::Private(val) => val
                .pinned_message
                .as_deref()
                .and_then(crate::types::Message::caption_entities),
            Self::Group(val) => val
                .pinned_message
                .as_deref()
                .and_then(crate::types::Message::caption_entities),
            Self::Supergroup(val) => val
                .pinned_message
                .as_deref()
                .and_then(crate::types::Message::caption_entities),
            Self::Channel(val) => val
                .pinned_message
                .as_deref()
                .and_then(crate::types::Message::caption_entities),
            Self::Unknown(_) => None,
        }
    }

    /// Helper method for nested field `channel_chat_created`.
    #[must_use]
    pub fn channel_chat_created(&self) -> Option<bool> {
        match self {
            Self::Private(val) => val
                .pinned_message
                .as_deref()
                .and_then(crate::types::Message::channel_chat_created),
            Self::Group(val) => val
                .pinned_message
                .as_deref()
                .and_then(crate::types::Message::channel_chat_created),
            Self::Supergroup(val) => val
                .pinned_message
                .as_deref()
                .and_then(crate::types::Message::channel_chat_created),
            Self::Channel(val) => val
                .pinned_message
                .as_deref()
                .and_then(crate::types::Message::channel_chat_created),
            Self::Unknown(_) => None,
        }
    }

    /// Helper method for nested field `chat`.
    #[must_use]
    pub fn chat(&self) -> Option<&crate::types::Chat> {
        match self {
            Self::Private(val) => val
                .pinned_message
                .as_deref()
                .map(crate::types::Message::chat),
            Self::Group(val) => val
                .pinned_message
                .as_deref()
                .map(crate::types::Message::chat),
            Self::Supergroup(val) => val
                .pinned_message
                .as_deref()
                .map(crate::types::Message::chat),
            Self::Channel(val) => val
                .pinned_message
                .as_deref()
                .map(crate::types::Message::chat),
            Self::Unknown(_) => None,
        }
    }

    /// Helper method for nested field `chat_background_set`.
    #[must_use]
    pub fn chat_background_set(&self) -> Option<&crate::types::ChatBackground> {
        match self {
            Self::Private(val) => val
                .pinned_message
                .as_deref()
                .and_then(crate::types::Message::chat_background_set),
            Self::Group(val) => val
                .pinned_message
                .as_deref()
                .and_then(crate::types::Message::chat_background_set),
            Self::Supergroup(val) => val
                .pinned_message
                .as_deref()
                .and_then(crate::types::Message::chat_background_set),
            Self::Channel(val) => val
                .pinned_message
                .as_deref()
                .and_then(crate::types::Message::chat_background_set),
            Self::Unknown(_) => None,
        }
    }

    /// Helper method for nested field `chat_owner_changed`.
    #[must_use]
    pub fn chat_owner_changed(&self) -> Option<&crate::types::ChatOwnerChanged> {
        match self {
            Self::Private(val) => val
                .pinned_message
                .as_deref()
                .and_then(crate::types::Message::chat_owner_changed),
            Self::Group(val) => val
                .pinned_message
                .as_deref()
                .and_then(crate::types::Message::chat_owner_changed),
            Self::Supergroup(val) => val
                .pinned_message
                .as_deref()
                .and_then(crate::types::Message::chat_owner_changed),
            Self::Channel(val) => val
                .pinned_message
                .as_deref()
                .and_then(crate::types::Message::chat_owner_changed),
            Self::Unknown(_) => None,
        }
    }

    /// Helper method for nested field `chat_owner_left`.
    #[must_use]
    pub fn chat_owner_left(&self) -> Option<&crate::types::ChatOwnerLeft> {
        match self {
            Self::Private(val) => val
                .pinned_message
                .as_deref()
                .and_then(crate::types::Message::chat_owner_left),
            Self::Group(val) => val
                .pinned_message
                .as_deref()
                .and_then(crate::types::Message::chat_owner_left),
            Self::Supergroup(val) => val
                .pinned_message
                .as_deref()
                .and_then(crate::types::Message::chat_owner_left),
            Self::Channel(val) => val
                .pinned_message
                .as_deref()
                .and_then(crate::types::Message::chat_owner_left),
            Self::Unknown(_) => None,
        }
    }

    /// Helper method for nested field `chat_shared`.
    #[must_use]
    pub fn chat_shared(&self) -> Option<&crate::types::ChatShared> {
        match self {
            Self::Private(val) => val
                .pinned_message
                .as_deref()
                .and_then(crate::types::Message::chat_shared),
            Self::Group(val) => val
                .pinned_message
                .as_deref()
                .and_then(crate::types::Message::chat_shared),
            Self::Supergroup(val) => val
                .pinned_message
                .as_deref()
                .and_then(crate::types::Message::chat_shared),
            Self::Channel(val) => val
                .pinned_message
                .as_deref()
                .and_then(crate::types::Message::chat_shared),
            Self::Unknown(_) => None,
        }
    }

    /// Helper method for nested field `checklist`.
    #[must_use]
    pub fn checklist(&self) -> Option<&crate::types::Checklist> {
        match self {
            Self::Private(val) => val
                .pinned_message
                .as_deref()
                .and_then(crate::types::Message::checklist),
            Self::Group(val) => val
                .pinned_message
                .as_deref()
                .and_then(crate::types::Message::checklist),
            Self::Supergroup(val) => val
                .pinned_message
                .as_deref()
                .and_then(crate::types::Message::checklist),
            Self::Channel(val) => val
                .pinned_message
                .as_deref()
                .and_then(crate::types::Message::checklist),
            Self::Unknown(_) => None,
        }
    }

    /// Helper method for nested field `checklist_tasks_added`.
    #[must_use]
    pub fn checklist_tasks_added(&self) -> Option<&crate::types::ChecklistTasksAdded> {
        match self {
            Self::Private(val) => val
                .pinned_message
                .as_deref()
                .and_then(crate::types::Message::checklist_tasks_added),
            Self::Group(val) => val
                .pinned_message
                .as_deref()
                .and_then(crate::types::Message::checklist_tasks_added),
            Self::Supergroup(val) => val
                .pinned_message
                .as_deref()
                .and_then(crate::types::Message::checklist_tasks_added),
            Self::Channel(val) => val
                .pinned_message
                .as_deref()
                .and_then(crate::types::Message::checklist_tasks_added),
            Self::Unknown(_) => None,
        }
    }

    /// Helper method for nested field `checklist_tasks_done`.
    #[must_use]
    pub fn checklist_tasks_done(&self) -> Option<&crate::types::ChecklistTasksDone> {
        match self {
            Self::Private(val) => val
                .pinned_message
                .as_deref()
                .and_then(crate::types::Message::checklist_tasks_done),
            Self::Group(val) => val
                .pinned_message
                .as_deref()
                .and_then(crate::types::Message::checklist_tasks_done),
            Self::Supergroup(val) => val
                .pinned_message
                .as_deref()
                .and_then(crate::types::Message::checklist_tasks_done),
            Self::Channel(val) => val
                .pinned_message
                .as_deref()
                .and_then(crate::types::Message::checklist_tasks_done),
            Self::Unknown(_) => None,
        }
    }

    /// Helper method for nested field `community_chat_added`.
    #[must_use]
    pub fn community_chat_added(&self) -> Option<&crate::types::CommunityChatAdded> {
        match self {
            Self::Private(val) => val
                .pinned_message
                .as_deref()
                .and_then(crate::types::Message::community_chat_added),
            Self::Group(val) => val
                .pinned_message
                .as_deref()
                .and_then(crate::types::Message::community_chat_added),
            Self::Supergroup(val) => val
                .pinned_message
                .as_deref()
                .and_then(crate::types::Message::community_chat_added),
            Self::Channel(val) => val
                .pinned_message
                .as_deref()
                .and_then(crate::types::Message::community_chat_added),
            Self::Unknown(_) => None,
        }
    }

    /// Helper method for nested field `community_chat_removed`.
    #[must_use]
    pub fn community_chat_removed(&self) -> Option<&crate::types::CommunityChatRemoved> {
        match self {
            Self::Private(val) => val
                .pinned_message
                .as_deref()
                .and_then(crate::types::Message::community_chat_removed),
            Self::Group(val) => val
                .pinned_message
                .as_deref()
                .and_then(crate::types::Message::community_chat_removed),
            Self::Supergroup(val) => val
                .pinned_message
                .as_deref()
                .and_then(crate::types::Message::community_chat_removed),
            Self::Channel(val) => val
                .pinned_message
                .as_deref()
                .and_then(crate::types::Message::community_chat_removed),
            Self::Unknown(_) => None,
        }
    }

    /// Helper method for nested field `connected_website`.
    #[must_use]
    pub fn connected_website(&self) -> Option<&str> {
        match self {
            Self::Private(val) => val
                .pinned_message
                .as_deref()
                .and_then(crate::types::Message::connected_website),
            Self::Group(val) => val
                .pinned_message
                .as_deref()
                .and_then(crate::types::Message::connected_website),
            Self::Supergroup(val) => val
                .pinned_message
                .as_deref()
                .and_then(crate::types::Message::connected_website),
            Self::Channel(val) => val
                .pinned_message
                .as_deref()
                .and_then(crate::types::Message::connected_website),
            Self::Unknown(_) => None,
        }
    }

    /// Helper method for nested field `contact`.
    #[must_use]
    pub fn contact(&self) -> Option<&crate::types::Contact> {
        match self {
            Self::Private(val) => val
                .pinned_message
                .as_deref()
                .and_then(crate::types::Message::contact),
            Self::Group(val) => val
                .pinned_message
                .as_deref()
                .and_then(crate::types::Message::contact),
            Self::Supergroup(val) => val
                .pinned_message
                .as_deref()
                .and_then(crate::types::Message::contact),
            Self::Channel(val) => val
                .pinned_message
                .as_deref()
                .and_then(crate::types::Message::contact),
            Self::Unknown(_) => None,
        }
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
    pub fn dark_theme_main_color(&self) -> Option<i32> {
        match self {
            Self::Private(val) => val
                .unique_gift_colors
                .as_ref()
                .map(|inner| inner.dark_theme_main_color),
            Self::Group(val) => val
                .unique_gift_colors
                .as_ref()
                .map(|inner| inner.dark_theme_main_color),
            Self::Supergroup(val) => val
                .unique_gift_colors
                .as_ref()
                .map(|inner| inner.dark_theme_main_color),
            Self::Channel(val) => val
                .unique_gift_colors
                .as_ref()
                .map(|inner| inner.dark_theme_main_color),
            Self::Unknown(_) => None,
        }
    }

    /// Helper method for nested field `dark_theme_other_colors`.
    #[must_use]
    pub fn dark_theme_other_colors(&self) -> Option<&[i32]> {
        match self {
            Self::Private(val) => val
                .unique_gift_colors
                .as_ref()
                .map(|inner| inner.dark_theme_other_colors.as_ref()),
            Self::Group(val) => val
                .unique_gift_colors
                .as_ref()
                .map(|inner| inner.dark_theme_other_colors.as_ref()),
            Self::Supergroup(val) => val
                .unique_gift_colors
                .as_ref()
                .map(|inner| inner.dark_theme_other_colors.as_ref()),
            Self::Channel(val) => val
                .unique_gift_colors
                .as_ref()
                .map(|inner| inner.dark_theme_other_colors.as_ref()),
            Self::Unknown(_) => None,
        }
    }

    /// Helper method for nested field `date`.
    #[must_use]
    pub fn date(&self) -> Option<i64> {
        match self {
            Self::Private(val) => val
                .pinned_message
                .as_deref()
                .map(crate::types::Message::date),
            Self::Group(val) => val
                .pinned_message
                .as_deref()
                .map(crate::types::Message::date),
            Self::Supergroup(val) => val
                .pinned_message
                .as_deref()
                .map(crate::types::Message::date),
            Self::Channel(val) => val
                .pinned_message
                .as_deref()
                .map(crate::types::Message::date),
            Self::Unknown(_) => None,
        }
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
        match self {
            Self::Private(val) => val
                .pinned_message
                .as_deref()
                .and_then(crate::types::Message::delete_chat_photo),
            Self::Group(val) => val
                .pinned_message
                .as_deref()
                .and_then(crate::types::Message::delete_chat_photo),
            Self::Supergroup(val) => val
                .pinned_message
                .as_deref()
                .and_then(crate::types::Message::delete_chat_photo),
            Self::Channel(val) => val
                .pinned_message
                .as_deref()
                .and_then(crate::types::Message::delete_chat_photo),
            Self::Unknown(_) => None,
        }
    }

    /// Helper method for nested field `dice`.
    #[must_use]
    pub fn dice(&self) -> Option<&crate::types::Dice> {
        match self {
            Self::Private(val) => val
                .pinned_message
                .as_deref()
                .and_then(crate::types::Message::dice),
            Self::Group(val) => val
                .pinned_message
                .as_deref()
                .and_then(crate::types::Message::dice),
            Self::Supergroup(val) => val
                .pinned_message
                .as_deref()
                .and_then(crate::types::Message::dice),
            Self::Channel(val) => val
                .pinned_message
                .as_deref()
                .and_then(crate::types::Message::dice),
            Self::Unknown(_) => None,
        }
    }

    /// Helper method for nested field `direct_message_price_changed`.
    #[must_use]
    pub fn direct_message_price_changed(&self) -> Option<&crate::types::DirectMessagePriceChanged> {
        match self {
            Self::Private(val) => val
                .pinned_message
                .as_deref()
                .and_then(crate::types::Message::direct_message_price_changed),
            Self::Group(val) => val
                .pinned_message
                .as_deref()
                .and_then(crate::types::Message::direct_message_price_changed),
            Self::Supergroup(val) => val
                .pinned_message
                .as_deref()
                .and_then(crate::types::Message::direct_message_price_changed),
            Self::Channel(val) => val
                .pinned_message
                .as_deref()
                .and_then(crate::types::Message::direct_message_price_changed),
            Self::Unknown(_) => None,
        }
    }

    /// Helper method for nested field `direct_messages_topic`.
    #[must_use]
    pub fn direct_messages_topic(&self) -> Option<&crate::types::DirectMessagesTopic> {
        match self {
            Self::Private(val) => val
                .pinned_message
                .as_deref()
                .and_then(crate::types::Message::direct_messages_topic),
            Self::Group(val) => val
                .pinned_message
                .as_deref()
                .and_then(crate::types::Message::direct_messages_topic),
            Self::Supergroup(val) => val
                .pinned_message
                .as_deref()
                .and_then(crate::types::Message::direct_messages_topic),
            Self::Channel(val) => val
                .pinned_message
                .as_deref()
                .and_then(crate::types::Message::direct_messages_topic),
            Self::Unknown(_) => None,
        }
    }

    /// Helper method for nested field `document`.
    #[must_use]
    pub fn document(&self) -> Option<&crate::types::Document> {
        match self {
            Self::Private(val) => val
                .pinned_message
                .as_deref()
                .and_then(crate::types::Message::document),
            Self::Group(val) => val
                .pinned_message
                .as_deref()
                .and_then(crate::types::Message::document),
            Self::Supergroup(val) => val
                .pinned_message
                .as_deref()
                .and_then(crate::types::Message::document),
            Self::Channel(val) => val
                .pinned_message
                .as_deref()
                .and_then(crate::types::Message::document),
            Self::Unknown(_) => None,
        }
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
        match self {
            Self::Private(val) => val
                .pinned_message
                .as_deref()
                .and_then(crate::types::Message::edit_date),
            Self::Group(val) => val
                .pinned_message
                .as_deref()
                .and_then(crate::types::Message::edit_date),
            Self::Supergroup(val) => val
                .pinned_message
                .as_deref()
                .and_then(crate::types::Message::edit_date),
            Self::Channel(val) => val
                .pinned_message
                .as_deref()
                .and_then(crate::types::Message::edit_date),
            Self::Unknown(_) => None,
        }
    }

    /// Helper method for nested field `effect_id`.
    #[must_use]
    pub fn effect_id(&self) -> Option<&str> {
        match self {
            Self::Private(val) => val
                .pinned_message
                .as_deref()
                .and_then(crate::types::Message::effect_id),
            Self::Group(val) => val
                .pinned_message
                .as_deref()
                .and_then(crate::types::Message::effect_id),
            Self::Supergroup(val) => val
                .pinned_message
                .as_deref()
                .and_then(crate::types::Message::effect_id),
            Self::Channel(val) => val
                .pinned_message
                .as_deref()
                .and_then(crate::types::Message::effect_id),
            Self::Unknown(_) => None,
        }
    }

    /// Helper method for nested field `entities`.
    #[must_use]
    pub fn entities(&self) -> Option<&[crate::types::MessageEntity]> {
        match self {
            Self::Private(val) => val
                .pinned_message
                .as_deref()
                .and_then(crate::types::Message::entities),
            Self::Group(val) => val
                .pinned_message
                .as_deref()
                .and_then(crate::types::Message::entities),
            Self::Supergroup(val) => val
                .pinned_message
                .as_deref()
                .and_then(crate::types::Message::entities),
            Self::Channel(val) => val
                .pinned_message
                .as_deref()
                .and_then(crate::types::Message::entities),
            Self::Unknown(_) => None,
        }
    }

    /// Helper method for nested field `ephemeral_message_id`.
    #[must_use]
    pub fn ephemeral_message_id(&self) -> Option<i64> {
        match self {
            Self::Private(val) => val
                .pinned_message
                .as_deref()
                .and_then(crate::types::Message::ephemeral_message_id),
            Self::Group(val) => val
                .pinned_message
                .as_deref()
                .and_then(crate::types::Message::ephemeral_message_id),
            Self::Supergroup(val) => val
                .pinned_message
                .as_deref()
                .and_then(crate::types::Message::ephemeral_message_id),
            Self::Channel(val) => val
                .pinned_message
                .as_deref()
                .and_then(crate::types::Message::ephemeral_message_id),
            Self::Unknown(_) => None,
        }
    }

    /// Helper method for nested field `external_reply`.
    #[must_use]
    pub fn external_reply(&self) -> Option<&crate::types::ExternalReplyInfo> {
        match self {
            Self::Private(val) => val
                .pinned_message
                .as_deref()
                .and_then(crate::types::Message::external_reply),
            Self::Group(val) => val
                .pinned_message
                .as_deref()
                .and_then(crate::types::Message::external_reply),
            Self::Supergroup(val) => val
                .pinned_message
                .as_deref()
                .and_then(crate::types::Message::external_reply),
            Self::Channel(val) => val
                .pinned_message
                .as_deref()
                .and_then(crate::types::Message::external_reply),
            Self::Unknown(_) => None,
        }
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
        match self {
            Self::Private(val) => val
                .pinned_message
                .as_deref()
                .and_then(crate::types::Message::forum_topic_closed),
            Self::Group(val) => val
                .pinned_message
                .as_deref()
                .and_then(crate::types::Message::forum_topic_closed),
            Self::Supergroup(val) => val
                .pinned_message
                .as_deref()
                .and_then(crate::types::Message::forum_topic_closed),
            Self::Channel(val) => val
                .pinned_message
                .as_deref()
                .and_then(crate::types::Message::forum_topic_closed),
            Self::Unknown(_) => None,
        }
    }

    /// Helper method for nested field `forum_topic_created`.
    #[must_use]
    pub fn forum_topic_created(&self) -> Option<&crate::types::ForumTopicCreated> {
        match self {
            Self::Private(val) => val
                .pinned_message
                .as_deref()
                .and_then(crate::types::Message::forum_topic_created),
            Self::Group(val) => val
                .pinned_message
                .as_deref()
                .and_then(crate::types::Message::forum_topic_created),
            Self::Supergroup(val) => val
                .pinned_message
                .as_deref()
                .and_then(crate::types::Message::forum_topic_created),
            Self::Channel(val) => val
                .pinned_message
                .as_deref()
                .and_then(crate::types::Message::forum_topic_created),
            Self::Unknown(_) => None,
        }
    }

    /// Helper method for nested field `forum_topic_edited`.
    #[must_use]
    pub fn forum_topic_edited(&self) -> Option<&crate::types::ForumTopicEdited> {
        match self {
            Self::Private(val) => val
                .pinned_message
                .as_deref()
                .and_then(crate::types::Message::forum_topic_edited),
            Self::Group(val) => val
                .pinned_message
                .as_deref()
                .and_then(crate::types::Message::forum_topic_edited),
            Self::Supergroup(val) => val
                .pinned_message
                .as_deref()
                .and_then(crate::types::Message::forum_topic_edited),
            Self::Channel(val) => val
                .pinned_message
                .as_deref()
                .and_then(crate::types::Message::forum_topic_edited),
            Self::Unknown(_) => None,
        }
    }

    /// Helper method for nested field `forum_topic_reopened`.
    #[must_use]
    pub fn forum_topic_reopened(&self) -> Option<&crate::types::ForumTopicReopened> {
        match self {
            Self::Private(val) => val
                .pinned_message
                .as_deref()
                .and_then(crate::types::Message::forum_topic_reopened),
            Self::Group(val) => val
                .pinned_message
                .as_deref()
                .and_then(crate::types::Message::forum_topic_reopened),
            Self::Supergroup(val) => val
                .pinned_message
                .as_deref()
                .and_then(crate::types::Message::forum_topic_reopened),
            Self::Channel(val) => val
                .pinned_message
                .as_deref()
                .and_then(crate::types::Message::forum_topic_reopened),
            Self::Unknown(_) => None,
        }
    }

    /// Helper method for nested field `forward_origin`.
    #[must_use]
    pub fn forward_origin(&self) -> Option<&crate::types::MessageOrigin> {
        match self {
            Self::Private(val) => val
                .pinned_message
                .as_deref()
                .and_then(crate::types::Message::forward_origin),
            Self::Group(val) => val
                .pinned_message
                .as_deref()
                .and_then(crate::types::Message::forward_origin),
            Self::Supergroup(val) => val
                .pinned_message
                .as_deref()
                .and_then(crate::types::Message::forward_origin),
            Self::Channel(val) => val
                .pinned_message
                .as_deref()
                .and_then(crate::types::Message::forward_origin),
            Self::Unknown(_) => None,
        }
    }

    /// Helper method for nested field `from`.
    #[must_use]
    pub fn from(&self) -> Option<&crate::types::User> {
        match self {
            Self::Private(val) => val
                .pinned_message
                .as_deref()
                .and_then(crate::types::Message::from),
            Self::Group(val) => val
                .pinned_message
                .as_deref()
                .and_then(crate::types::Message::from),
            Self::Supergroup(val) => val
                .pinned_message
                .as_deref()
                .and_then(crate::types::Message::from),
            Self::Channel(val) => val
                .pinned_message
                .as_deref()
                .and_then(crate::types::Message::from),
            Self::Unknown(_) => None,
        }
    }

    /// Helper method for nested field `game`.
    #[must_use]
    pub fn game(&self) -> Option<&crate::types::Game> {
        match self {
            Self::Private(val) => val
                .pinned_message
                .as_deref()
                .and_then(crate::types::Message::game),
            Self::Group(val) => val
                .pinned_message
                .as_deref()
                .and_then(crate::types::Message::game),
            Self::Supergroup(val) => val
                .pinned_message
                .as_deref()
                .and_then(crate::types::Message::game),
            Self::Channel(val) => val
                .pinned_message
                .as_deref()
                .and_then(crate::types::Message::game),
            Self::Unknown(_) => None,
        }
    }

    /// Helper method for nested field `general_forum_topic_hidden`.
    #[must_use]
    pub fn general_forum_topic_hidden(&self) -> Option<&crate::types::GeneralForumTopicHidden> {
        match self {
            Self::Private(val) => val
                .pinned_message
                .as_deref()
                .and_then(crate::types::Message::general_forum_topic_hidden),
            Self::Group(val) => val
                .pinned_message
                .as_deref()
                .and_then(crate::types::Message::general_forum_topic_hidden),
            Self::Supergroup(val) => val
                .pinned_message
                .as_deref()
                .and_then(crate::types::Message::general_forum_topic_hidden),
            Self::Channel(val) => val
                .pinned_message
                .as_deref()
                .and_then(crate::types::Message::general_forum_topic_hidden),
            Self::Unknown(_) => None,
        }
    }

    /// Helper method for nested field `general_forum_topic_unhidden`.
    #[must_use]
    pub fn general_forum_topic_unhidden(&self) -> Option<&crate::types::GeneralForumTopicUnhidden> {
        match self {
            Self::Private(val) => val
                .pinned_message
                .as_deref()
                .and_then(crate::types::Message::general_forum_topic_unhidden),
            Self::Group(val) => val
                .pinned_message
                .as_deref()
                .and_then(crate::types::Message::general_forum_topic_unhidden),
            Self::Supergroup(val) => val
                .pinned_message
                .as_deref()
                .and_then(crate::types::Message::general_forum_topic_unhidden),
            Self::Channel(val) => val
                .pinned_message
                .as_deref()
                .and_then(crate::types::Message::general_forum_topic_unhidden),
            Self::Unknown(_) => None,
        }
    }

    /// Helper method for nested field `gift`.
    #[must_use]
    pub fn gift(&self) -> Option<&crate::types::GiftInfo> {
        match self {
            Self::Private(val) => val
                .pinned_message
                .as_deref()
                .and_then(crate::types::Message::gift),
            Self::Group(val) => val
                .pinned_message
                .as_deref()
                .and_then(crate::types::Message::gift),
            Self::Supergroup(val) => val
                .pinned_message
                .as_deref()
                .and_then(crate::types::Message::gift),
            Self::Channel(val) => val
                .pinned_message
                .as_deref()
                .and_then(crate::types::Message::gift),
            Self::Unknown(_) => None,
        }
    }

    /// Helper method for nested field `gift_upgrade_sent`.
    #[must_use]
    pub fn gift_upgrade_sent(&self) -> Option<&crate::types::GiftInfo> {
        match self {
            Self::Private(val) => val
                .pinned_message
                .as_deref()
                .and_then(crate::types::Message::gift_upgrade_sent),
            Self::Group(val) => val
                .pinned_message
                .as_deref()
                .and_then(crate::types::Message::gift_upgrade_sent),
            Self::Supergroup(val) => val
                .pinned_message
                .as_deref()
                .and_then(crate::types::Message::gift_upgrade_sent),
            Self::Channel(val) => val
                .pinned_message
                .as_deref()
                .and_then(crate::types::Message::gift_upgrade_sent),
            Self::Unknown(_) => None,
        }
    }

    /// Helper method for nested field `gifts_from_channels`.
    #[must_use]
    pub fn gifts_from_channels(&self) -> bool {
        {
            let inner = self.accepted_gift_types();
            inner.gifts_from_channels
        }
    }

    /// Helper method for nested field `giveaway`.
    #[must_use]
    pub fn giveaway(&self) -> Option<&crate::types::Giveaway> {
        match self {
            Self::Private(val) => val
                .pinned_message
                .as_deref()
                .and_then(crate::types::Message::giveaway),
            Self::Group(val) => val
                .pinned_message
                .as_deref()
                .and_then(crate::types::Message::giveaway),
            Self::Supergroup(val) => val
                .pinned_message
                .as_deref()
                .and_then(crate::types::Message::giveaway),
            Self::Channel(val) => val
                .pinned_message
                .as_deref()
                .and_then(crate::types::Message::giveaway),
            Self::Unknown(_) => None,
        }
    }

    /// Helper method for nested field `giveaway_completed`.
    #[must_use]
    pub fn giveaway_completed(&self) -> Option<&crate::types::GiveawayCompleted> {
        match self {
            Self::Private(val) => val
                .pinned_message
                .as_deref()
                .and_then(crate::types::Message::giveaway_completed),
            Self::Group(val) => val
                .pinned_message
                .as_deref()
                .and_then(crate::types::Message::giveaway_completed),
            Self::Supergroup(val) => val
                .pinned_message
                .as_deref()
                .and_then(crate::types::Message::giveaway_completed),
            Self::Channel(val) => val
                .pinned_message
                .as_deref()
                .and_then(crate::types::Message::giveaway_completed),
            Self::Unknown(_) => None,
        }
    }

    /// Helper method for nested field `giveaway_created`.
    #[must_use]
    pub fn giveaway_created(&self) -> Option<&crate::types::GiveawayCreated> {
        match self {
            Self::Private(val) => val
                .pinned_message
                .as_deref()
                .and_then(crate::types::Message::giveaway_created),
            Self::Group(val) => val
                .pinned_message
                .as_deref()
                .and_then(crate::types::Message::giveaway_created),
            Self::Supergroup(val) => val
                .pinned_message
                .as_deref()
                .and_then(crate::types::Message::giveaway_created),
            Self::Channel(val) => val
                .pinned_message
                .as_deref()
                .and_then(crate::types::Message::giveaway_created),
            Self::Unknown(_) => None,
        }
    }

    /// Helper method for nested field `giveaway_winners`.
    #[must_use]
    pub fn giveaway_winners(&self) -> Option<&crate::types::GiveawayWinners> {
        match self {
            Self::Private(val) => val
                .pinned_message
                .as_deref()
                .and_then(crate::types::Message::giveaway_winners),
            Self::Group(val) => val
                .pinned_message
                .as_deref()
                .and_then(crate::types::Message::giveaway_winners),
            Self::Supergroup(val) => val
                .pinned_message
                .as_deref()
                .and_then(crate::types::Message::giveaway_winners),
            Self::Channel(val) => val
                .pinned_message
                .as_deref()
                .and_then(crate::types::Message::giveaway_winners),
            Self::Unknown(_) => None,
        }
    }

    /// Helper method for nested field `group_chat_created`.
    #[must_use]
    pub fn group_chat_created(&self) -> Option<bool> {
        match self {
            Self::Private(val) => val
                .pinned_message
                .as_deref()
                .and_then(crate::types::Message::group_chat_created),
            Self::Group(val) => val
                .pinned_message
                .as_deref()
                .and_then(crate::types::Message::group_chat_created),
            Self::Supergroup(val) => val
                .pinned_message
                .as_deref()
                .and_then(crate::types::Message::group_chat_created),
            Self::Channel(val) => val
                .pinned_message
                .as_deref()
                .and_then(crate::types::Message::group_chat_created),
            Self::Unknown(_) => None,
        }
    }

    /// Helper method for nested field `guest_bot_caller_chat`.
    #[must_use]
    pub fn guest_bot_caller_chat(&self) -> Option<&crate::types::Chat> {
        match self {
            Self::Private(val) => val
                .pinned_message
                .as_deref()
                .and_then(crate::types::Message::guest_bot_caller_chat),
            Self::Group(val) => val
                .pinned_message
                .as_deref()
                .and_then(crate::types::Message::guest_bot_caller_chat),
            Self::Supergroup(val) => val
                .pinned_message
                .as_deref()
                .and_then(crate::types::Message::guest_bot_caller_chat),
            Self::Channel(val) => val
                .pinned_message
                .as_deref()
                .and_then(crate::types::Message::guest_bot_caller_chat),
            Self::Unknown(_) => None,
        }
    }

    /// Helper method for nested field `guest_bot_caller_user`.
    #[must_use]
    pub fn guest_bot_caller_user(&self) -> Option<&crate::types::User> {
        match self {
            Self::Private(val) => val
                .pinned_message
                .as_deref()
                .and_then(crate::types::Message::guest_bot_caller_user),
            Self::Group(val) => val
                .pinned_message
                .as_deref()
                .and_then(crate::types::Message::guest_bot_caller_user),
            Self::Supergroup(val) => val
                .pinned_message
                .as_deref()
                .and_then(crate::types::Message::guest_bot_caller_user),
            Self::Channel(val) => val
                .pinned_message
                .as_deref()
                .and_then(crate::types::Message::guest_bot_caller_user),
            Self::Unknown(_) => None,
        }
    }

    /// Helper method for nested field `guest_query_id`.
    #[must_use]
    pub fn guest_query_id(&self) -> Option<&str> {
        match self {
            Self::Private(val) => val
                .pinned_message
                .as_deref()
                .and_then(crate::types::Message::guest_query_id),
            Self::Group(val) => val
                .pinned_message
                .as_deref()
                .and_then(crate::types::Message::guest_query_id),
            Self::Supergroup(val) => val
                .pinned_message
                .as_deref()
                .and_then(crate::types::Message::guest_query_id),
            Self::Channel(val) => val
                .pinned_message
                .as_deref()
                .and_then(crate::types::Message::guest_query_id),
            Self::Unknown(_) => None,
        }
    }

    /// Helper method for nested field `has_main_web_app`.
    #[must_use]
    pub fn has_main_web_app(&self) -> Option<bool> {
        match self {
            Self::Private(val) => val
                .guard_bot
                .as_deref()
                .and_then(|inner| inner.has_main_web_app),
            Self::Group(val) => val
                .guard_bot
                .as_deref()
                .and_then(|inner| inner.has_main_web_app),
            Self::Supergroup(val) => val
                .guard_bot
                .as_deref()
                .and_then(|inner| inner.has_main_web_app),
            Self::Channel(val) => val
                .guard_bot
                .as_deref()
                .and_then(|inner| inner.has_main_web_app),
            Self::Unknown(_) => None,
        }
    }

    /// Helper method for nested field `has_media_spoiler`.
    #[must_use]
    pub fn has_media_spoiler(&self) -> Option<bool> {
        match self {
            Self::Private(val) => val
                .pinned_message
                .as_deref()
                .and_then(crate::types::Message::has_media_spoiler),
            Self::Group(val) => val
                .pinned_message
                .as_deref()
                .and_then(crate::types::Message::has_media_spoiler),
            Self::Supergroup(val) => val
                .pinned_message
                .as_deref()
                .and_then(crate::types::Message::has_media_spoiler),
            Self::Channel(val) => val
                .pinned_message
                .as_deref()
                .and_then(crate::types::Message::has_media_spoiler),
            Self::Unknown(_) => None,
        }
    }

    /// Helper method for nested field `has_topics_enabled`.
    #[must_use]
    pub fn has_topics_enabled(&self) -> Option<bool> {
        match self {
            Self::Private(val) => val
                .guard_bot
                .as_deref()
                .and_then(|inner| inner.has_topics_enabled),
            Self::Group(val) => val
                .guard_bot
                .as_deref()
                .and_then(|inner| inner.has_topics_enabled),
            Self::Supergroup(val) => val
                .guard_bot
                .as_deref()
                .and_then(|inner| inner.has_topics_enabled),
            Self::Channel(val) => val
                .guard_bot
                .as_deref()
                .and_then(|inner| inner.has_topics_enabled),
            Self::Unknown(_) => None,
        }
    }

    /// Helper method for nested field `invoice`.
    #[must_use]
    pub fn invoice(&self) -> Option<&crate::types::Invoice> {
        match self {
            Self::Private(val) => val
                .pinned_message
                .as_deref()
                .and_then(crate::types::Message::invoice),
            Self::Group(val) => val
                .pinned_message
                .as_deref()
                .and_then(crate::types::Message::invoice),
            Self::Supergroup(val) => val
                .pinned_message
                .as_deref()
                .and_then(crate::types::Message::invoice),
            Self::Channel(val) => val
                .pinned_message
                .as_deref()
                .and_then(crate::types::Message::invoice),
            Self::Unknown(_) => None,
        }
    }

    /// Helper method for nested field `is_automatic_forward`.
    #[must_use]
    pub fn is_automatic_forward(&self) -> Option<bool> {
        match self {
            Self::Private(val) => val
                .pinned_message
                .as_deref()
                .and_then(crate::types::Message::is_automatic_forward),
            Self::Group(val) => val
                .pinned_message
                .as_deref()
                .and_then(crate::types::Message::is_automatic_forward),
            Self::Supergroup(val) => val
                .pinned_message
                .as_deref()
                .and_then(crate::types::Message::is_automatic_forward),
            Self::Channel(val) => val
                .pinned_message
                .as_deref()
                .and_then(crate::types::Message::is_automatic_forward),
            Self::Unknown(_) => None,
        }
    }

    /// Helper method for nested field `is_bot`.
    #[must_use]
    pub fn is_bot(&self) -> Option<bool> {
        match self {
            Self::Private(val) => val.guard_bot.as_deref().map(|inner| inner.is_bot),
            Self::Group(val) => val.guard_bot.as_deref().map(|inner| inner.is_bot),
            Self::Supergroup(val) => val.guard_bot.as_deref().map(|inner| inner.is_bot),
            Self::Channel(val) => val.guard_bot.as_deref().map(|inner| inner.is_bot),
            Self::Unknown(_) => None,
        }
    }

    /// Helper method for nested field `is_from_offline`.
    #[must_use]
    pub fn is_from_offline(&self) -> Option<bool> {
        match self {
            Self::Private(val) => val
                .pinned_message
                .as_deref()
                .and_then(crate::types::Message::is_from_offline),
            Self::Group(val) => val
                .pinned_message
                .as_deref()
                .and_then(crate::types::Message::is_from_offline),
            Self::Supergroup(val) => val
                .pinned_message
                .as_deref()
                .and_then(crate::types::Message::is_from_offline),
            Self::Channel(val) => val
                .pinned_message
                .as_deref()
                .and_then(crate::types::Message::is_from_offline),
            Self::Unknown(_) => None,
        }
    }

    /// Helper method for nested field `is_paid_post`.
    #[must_use]
    pub fn is_paid_post(&self) -> Option<bool> {
        match self {
            Self::Private(val) => val
                .pinned_message
                .as_deref()
                .and_then(crate::types::Message::is_paid_post),
            Self::Group(val) => val
                .pinned_message
                .as_deref()
                .and_then(crate::types::Message::is_paid_post),
            Self::Supergroup(val) => val
                .pinned_message
                .as_deref()
                .and_then(crate::types::Message::is_paid_post),
            Self::Channel(val) => val
                .pinned_message
                .as_deref()
                .and_then(crate::types::Message::is_paid_post),
            Self::Unknown(_) => None,
        }
    }

    /// Helper method for nested field `is_premium`.
    #[must_use]
    pub fn is_premium(&self) -> Option<bool> {
        match self {
            Self::Private(val) => val.guard_bot.as_deref().and_then(|inner| inner.is_premium),
            Self::Group(val) => val.guard_bot.as_deref().and_then(|inner| inner.is_premium),
            Self::Supergroup(val) => val.guard_bot.as_deref().and_then(|inner| inner.is_premium),
            Self::Channel(val) => val.guard_bot.as_deref().and_then(|inner| inner.is_premium),
            Self::Unknown(_) => None,
        }
    }

    /// Helper method for nested field `is_topic_message`.
    #[must_use]
    pub fn is_topic_message(&self) -> Option<bool> {
        match self {
            Self::Private(val) => val
                .pinned_message
                .as_deref()
                .and_then(crate::types::Message::is_topic_message),
            Self::Group(val) => val
                .pinned_message
                .as_deref()
                .and_then(crate::types::Message::is_topic_message),
            Self::Supergroup(val) => val
                .pinned_message
                .as_deref()
                .and_then(crate::types::Message::is_topic_message),
            Self::Channel(val) => val
                .pinned_message
                .as_deref()
                .and_then(crate::types::Message::is_topic_message),
            Self::Unknown(_) => None,
        }
    }

    /// Helper method for nested field `language_code`.
    #[must_use]
    pub fn language_code(&self) -> Option<&str> {
        match self {
            Self::Private(val) => val
                .guard_bot
                .as_deref()
                .and_then(|inner| inner.language_code.as_deref()),
            Self::Group(val) => val
                .guard_bot
                .as_deref()
                .and_then(|inner| inner.language_code.as_deref()),
            Self::Supergroup(val) => val
                .guard_bot
                .as_deref()
                .and_then(|inner| inner.language_code.as_deref()),
            Self::Channel(val) => val
                .guard_bot
                .as_deref()
                .and_then(|inner| inner.language_code.as_deref()),
            Self::Unknown(_) => None,
        }
    }

    /// Helper method for nested field `left_chat_member`.
    #[must_use]
    pub fn left_chat_member(&self) -> Option<&crate::types::User> {
        match self {
            Self::Private(val) => val
                .pinned_message
                .as_deref()
                .and_then(crate::types::Message::left_chat_member),
            Self::Group(val) => val
                .pinned_message
                .as_deref()
                .and_then(crate::types::Message::left_chat_member),
            Self::Supergroup(val) => val
                .pinned_message
                .as_deref()
                .and_then(crate::types::Message::left_chat_member),
            Self::Channel(val) => val
                .pinned_message
                .as_deref()
                .and_then(crate::types::Message::left_chat_member),
            Self::Unknown(_) => None,
        }
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
    pub fn light_theme_main_color(&self) -> Option<i32> {
        match self {
            Self::Private(val) => val
                .unique_gift_colors
                .as_ref()
                .map(|inner| inner.light_theme_main_color),
            Self::Group(val) => val
                .unique_gift_colors
                .as_ref()
                .map(|inner| inner.light_theme_main_color),
            Self::Supergroup(val) => val
                .unique_gift_colors
                .as_ref()
                .map(|inner| inner.light_theme_main_color),
            Self::Channel(val) => val
                .unique_gift_colors
                .as_ref()
                .map(|inner| inner.light_theme_main_color),
            Self::Unknown(_) => None,
        }
    }

    /// Helper method for nested field `light_theme_other_colors`.
    #[must_use]
    pub fn light_theme_other_colors(&self) -> Option<&[i32]> {
        match self {
            Self::Private(val) => val
                .unique_gift_colors
                .as_ref()
                .map(|inner| inner.light_theme_other_colors.as_ref()),
            Self::Group(val) => val
                .unique_gift_colors
                .as_ref()
                .map(|inner| inner.light_theme_other_colors.as_ref()),
            Self::Supergroup(val) => val
                .unique_gift_colors
                .as_ref()
                .map(|inner| inner.light_theme_other_colors.as_ref()),
            Self::Channel(val) => val
                .unique_gift_colors
                .as_ref()
                .map(|inner| inner.light_theme_other_colors.as_ref()),
            Self::Unknown(_) => None,
        }
    }

    /// Helper method for nested field `limited_gifts`.
    #[must_use]
    pub fn limited_gifts(&self) -> bool {
        {
            let inner = self.accepted_gift_types();
            inner.limited_gifts
        }
    }

    /// Helper method for nested field `link_preview_options`.
    #[must_use]
    pub fn link_preview_options(&self) -> Option<&crate::types::LinkPreviewOptions> {
        match self {
            Self::Private(val) => val
                .pinned_message
                .as_deref()
                .and_then(crate::types::Message::link_preview_options),
            Self::Group(val) => val
                .pinned_message
                .as_deref()
                .and_then(crate::types::Message::link_preview_options),
            Self::Supergroup(val) => val
                .pinned_message
                .as_deref()
                .and_then(crate::types::Message::link_preview_options),
            Self::Channel(val) => val
                .pinned_message
                .as_deref()
                .and_then(crate::types::Message::link_preview_options),
            Self::Unknown(_) => None,
        }
    }

    /// Helper method for nested field `live_photo`.
    #[must_use]
    pub fn live_photo(&self) -> Option<&crate::types::LivePhoto> {
        match self {
            Self::Private(val) => val
                .pinned_message
                .as_deref()
                .and_then(crate::types::Message::live_photo),
            Self::Group(val) => val
                .pinned_message
                .as_deref()
                .and_then(crate::types::Message::live_photo),
            Self::Supergroup(val) => val
                .pinned_message
                .as_deref()
                .and_then(crate::types::Message::live_photo),
            Self::Channel(val) => val
                .pinned_message
                .as_deref()
                .and_then(crate::types::Message::live_photo),
            Self::Unknown(_) => None,
        }
    }

    /// Helper method for nested field `managed_bot_created`.
    #[must_use]
    pub fn managed_bot_created(&self) -> Option<&crate::types::ManagedBotCreated> {
        match self {
            Self::Private(val) => val
                .pinned_message
                .as_deref()
                .and_then(crate::types::Message::managed_bot_created),
            Self::Group(val) => val
                .pinned_message
                .as_deref()
                .and_then(crate::types::Message::managed_bot_created),
            Self::Supergroup(val) => val
                .pinned_message
                .as_deref()
                .and_then(crate::types::Message::managed_bot_created),
            Self::Channel(val) => val
                .pinned_message
                .as_deref()
                .and_then(crate::types::Message::managed_bot_created),
            Self::Unknown(_) => None,
        }
    }

    /// Helper method for nested field `media_group_id`.
    #[must_use]
    pub fn media_group_id(&self) -> Option<&str> {
        match self {
            Self::Private(val) => val
                .pinned_message
                .as_deref()
                .and_then(crate::types::Message::media_group_id),
            Self::Group(val) => val
                .pinned_message
                .as_deref()
                .and_then(crate::types::Message::media_group_id),
            Self::Supergroup(val) => val
                .pinned_message
                .as_deref()
                .and_then(crate::types::Message::media_group_id),
            Self::Channel(val) => val
                .pinned_message
                .as_deref()
                .and_then(crate::types::Message::media_group_id),
            Self::Unknown(_) => None,
        }
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
        match self {
            Self::Private(val) => val
                .pinned_message
                .as_deref()
                .and_then(crate::types::Message::message_auto_delete_timer_changed),
            Self::Group(val) => val
                .pinned_message
                .as_deref()
                .and_then(crate::types::Message::message_auto_delete_timer_changed),
            Self::Supergroup(val) => val
                .pinned_message
                .as_deref()
                .and_then(crate::types::Message::message_auto_delete_timer_changed),
            Self::Channel(val) => val
                .pinned_message
                .as_deref()
                .and_then(crate::types::Message::message_auto_delete_timer_changed),
            Self::Unknown(_) => None,
        }
    }

    /// Helper method for nested field `message_id`.
    #[must_use]
    pub fn message_id(&self) -> Option<i64> {
        match self {
            Self::Private(val) => val
                .pinned_message
                .as_deref()
                .map(crate::types::Message::message_id),
            Self::Group(val) => val
                .pinned_message
                .as_deref()
                .map(crate::types::Message::message_id),
            Self::Supergroup(val) => val
                .pinned_message
                .as_deref()
                .map(crate::types::Message::message_id),
            Self::Channel(val) => val
                .pinned_message
                .as_deref()
                .map(crate::types::Message::message_id),
            Self::Unknown(_) => None,
        }
    }

    /// Helper method for nested field `message_thread_id`.
    #[must_use]
    pub fn message_thread_id(&self) -> Option<i64> {
        match self {
            Self::Private(val) => val
                .pinned_message
                .as_deref()
                .and_then(crate::types::Message::message_thread_id),
            Self::Group(val) => val
                .pinned_message
                .as_deref()
                .and_then(crate::types::Message::message_thread_id),
            Self::Supergroup(val) => val
                .pinned_message
                .as_deref()
                .and_then(crate::types::Message::message_thread_id),
            Self::Channel(val) => val
                .pinned_message
                .as_deref()
                .and_then(crate::types::Message::message_thread_id),
            Self::Unknown(_) => None,
        }
    }

    /// Helper method for nested field `migrate_from_chat_id`.
    #[must_use]
    pub fn migrate_from_chat_id(&self) -> Option<i64> {
        match self {
            Self::Private(val) => val
                .pinned_message
                .as_deref()
                .and_then(crate::types::Message::migrate_from_chat_id),
            Self::Group(val) => val
                .pinned_message
                .as_deref()
                .and_then(crate::types::Message::migrate_from_chat_id),
            Self::Supergroup(val) => val
                .pinned_message
                .as_deref()
                .and_then(crate::types::Message::migrate_from_chat_id),
            Self::Channel(val) => val
                .pinned_message
                .as_deref()
                .and_then(crate::types::Message::migrate_from_chat_id),
            Self::Unknown(_) => None,
        }
    }

    /// Helper method for nested field `migrate_to_chat_id`.
    #[must_use]
    pub fn migrate_to_chat_id(&self) -> Option<i64> {
        match self {
            Self::Private(val) => val
                .pinned_message
                .as_deref()
                .and_then(crate::types::Message::migrate_to_chat_id),
            Self::Group(val) => val
                .pinned_message
                .as_deref()
                .and_then(crate::types::Message::migrate_to_chat_id),
            Self::Supergroup(val) => val
                .pinned_message
                .as_deref()
                .and_then(crate::types::Message::migrate_to_chat_id),
            Self::Channel(val) => val
                .pinned_message
                .as_deref()
                .and_then(crate::types::Message::migrate_to_chat_id),
            Self::Unknown(_) => None,
        }
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
        match self {
            Self::Private(val) => val
                .unique_gift_colors
                .as_ref()
                .map(|inner| inner.model_custom_emoji_id.as_ref()),
            Self::Group(val) => val
                .unique_gift_colors
                .as_ref()
                .map(|inner| inner.model_custom_emoji_id.as_ref()),
            Self::Supergroup(val) => val
                .unique_gift_colors
                .as_ref()
                .map(|inner| inner.model_custom_emoji_id.as_ref()),
            Self::Channel(val) => val
                .unique_gift_colors
                .as_ref()
                .map(|inner| inner.model_custom_emoji_id.as_ref()),
            Self::Unknown(_) => None,
        }
    }

    /// Helper method for nested field `month`.
    #[must_use]
    pub fn month(&self) -> Option<u8> {
        match self {
            Self::Private(val) => val.birthdate.as_ref().map(|inner| inner.month),
            _ => None,
        }
    }

    /// Helper method for nested field `name`.
    #[must_use]
    pub fn name(&self) -> Option<&str> {
        match self {
            Self::Private(val) => val.community.as_ref().map(|inner| inner.name.as_ref()),
            Self::Group(val) => val.community.as_ref().map(|inner| inner.name.as_ref()),
            Self::Supergroup(val) => val.community.as_ref().map(|inner| inner.name.as_ref()),
            Self::Channel(val) => val.community.as_ref().map(|inner| inner.name.as_ref()),
            Self::Unknown(_) => None,
        }
    }

    /// Helper method for nested field `new_chat_members`.
    #[must_use]
    pub fn new_chat_members(&self) -> Option<&[crate::types::User]> {
        match self {
            Self::Private(val) => val
                .pinned_message
                .as_deref()
                .and_then(crate::types::Message::new_chat_members),
            Self::Group(val) => val
                .pinned_message
                .as_deref()
                .and_then(crate::types::Message::new_chat_members),
            Self::Supergroup(val) => val
                .pinned_message
                .as_deref()
                .and_then(crate::types::Message::new_chat_members),
            Self::Channel(val) => val
                .pinned_message
                .as_deref()
                .and_then(crate::types::Message::new_chat_members),
            Self::Unknown(_) => None,
        }
    }

    /// Helper method for nested field `new_chat_photo`.
    #[must_use]
    pub fn new_chat_photo(&self) -> Option<&[crate::types::PhotoSize]> {
        match self {
            Self::Private(val) => val
                .pinned_message
                .as_deref()
                .and_then(crate::types::Message::new_chat_photo),
            Self::Group(val) => val
                .pinned_message
                .as_deref()
                .and_then(crate::types::Message::new_chat_photo),
            Self::Supergroup(val) => val
                .pinned_message
                .as_deref()
                .and_then(crate::types::Message::new_chat_photo),
            Self::Channel(val) => val
                .pinned_message
                .as_deref()
                .and_then(crate::types::Message::new_chat_photo),
            Self::Unknown(_) => None,
        }
    }

    /// Helper method for nested field `new_chat_title`.
    #[must_use]
    pub fn new_chat_title(&self) -> Option<&str> {
        match self {
            Self::Private(val) => val
                .pinned_message
                .as_deref()
                .and_then(crate::types::Message::new_chat_title),
            Self::Group(val) => val
                .pinned_message
                .as_deref()
                .and_then(crate::types::Message::new_chat_title),
            Self::Supergroup(val) => val
                .pinned_message
                .as_deref()
                .and_then(crate::types::Message::new_chat_title),
            Self::Channel(val) => val
                .pinned_message
                .as_deref()
                .and_then(crate::types::Message::new_chat_title),
            Self::Unknown(_) => None,
        }
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
        match self {
            Self::Private(val) => val
                .pinned_message
                .as_deref()
                .and_then(crate::types::Message::paid_media),
            Self::Group(val) => val
                .pinned_message
                .as_deref()
                .and_then(crate::types::Message::paid_media),
            Self::Supergroup(val) => val
                .pinned_message
                .as_deref()
                .and_then(crate::types::Message::paid_media),
            Self::Channel(val) => val
                .pinned_message
                .as_deref()
                .and_then(crate::types::Message::paid_media),
            Self::Unknown(_) => None,
        }
    }

    /// Helper method for nested field `paid_message_price_changed`.
    #[must_use]
    pub fn paid_message_price_changed(&self) -> Option<&crate::types::PaidMessagePriceChanged> {
        match self {
            Self::Private(val) => val
                .pinned_message
                .as_deref()
                .and_then(crate::types::Message::paid_message_price_changed),
            Self::Group(val) => val
                .pinned_message
                .as_deref()
                .and_then(crate::types::Message::paid_message_price_changed),
            Self::Supergroup(val) => val
                .pinned_message
                .as_deref()
                .and_then(crate::types::Message::paid_message_price_changed),
            Self::Channel(val) => val
                .pinned_message
                .as_deref()
                .and_then(crate::types::Message::paid_message_price_changed),
            Self::Unknown(_) => None,
        }
    }

    /// Helper method for nested field `paid_star_count`.
    #[must_use]
    pub fn paid_star_count(&self) -> Option<i64> {
        match self {
            Self::Private(val) => val
                .pinned_message
                .as_deref()
                .and_then(crate::types::Message::paid_star_count),
            Self::Group(val) => val
                .pinned_message
                .as_deref()
                .and_then(crate::types::Message::paid_star_count),
            Self::Supergroup(val) => val
                .pinned_message
                .as_deref()
                .and_then(crate::types::Message::paid_star_count),
            Self::Channel(val) => val
                .pinned_message
                .as_deref()
                .and_then(crate::types::Message::paid_star_count),
            Self::Unknown(_) => None,
        }
    }

    /// Helper method for nested field `passport_data`.
    #[must_use]
    pub fn passport_data(&self) -> Option<&crate::types::PassportData> {
        match self {
            Self::Private(val) => val
                .pinned_message
                .as_deref()
                .and_then(crate::types::Message::passport_data),
            Self::Group(val) => val
                .pinned_message
                .as_deref()
                .and_then(crate::types::Message::passport_data),
            Self::Supergroup(val) => val
                .pinned_message
                .as_deref()
                .and_then(crate::types::Message::passport_data),
            Self::Channel(val) => val
                .pinned_message
                .as_deref()
                .and_then(crate::types::Message::passport_data),
            Self::Unknown(_) => None,
        }
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
        match self {
            Self::Private(val) => val
                .pinned_message
                .as_deref()
                .and_then(crate::types::Message::poll),
            Self::Group(val) => val
                .pinned_message
                .as_deref()
                .and_then(crate::types::Message::poll),
            Self::Supergroup(val) => val
                .pinned_message
                .as_deref()
                .and_then(crate::types::Message::poll),
            Self::Channel(val) => val
                .pinned_message
                .as_deref()
                .and_then(crate::types::Message::poll),
            Self::Unknown(_) => None,
        }
    }

    /// Helper method for nested field `poll_option_added`.
    #[must_use]
    pub fn poll_option_added(&self) -> Option<&crate::types::PollOptionAdded> {
        match self {
            Self::Private(val) => val
                .pinned_message
                .as_deref()
                .and_then(crate::types::Message::poll_option_added),
            Self::Group(val) => val
                .pinned_message
                .as_deref()
                .and_then(crate::types::Message::poll_option_added),
            Self::Supergroup(val) => val
                .pinned_message
                .as_deref()
                .and_then(crate::types::Message::poll_option_added),
            Self::Channel(val) => val
                .pinned_message
                .as_deref()
                .and_then(crate::types::Message::poll_option_added),
            Self::Unknown(_) => None,
        }
    }

    /// Helper method for nested field `poll_option_deleted`.
    #[must_use]
    pub fn poll_option_deleted(&self) -> Option<&crate::types::PollOptionDeleted> {
        match self {
            Self::Private(val) => val
                .pinned_message
                .as_deref()
                .and_then(crate::types::Message::poll_option_deleted),
            Self::Group(val) => val
                .pinned_message
                .as_deref()
                .and_then(crate::types::Message::poll_option_deleted),
            Self::Supergroup(val) => val
                .pinned_message
                .as_deref()
                .and_then(crate::types::Message::poll_option_deleted),
            Self::Channel(val) => val
                .pinned_message
                .as_deref()
                .and_then(crate::types::Message::poll_option_deleted),
            Self::Unknown(_) => None,
        }
    }

    /// Helper method for nested field `premium_subscription`.
    #[must_use]
    pub fn premium_subscription(&self) -> bool {
        {
            let inner = self.accepted_gift_types();
            inner.premium_subscription
        }
    }

    /// Helper method for nested field `proximity_alert_triggered`.
    #[must_use]
    pub fn proximity_alert_triggered(&self) -> Option<&crate::types::ProximityAlertTriggered> {
        match self {
            Self::Private(val) => val
                .pinned_message
                .as_deref()
                .and_then(crate::types::Message::proximity_alert_triggered),
            Self::Group(val) => val
                .pinned_message
                .as_deref()
                .and_then(crate::types::Message::proximity_alert_triggered),
            Self::Supergroup(val) => val
                .pinned_message
                .as_deref()
                .and_then(crate::types::Message::proximity_alert_triggered),
            Self::Channel(val) => val
                .pinned_message
                .as_deref()
                .and_then(crate::types::Message::proximity_alert_triggered),
            Self::Unknown(_) => None,
        }
    }

    /// Helper method for nested field `quote`.
    #[must_use]
    pub fn quote(&self) -> Option<&crate::types::TextQuote> {
        match self {
            Self::Private(val) => val
                .pinned_message
                .as_deref()
                .and_then(crate::types::Message::quote),
            Self::Group(val) => val
                .pinned_message
                .as_deref()
                .and_then(crate::types::Message::quote),
            Self::Supergroup(val) => val
                .pinned_message
                .as_deref()
                .and_then(crate::types::Message::quote),
            Self::Channel(val) => val
                .pinned_message
                .as_deref()
                .and_then(crate::types::Message::quote),
            Self::Unknown(_) => None,
        }
    }

    /// Helper method for nested field `receiver_user`.
    #[must_use]
    pub fn receiver_user(&self) -> Option<&crate::types::User> {
        match self {
            Self::Private(val) => val
                .pinned_message
                .as_deref()
                .and_then(crate::types::Message::receiver_user),
            Self::Group(val) => val
                .pinned_message
                .as_deref()
                .and_then(crate::types::Message::receiver_user),
            Self::Supergroup(val) => val
                .pinned_message
                .as_deref()
                .and_then(crate::types::Message::receiver_user),
            Self::Channel(val) => val
                .pinned_message
                .as_deref()
                .and_then(crate::types::Message::receiver_user),
            Self::Unknown(_) => None,
        }
    }

    /// Helper method for nested field `refunded_payment`.
    #[must_use]
    pub fn refunded_payment(&self) -> Option<&crate::types::RefundedPayment> {
        match self {
            Self::Private(val) => val
                .pinned_message
                .as_deref()
                .and_then(crate::types::Message::refunded_payment),
            Self::Group(val) => val
                .pinned_message
                .as_deref()
                .and_then(crate::types::Message::refunded_payment),
            Self::Supergroup(val) => val
                .pinned_message
                .as_deref()
                .and_then(crate::types::Message::refunded_payment),
            Self::Channel(val) => val
                .pinned_message
                .as_deref()
                .and_then(crate::types::Message::refunded_payment),
            Self::Unknown(_) => None,
        }
    }

    /// Helper method for nested field `reply_markup`.
    #[must_use]
    pub fn reply_markup(&self) -> Option<&crate::types::InlineKeyboardMarkup> {
        match self {
            Self::Private(val) => val
                .pinned_message
                .as_deref()
                .and_then(crate::types::Message::reply_markup),
            Self::Group(val) => val
                .pinned_message
                .as_deref()
                .and_then(crate::types::Message::reply_markup),
            Self::Supergroup(val) => val
                .pinned_message
                .as_deref()
                .and_then(crate::types::Message::reply_markup),
            Self::Channel(val) => val
                .pinned_message
                .as_deref()
                .and_then(crate::types::Message::reply_markup),
            Self::Unknown(_) => None,
        }
    }

    /// Helper method for nested field `reply_to_checklist_task_id`.
    #[must_use]
    pub fn reply_to_checklist_task_id(&self) -> Option<i64> {
        match self {
            Self::Private(val) => val
                .pinned_message
                .as_deref()
                .and_then(crate::types::Message::reply_to_checklist_task_id),
            Self::Group(val) => val
                .pinned_message
                .as_deref()
                .and_then(crate::types::Message::reply_to_checklist_task_id),
            Self::Supergroup(val) => val
                .pinned_message
                .as_deref()
                .and_then(crate::types::Message::reply_to_checklist_task_id),
            Self::Channel(val) => val
                .pinned_message
                .as_deref()
                .and_then(crate::types::Message::reply_to_checklist_task_id),
            Self::Unknown(_) => None,
        }
    }

    /// Helper method for nested field `reply_to_message`.
    #[must_use]
    pub fn reply_to_message(&self) -> Option<&crate::types::Message> {
        match self {
            Self::Private(val) => val
                .pinned_message
                .as_deref()
                .and_then(crate::types::Message::reply_to_message),
            Self::Group(val) => val
                .pinned_message
                .as_deref()
                .and_then(crate::types::Message::reply_to_message),
            Self::Supergroup(val) => val
                .pinned_message
                .as_deref()
                .and_then(crate::types::Message::reply_to_message),
            Self::Channel(val) => val
                .pinned_message
                .as_deref()
                .and_then(crate::types::Message::reply_to_message),
            Self::Unknown(_) => None,
        }
    }

    /// Helper method for nested field `reply_to_poll_option_id`.
    #[must_use]
    pub fn reply_to_poll_option_id(&self) -> Option<&str> {
        match self {
            Self::Private(val) => val
                .pinned_message
                .as_deref()
                .and_then(crate::types::Message::reply_to_poll_option_id),
            Self::Group(val) => val
                .pinned_message
                .as_deref()
                .and_then(crate::types::Message::reply_to_poll_option_id),
            Self::Supergroup(val) => val
                .pinned_message
                .as_deref()
                .and_then(crate::types::Message::reply_to_poll_option_id),
            Self::Channel(val) => val
                .pinned_message
                .as_deref()
                .and_then(crate::types::Message::reply_to_poll_option_id),
            Self::Unknown(_) => None,
        }
    }

    /// Helper method for nested field `reply_to_story`.
    #[must_use]
    pub fn reply_to_story(&self) -> Option<&crate::types::Story> {
        match self {
            Self::Private(val) => val
                .pinned_message
                .as_deref()
                .and_then(crate::types::Message::reply_to_story),
            Self::Group(val) => val
                .pinned_message
                .as_deref()
                .and_then(crate::types::Message::reply_to_story),
            Self::Supergroup(val) => val
                .pinned_message
                .as_deref()
                .and_then(crate::types::Message::reply_to_story),
            Self::Channel(val) => val
                .pinned_message
                .as_deref()
                .and_then(crate::types::Message::reply_to_story),
            Self::Unknown(_) => None,
        }
    }

    /// Helper method for nested field `rich_message`.
    #[must_use]
    pub fn rich_message(&self) -> Option<&crate::types::RichMessage> {
        match self {
            Self::Private(val) => val
                .pinned_message
                .as_deref()
                .and_then(crate::types::Message::rich_message),
            Self::Group(val) => val
                .pinned_message
                .as_deref()
                .and_then(crate::types::Message::rich_message),
            Self::Supergroup(val) => val
                .pinned_message
                .as_deref()
                .and_then(crate::types::Message::rich_message),
            Self::Channel(val) => val
                .pinned_message
                .as_deref()
                .and_then(crate::types::Message::rich_message),
            Self::Unknown(_) => None,
        }
    }

    /// Helper method for nested field `sender_boost_count`.
    #[must_use]
    pub fn sender_boost_count(&self) -> Option<i64> {
        match self {
            Self::Private(val) => val
                .pinned_message
                .as_deref()
                .and_then(crate::types::Message::sender_boost_count),
            Self::Group(val) => val
                .pinned_message
                .as_deref()
                .and_then(crate::types::Message::sender_boost_count),
            Self::Supergroup(val) => val
                .pinned_message
                .as_deref()
                .and_then(crate::types::Message::sender_boost_count),
            Self::Channel(val) => val
                .pinned_message
                .as_deref()
                .and_then(crate::types::Message::sender_boost_count),
            Self::Unknown(_) => None,
        }
    }

    /// Helper method for nested field `sender_business_bot`.
    #[must_use]
    pub fn sender_business_bot(&self) -> Option<&crate::types::User> {
        match self {
            Self::Private(val) => val
                .pinned_message
                .as_deref()
                .and_then(crate::types::Message::sender_business_bot),
            Self::Group(val) => val
                .pinned_message
                .as_deref()
                .and_then(crate::types::Message::sender_business_bot),
            Self::Supergroup(val) => val
                .pinned_message
                .as_deref()
                .and_then(crate::types::Message::sender_business_bot),
            Self::Channel(val) => val
                .pinned_message
                .as_deref()
                .and_then(crate::types::Message::sender_business_bot),
            Self::Unknown(_) => None,
        }
    }

    /// Helper method for nested field `sender_chat`.
    #[must_use]
    pub fn sender_chat(&self) -> Option<&crate::types::Chat> {
        match self {
            Self::Private(val) => val
                .pinned_message
                .as_deref()
                .and_then(crate::types::Message::sender_chat),
            Self::Group(val) => val
                .pinned_message
                .as_deref()
                .and_then(crate::types::Message::sender_chat),
            Self::Supergroup(val) => val
                .pinned_message
                .as_deref()
                .and_then(crate::types::Message::sender_chat),
            Self::Channel(val) => val
                .pinned_message
                .as_deref()
                .and_then(crate::types::Message::sender_chat),
            Self::Unknown(_) => None,
        }
    }

    /// Helper method for nested field `sender_tag`.
    #[must_use]
    pub fn sender_tag(&self) -> Option<&str> {
        match self {
            Self::Private(val) => val
                .pinned_message
                .as_deref()
                .and_then(crate::types::Message::sender_tag),
            Self::Group(val) => val
                .pinned_message
                .as_deref()
                .and_then(crate::types::Message::sender_tag),
            Self::Supergroup(val) => val
                .pinned_message
                .as_deref()
                .and_then(crate::types::Message::sender_tag),
            Self::Channel(val) => val
                .pinned_message
                .as_deref()
                .and_then(crate::types::Message::sender_tag),
            Self::Unknown(_) => None,
        }
    }

    /// Helper method for nested field `show_caption_above_media`.
    #[must_use]
    pub fn show_caption_above_media(&self) -> Option<bool> {
        match self {
            Self::Private(val) => val
                .pinned_message
                .as_deref()
                .and_then(crate::types::Message::show_caption_above_media),
            Self::Group(val) => val
                .pinned_message
                .as_deref()
                .and_then(crate::types::Message::show_caption_above_media),
            Self::Supergroup(val) => val
                .pinned_message
                .as_deref()
                .and_then(crate::types::Message::show_caption_above_media),
            Self::Channel(val) => val
                .pinned_message
                .as_deref()
                .and_then(crate::types::Message::show_caption_above_media),
            Self::Unknown(_) => None,
        }
    }

    /// Helper method for nested field `small_file_id`.
    #[must_use]
    pub fn small_file_id(&self) -> Option<&str> {
        match self {
            Self::Private(val) => val.photo.as_ref().map(|inner| inner.small_file_id.as_ref()),
            Self::Group(val) => val.photo.as_ref().map(|inner| inner.small_file_id.as_ref()),
            Self::Supergroup(val) => val.photo.as_ref().map(|inner| inner.small_file_id.as_ref()),
            Self::Channel(val) => val.photo.as_ref().map(|inner| inner.small_file_id.as_ref()),
            Self::Unknown(_) => None,
        }
    }

    /// Helper method for nested field `small_file_unique_id`.
    #[must_use]
    pub fn small_file_unique_id(&self) -> Option<&str> {
        match self {
            Self::Private(val) => val
                .photo
                .as_ref()
                .map(|inner| inner.small_file_unique_id.as_ref()),
            Self::Group(val) => val
                .photo
                .as_ref()
                .map(|inner| inner.small_file_unique_id.as_ref()),
            Self::Supergroup(val) => val
                .photo
                .as_ref()
                .map(|inner| inner.small_file_unique_id.as_ref()),
            Self::Channel(val) => val
                .photo
                .as_ref()
                .map(|inner| inner.small_file_unique_id.as_ref()),
            Self::Unknown(_) => None,
        }
    }

    /// Helper method for nested field `story`.
    #[must_use]
    pub fn story(&self) -> Option<&crate::types::Story> {
        match self {
            Self::Private(val) => val
                .pinned_message
                .as_deref()
                .and_then(crate::types::Message::story),
            Self::Group(val) => val
                .pinned_message
                .as_deref()
                .and_then(crate::types::Message::story),
            Self::Supergroup(val) => val
                .pinned_message
                .as_deref()
                .and_then(crate::types::Message::story),
            Self::Channel(val) => val
                .pinned_message
                .as_deref()
                .and_then(crate::types::Message::story),
            Self::Unknown(_) => None,
        }
    }

    /// Helper method for nested field `successful_payment`.
    #[must_use]
    pub fn successful_payment(&self) -> Option<&crate::types::SuccessfulPayment> {
        match self {
            Self::Private(val) => val
                .pinned_message
                .as_deref()
                .and_then(crate::types::Message::successful_payment),
            Self::Group(val) => val
                .pinned_message
                .as_deref()
                .and_then(crate::types::Message::successful_payment),
            Self::Supergroup(val) => val
                .pinned_message
                .as_deref()
                .and_then(crate::types::Message::successful_payment),
            Self::Channel(val) => val
                .pinned_message
                .as_deref()
                .and_then(crate::types::Message::successful_payment),
            Self::Unknown(_) => None,
        }
    }

    /// Helper method for nested field `suggested_post_approval_failed`.
    #[must_use]
    pub fn suggested_post_approval_failed(
        &self,
    ) -> Option<&crate::types::SuggestedPostApprovalFailed> {
        match self {
            Self::Private(val) => val
                .pinned_message
                .as_deref()
                .and_then(crate::types::Message::suggested_post_approval_failed),
            Self::Group(val) => val
                .pinned_message
                .as_deref()
                .and_then(crate::types::Message::suggested_post_approval_failed),
            Self::Supergroup(val) => val
                .pinned_message
                .as_deref()
                .and_then(crate::types::Message::suggested_post_approval_failed),
            Self::Channel(val) => val
                .pinned_message
                .as_deref()
                .and_then(crate::types::Message::suggested_post_approval_failed),
            Self::Unknown(_) => None,
        }
    }

    /// Helper method for nested field `suggested_post_approved`.
    #[must_use]
    pub fn suggested_post_approved(&self) -> Option<&crate::types::SuggestedPostApproved> {
        match self {
            Self::Private(val) => val
                .pinned_message
                .as_deref()
                .and_then(crate::types::Message::suggested_post_approved),
            Self::Group(val) => val
                .pinned_message
                .as_deref()
                .and_then(crate::types::Message::suggested_post_approved),
            Self::Supergroup(val) => val
                .pinned_message
                .as_deref()
                .and_then(crate::types::Message::suggested_post_approved),
            Self::Channel(val) => val
                .pinned_message
                .as_deref()
                .and_then(crate::types::Message::suggested_post_approved),
            Self::Unknown(_) => None,
        }
    }

    /// Helper method for nested field `suggested_post_declined`.
    #[must_use]
    pub fn suggested_post_declined(&self) -> Option<&crate::types::SuggestedPostDeclined> {
        match self {
            Self::Private(val) => val
                .pinned_message
                .as_deref()
                .and_then(crate::types::Message::suggested_post_declined),
            Self::Group(val) => val
                .pinned_message
                .as_deref()
                .and_then(crate::types::Message::suggested_post_declined),
            Self::Supergroup(val) => val
                .pinned_message
                .as_deref()
                .and_then(crate::types::Message::suggested_post_declined),
            Self::Channel(val) => val
                .pinned_message
                .as_deref()
                .and_then(crate::types::Message::suggested_post_declined),
            Self::Unknown(_) => None,
        }
    }

    /// Helper method for nested field `suggested_post_info`.
    #[must_use]
    pub fn suggested_post_info(&self) -> Option<&crate::types::SuggestedPostInfo> {
        match self {
            Self::Private(val) => val
                .pinned_message
                .as_deref()
                .and_then(crate::types::Message::suggested_post_info),
            Self::Group(val) => val
                .pinned_message
                .as_deref()
                .and_then(crate::types::Message::suggested_post_info),
            Self::Supergroup(val) => val
                .pinned_message
                .as_deref()
                .and_then(crate::types::Message::suggested_post_info),
            Self::Channel(val) => val
                .pinned_message
                .as_deref()
                .and_then(crate::types::Message::suggested_post_info),
            Self::Unknown(_) => None,
        }
    }

    /// Helper method for nested field `suggested_post_paid`.
    #[must_use]
    pub fn suggested_post_paid(&self) -> Option<&crate::types::SuggestedPostPaid> {
        match self {
            Self::Private(val) => val
                .pinned_message
                .as_deref()
                .and_then(crate::types::Message::suggested_post_paid),
            Self::Group(val) => val
                .pinned_message
                .as_deref()
                .and_then(crate::types::Message::suggested_post_paid),
            Self::Supergroup(val) => val
                .pinned_message
                .as_deref()
                .and_then(crate::types::Message::suggested_post_paid),
            Self::Channel(val) => val
                .pinned_message
                .as_deref()
                .and_then(crate::types::Message::suggested_post_paid),
            Self::Unknown(_) => None,
        }
    }

    /// Helper method for nested field `suggested_post_refunded`.
    #[must_use]
    pub fn suggested_post_refunded(&self) -> Option<&crate::types::SuggestedPostRefunded> {
        match self {
            Self::Private(val) => val
                .pinned_message
                .as_deref()
                .and_then(crate::types::Message::suggested_post_refunded),
            Self::Group(val) => val
                .pinned_message
                .as_deref()
                .and_then(crate::types::Message::suggested_post_refunded),
            Self::Supergroup(val) => val
                .pinned_message
                .as_deref()
                .and_then(crate::types::Message::suggested_post_refunded),
            Self::Channel(val) => val
                .pinned_message
                .as_deref()
                .and_then(crate::types::Message::suggested_post_refunded),
            Self::Unknown(_) => None,
        }
    }

    /// Helper method for nested field `supergroup_chat_created`.
    #[must_use]
    pub fn supergroup_chat_created(&self) -> Option<bool> {
        match self {
            Self::Private(val) => val
                .pinned_message
                .as_deref()
                .and_then(crate::types::Message::supergroup_chat_created),
            Self::Group(val) => val
                .pinned_message
                .as_deref()
                .and_then(crate::types::Message::supergroup_chat_created),
            Self::Supergroup(val) => val
                .pinned_message
                .as_deref()
                .and_then(crate::types::Message::supergroup_chat_created),
            Self::Channel(val) => val
                .pinned_message
                .as_deref()
                .and_then(crate::types::Message::supergroup_chat_created),
            Self::Unknown(_) => None,
        }
    }

    /// Helper method for nested field `supports_guest_queries`.
    #[must_use]
    pub fn supports_guest_queries(&self) -> Option<bool> {
        match self {
            Self::Private(val) => val
                .guard_bot
                .as_deref()
                .and_then(|inner| inner.supports_guest_queries),
            Self::Group(val) => val
                .guard_bot
                .as_deref()
                .and_then(|inner| inner.supports_guest_queries),
            Self::Supergroup(val) => val
                .guard_bot
                .as_deref()
                .and_then(|inner| inner.supports_guest_queries),
            Self::Channel(val) => val
                .guard_bot
                .as_deref()
                .and_then(|inner| inner.supports_guest_queries),
            Self::Unknown(_) => None,
        }
    }

    /// Helper method for nested field `supports_inline_queries`.
    #[must_use]
    pub fn supports_inline_queries(&self) -> Option<bool> {
        match self {
            Self::Private(val) => val
                .guard_bot
                .as_deref()
                .and_then(|inner| inner.supports_inline_queries),
            Self::Group(val) => val
                .guard_bot
                .as_deref()
                .and_then(|inner| inner.supports_inline_queries),
            Self::Supergroup(val) => val
                .guard_bot
                .as_deref()
                .and_then(|inner| inner.supports_inline_queries),
            Self::Channel(val) => val
                .guard_bot
                .as_deref()
                .and_then(|inner| inner.supports_inline_queries),
            Self::Unknown(_) => None,
        }
    }

    /// Helper method for nested field `supports_join_request_queries`.
    #[must_use]
    pub fn supports_join_request_queries(&self) -> Option<bool> {
        match self {
            Self::Private(val) => val
                .guard_bot
                .as_deref()
                .and_then(|inner| inner.supports_join_request_queries),
            Self::Group(val) => val
                .guard_bot
                .as_deref()
                .and_then(|inner| inner.supports_join_request_queries),
            Self::Supergroup(val) => val
                .guard_bot
                .as_deref()
                .and_then(|inner| inner.supports_join_request_queries),
            Self::Channel(val) => val
                .guard_bot
                .as_deref()
                .and_then(|inner| inner.supports_join_request_queries),
            Self::Unknown(_) => None,
        }
    }

    /// Helper method for nested field `symbol_custom_emoji_id`.
    #[must_use]
    pub fn symbol_custom_emoji_id(&self) -> Option<&str> {
        match self {
            Self::Private(val) => val
                .unique_gift_colors
                .as_ref()
                .map(|inner| inner.symbol_custom_emoji_id.as_ref()),
            Self::Group(val) => val
                .unique_gift_colors
                .as_ref()
                .map(|inner| inner.symbol_custom_emoji_id.as_ref()),
            Self::Supergroup(val) => val
                .unique_gift_colors
                .as_ref()
                .map(|inner| inner.symbol_custom_emoji_id.as_ref()),
            Self::Channel(val) => val
                .unique_gift_colors
                .as_ref()
                .map(|inner| inner.symbol_custom_emoji_id.as_ref()),
            Self::Unknown(_) => None,
        }
    }

    /// Helper method for nested field `text`.
    #[must_use]
    pub fn text(&self) -> Option<&str> {
        match self {
            Self::Private(val) => val
                .pinned_message
                .as_deref()
                .and_then(crate::types::Message::text),
            Self::Group(val) => val
                .pinned_message
                .as_deref()
                .and_then(crate::types::Message::text),
            Self::Supergroup(val) => val
                .pinned_message
                .as_deref()
                .and_then(crate::types::Message::text),
            Self::Channel(val) => val
                .pinned_message
                .as_deref()
                .and_then(crate::types::Message::text),
            Self::Unknown(_) => None,
        }
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
        match self {
            Self::Private(val) => val
                .pinned_message
                .as_deref()
                .and_then(crate::types::Message::unique_gift),
            Self::Group(val) => val
                .pinned_message
                .as_deref()
                .and_then(crate::types::Message::unique_gift),
            Self::Supergroup(val) => val
                .pinned_message
                .as_deref()
                .and_then(crate::types::Message::unique_gift),
            Self::Channel(val) => val
                .pinned_message
                .as_deref()
                .and_then(crate::types::Message::unique_gift),
            Self::Unknown(_) => None,
        }
    }

    /// Helper method for nested field `unique_gifts`.
    #[must_use]
    pub fn unique_gifts(&self) -> bool {
        {
            let inner = self.accepted_gift_types();
            inner.unique_gifts
        }
    }

    /// Helper method for nested field `unlimited_gifts`.
    #[must_use]
    pub fn unlimited_gifts(&self) -> bool {
        {
            let inner = self.accepted_gift_types();
            inner.unlimited_gifts
        }
    }

    /// Helper method for nested field `users_shared`.
    #[must_use]
    pub fn users_shared(&self) -> Option<&crate::types::UsersShared> {
        match self {
            Self::Private(val) => val
                .pinned_message
                .as_deref()
                .and_then(crate::types::Message::users_shared),
            Self::Group(val) => val
                .pinned_message
                .as_deref()
                .and_then(crate::types::Message::users_shared),
            Self::Supergroup(val) => val
                .pinned_message
                .as_deref()
                .and_then(crate::types::Message::users_shared),
            Self::Channel(val) => val
                .pinned_message
                .as_deref()
                .and_then(crate::types::Message::users_shared),
            Self::Unknown(_) => None,
        }
    }

    /// Helper method for nested field `venue`.
    #[must_use]
    pub fn venue(&self) -> Option<&crate::types::Venue> {
        match self {
            Self::Private(val) => val
                .pinned_message
                .as_deref()
                .and_then(crate::types::Message::venue),
            Self::Group(val) => val
                .pinned_message
                .as_deref()
                .and_then(crate::types::Message::venue),
            Self::Supergroup(val) => val
                .pinned_message
                .as_deref()
                .and_then(crate::types::Message::venue),
            Self::Channel(val) => val
                .pinned_message
                .as_deref()
                .and_then(crate::types::Message::venue),
            Self::Unknown(_) => None,
        }
    }

    /// Helper method for nested field `via_bot`.
    #[must_use]
    pub fn via_bot(&self) -> Option<&crate::types::User> {
        match self {
            Self::Private(val) => val
                .pinned_message
                .as_deref()
                .and_then(crate::types::Message::via_bot),
            Self::Group(val) => val
                .pinned_message
                .as_deref()
                .and_then(crate::types::Message::via_bot),
            Self::Supergroup(val) => val
                .pinned_message
                .as_deref()
                .and_then(crate::types::Message::via_bot),
            Self::Channel(val) => val
                .pinned_message
                .as_deref()
                .and_then(crate::types::Message::via_bot),
            Self::Unknown(_) => None,
        }
    }

    /// Helper method for nested field `video`.
    #[must_use]
    pub fn video(&self) -> Option<&crate::types::Video> {
        match self {
            Self::Private(val) => val
                .pinned_message
                .as_deref()
                .and_then(crate::types::Message::video),
            Self::Group(val) => val
                .pinned_message
                .as_deref()
                .and_then(crate::types::Message::video),
            Self::Supergroup(val) => val
                .pinned_message
                .as_deref()
                .and_then(crate::types::Message::video),
            Self::Channel(val) => val
                .pinned_message
                .as_deref()
                .and_then(crate::types::Message::video),
            Self::Unknown(_) => None,
        }
    }

    /// Helper method for nested field `video_chat_ended`.
    #[must_use]
    pub fn video_chat_ended(&self) -> Option<&crate::types::VideoChatEnded> {
        match self {
            Self::Private(val) => val
                .pinned_message
                .as_deref()
                .and_then(crate::types::Message::video_chat_ended),
            Self::Group(val) => val
                .pinned_message
                .as_deref()
                .and_then(crate::types::Message::video_chat_ended),
            Self::Supergroup(val) => val
                .pinned_message
                .as_deref()
                .and_then(crate::types::Message::video_chat_ended),
            Self::Channel(val) => val
                .pinned_message
                .as_deref()
                .and_then(crate::types::Message::video_chat_ended),
            Self::Unknown(_) => None,
        }
    }

    /// Helper method for nested field `video_chat_participants_invited`.
    #[must_use]
    pub fn video_chat_participants_invited(
        &self,
    ) -> Option<&crate::types::VideoChatParticipantsInvited> {
        match self {
            Self::Private(val) => val
                .pinned_message
                .as_deref()
                .and_then(crate::types::Message::video_chat_participants_invited),
            Self::Group(val) => val
                .pinned_message
                .as_deref()
                .and_then(crate::types::Message::video_chat_participants_invited),
            Self::Supergroup(val) => val
                .pinned_message
                .as_deref()
                .and_then(crate::types::Message::video_chat_participants_invited),
            Self::Channel(val) => val
                .pinned_message
                .as_deref()
                .and_then(crate::types::Message::video_chat_participants_invited),
            Self::Unknown(_) => None,
        }
    }

    /// Helper method for nested field `video_chat_scheduled`.
    #[must_use]
    pub fn video_chat_scheduled(&self) -> Option<&crate::types::VideoChatScheduled> {
        match self {
            Self::Private(val) => val
                .pinned_message
                .as_deref()
                .and_then(crate::types::Message::video_chat_scheduled),
            Self::Group(val) => val
                .pinned_message
                .as_deref()
                .and_then(crate::types::Message::video_chat_scheduled),
            Self::Supergroup(val) => val
                .pinned_message
                .as_deref()
                .and_then(crate::types::Message::video_chat_scheduled),
            Self::Channel(val) => val
                .pinned_message
                .as_deref()
                .and_then(crate::types::Message::video_chat_scheduled),
            Self::Unknown(_) => None,
        }
    }

    /// Helper method for nested field `video_chat_started`.
    #[must_use]
    pub fn video_chat_started(&self) -> Option<&crate::types::VideoChatStarted> {
        match self {
            Self::Private(val) => val
                .pinned_message
                .as_deref()
                .and_then(crate::types::Message::video_chat_started),
            Self::Group(val) => val
                .pinned_message
                .as_deref()
                .and_then(crate::types::Message::video_chat_started),
            Self::Supergroup(val) => val
                .pinned_message
                .as_deref()
                .and_then(crate::types::Message::video_chat_started),
            Self::Channel(val) => val
                .pinned_message
                .as_deref()
                .and_then(crate::types::Message::video_chat_started),
            Self::Unknown(_) => None,
        }
    }

    /// Helper method for nested field `video_note`.
    #[must_use]
    pub fn video_note(&self) -> Option<&crate::types::VideoNote> {
        match self {
            Self::Private(val) => val
                .pinned_message
                .as_deref()
                .and_then(crate::types::Message::video_note),
            Self::Group(val) => val
                .pinned_message
                .as_deref()
                .and_then(crate::types::Message::video_note),
            Self::Supergroup(val) => val
                .pinned_message
                .as_deref()
                .and_then(crate::types::Message::video_note),
            Self::Channel(val) => val
                .pinned_message
                .as_deref()
                .and_then(crate::types::Message::video_note),
            Self::Unknown(_) => None,
        }
    }

    /// Helper method for nested field `voice`.
    #[must_use]
    pub fn voice(&self) -> Option<&crate::types::Voice> {
        match self {
            Self::Private(val) => val
                .pinned_message
                .as_deref()
                .and_then(crate::types::Message::voice),
            Self::Group(val) => val
                .pinned_message
                .as_deref()
                .and_then(crate::types::Message::voice),
            Self::Supergroup(val) => val
                .pinned_message
                .as_deref()
                .and_then(crate::types::Message::voice),
            Self::Channel(val) => val
                .pinned_message
                .as_deref()
                .and_then(crate::types::Message::voice),
            Self::Unknown(_) => None,
        }
    }

    /// Helper method for nested field `web_app_data`.
    #[must_use]
    pub fn web_app_data(&self) -> Option<&crate::types::WebAppData> {
        match self {
            Self::Private(val) => val
                .pinned_message
                .as_deref()
                .and_then(crate::types::Message::web_app_data),
            Self::Group(val) => val
                .pinned_message
                .as_deref()
                .and_then(crate::types::Message::web_app_data),
            Self::Supergroup(val) => val
                .pinned_message
                .as_deref()
                .and_then(crate::types::Message::web_app_data),
            Self::Channel(val) => val
                .pinned_message
                .as_deref()
                .and_then(crate::types::Message::web_app_data),
            Self::Unknown(_) => None,
        }
    }

    /// Helper method for nested field `write_access_allowed`.
    #[must_use]
    pub fn write_access_allowed(&self) -> Option<&crate::types::WriteAccessAllowed> {
        match self {
            Self::Private(val) => val
                .pinned_message
                .as_deref()
                .and_then(crate::types::Message::write_access_allowed),
            Self::Group(val) => val
                .pinned_message
                .as_deref()
                .and_then(crate::types::Message::write_access_allowed),
            Self::Supergroup(val) => val
                .pinned_message
                .as_deref()
                .and_then(crate::types::Message::write_access_allowed),
            Self::Channel(val) => val
                .pinned_message
                .as_deref()
                .and_then(crate::types::Message::write_access_allowed),
            Self::Unknown(_) => None,
        }
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
impl From<crate::types::ChatFullInfoUnknown> for ChatFullInfo {
    fn from(val: crate::types::ChatFullInfoUnknown) -> Self {
        Self::Unknown(val)
    }
}
impl TryFrom<ChatFullInfo> for crate::types::ChatFullInfoUnknown {
    type Error = crate::errors::ConvertToTypeError;

    fn try_from(val: ChatFullInfo) -> Result<Self, Self::Error> {
        if let ChatFullInfo::Unknown(inner) = val {
            Ok(inner)
        } else {
            Err(Self::Error::new(
                stringify!(ChatFullInfo),
                stringify!(ChatFullInfoUnknown),
            ))
        }
    }
}
