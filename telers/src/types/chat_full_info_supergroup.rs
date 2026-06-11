use serde::{Deserialize, Serialize};
/// This object represents a supergroup chat.
/// # Notes
/// This object represents a chat from original chat type `supergroup`.
/// # Documentation
/// <https://core.telegram.org/bots/api#chatfullinfo>
#[derive(Clone, Debug, Serialize, Deserialize)]
pub struct ChatFullInfoSupergroup {
    /// Unique identifier for this chat. This number may have more than 32 significant bits and some programming languages may have difficulty/silent defects in interpreting it. But it has at most 52 significant bits, so a signed 64-bit integer or double-precision float type are safe for storing this identifier.
    pub id: i64,
    /// Title, for supergroups, channels and group chats
    #[serde(skip_serializing_if = "Option::is_none")]
    pub title: Option<Box<str>>,
    /// Username, for private chats, supergroups and channels if available
    #[serde(skip_serializing_if = "Option::is_none")]
    pub username: Option<Box<str>>,
    /// `true`, if the supergroup chat is a forum (has topics enabled)
    #[serde(skip_serializing_if = "Option::is_none")]
    pub is_forum: Option<bool>,
    /// `true`, if the chat is the direct messages chat of a channel
    #[serde(skip_serializing_if = "Option::is_none")]
    pub is_direct_messages: Option<bool>,
    /// Identifier of the accent color for the chat name and backgrounds of the chat photo, reply header, and link preview. See accent colors for more details.
    pub accent_color_id: i64,
    /// The maximum number of reactions that can be set on a message in the chat
    pub max_reaction_count: i64,
    /// Chat photo
    #[serde(skip_serializing_if = "Option::is_none")]
    pub photo: Option<crate::types::ChatPhoto>,
    /// If non-empty, the list of all active chat usernames; for private chats, supergroups and channels
    #[serde(skip_serializing_if = "Option::is_none")]
    pub active_usernames: Option<Box<[Box<str>]>>,
    /// List of available reactions allowed in the chat. If omitted, then all emoji reactions are allowed.
    #[serde(skip_serializing_if = "Option::is_none")]
    pub available_reactions: Option<Box<[crate::types::ReactionType]>>,
    /// Custom emoji identifier of the emoji chosen by the chat for the reply header and link preview background
    #[serde(skip_serializing_if = "Option::is_none")]
    pub background_custom_emoji_id: Option<Box<str>>,
    /// Identifier of the accent color for the chat's profile background. See profile accent colors for more details.
    #[serde(skip_serializing_if = "Option::is_none")]
    pub profile_accent_color_id: Option<i64>,
    /// Custom emoji identifier of the emoji chosen by the chat for its profile background
    #[serde(skip_serializing_if = "Option::is_none")]
    pub profile_background_custom_emoji_id: Option<Box<str>>,
    /// `true`, if users need to join the supergroup before they can send messages
    #[serde(skip_serializing_if = "Option::is_none")]
    pub join_to_send_messages: Option<bool>,
    /// `true`, if all users directly joining the supergroup without using an invite link need to be approved by supergroup administrators
    #[serde(skip_serializing_if = "Option::is_none")]
    pub join_by_request: Option<bool>,
    /// Description, for groups, supergroups and channel chats
    #[serde(skip_serializing_if = "Option::is_none")]
    pub description: Option<Box<str>>,
    /// Primary invite link, for groups, supergroups and channel chats
    #[serde(skip_serializing_if = "Option::is_none")]
    pub invite_link: Option<Box<str>>,
    /// The most recent pinned message (by sending date)
    #[serde(skip_serializing_if = "Option::is_none")]
    pub pinned_message: Option<Box<crate::types::Message>>,
    /// Default chat member permissions, for groups and supergroups
    #[serde(skip_serializing_if = "Option::is_none")]
    pub permissions: Option<crate::types::ChatPermissions>,
    /// For supergroups, the minimum allowed delay between consecutive messages sent by each unprivileged user; in seconds
    #[serde(skip_serializing_if = "Option::is_none")]
    pub slow_mode_delay: Option<i64>,
    /// For supergroups, the minimum number of boosts that a non-administrator user needs to add in order to ignore slow mode and chat permissions
    #[serde(skip_serializing_if = "Option::is_none")]
    pub unrestrict_boost_count: Option<i64>,
    /// The time after which all messages sent to the chat will be automatically deleted; in seconds
    #[serde(skip_serializing_if = "Option::is_none")]
    pub message_auto_delete_time: Option<i64>,
    /// `true`, if aggressive anti-spam checks are enabled in the supergroup. The field is only available to chat administrators.
    #[serde(skip_serializing_if = "Option::is_none")]
    pub has_aggressive_anti_spam_enabled: Option<bool>,
    /// `true`, if non-administrators can only get the list of bots and administrators in the chat
    #[serde(skip_serializing_if = "Option::is_none")]
    pub has_hidden_members: Option<bool>,
    /// `true`, if messages from the chat can't be forwarded to other chats
    #[serde(skip_serializing_if = "Option::is_none")]
    pub has_protected_content: Option<bool>,
    /// `true`, if new chat members will have access to old messages; available only to chat administrators
    #[serde(skip_serializing_if = "Option::is_none")]
    pub has_visible_history: Option<bool>,
    /// For supergroups, name of the group sticker set
    #[serde(skip_serializing_if = "Option::is_none")]
    pub sticker_set_name: Option<Box<str>>,
    /// For supergroups, the name of the group's custom emoji sticker set. Custom emoji from this set can be used by all users and bots in the group.
    #[serde(skip_serializing_if = "Option::is_none")]
    pub custom_emoji_sticker_set_name: Option<Box<str>>,
    /// Unique identifier for the linked chat, i.e. the discussion group identifier for a channel and vice versa; for supergroups and channel chats. This identifier may be greater than 32 bits and some programming languages may have difficulty/silent defects in interpreting it. But it is smaller than 52 bits, so a signed 64 bit integer or double-precision float type are safe for storing this identifier.
    #[serde(skip_serializing_if = "Option::is_none")]
    pub linked_chat_id: Option<i64>,
    /// For supergroups, the location to which the supergroup is connected
    #[serde(skip_serializing_if = "Option::is_none")]
    pub location: Option<crate::types::ChatLocation>,
    /// The color scheme based on a unique gift that must be used for the chat's name, message replies and link previews
    #[serde(skip_serializing_if = "Option::is_none")]
    pub unique_gift_colors: Option<crate::types::UniqueGiftColors>,
    /// The number of Telegram Stars a general user has to pay to send a message to the chat
    #[serde(skip_serializing_if = "Option::is_none")]
    pub paid_message_star_count: Option<i64>,
    /// The bot that processes join request queries in the chat. The field is only available to chat administrators.
    #[serde(skip_serializing_if = "Option::is_none")]
    pub guard_bot: Option<Box<crate::types::User>>,
}
impl ChatFullInfoSupergroup {
    /// Creates a new `ChatFullInfoSupergroup`.
    ///
    /// # Arguments
    /// * `id` - Unique identifier for this chat. This number may have more than 32 significant bits and some programming languages may have difficulty/silent defects in interpreting it. But it has at most 52 significant bits, so a signed 64-bit integer or double-precision float type are safe for storing this identifier.
    /// * `accent_color_id` - Identifier of the accent color for the chat name and backgrounds of the chat photo, reply header, and link preview. See accent colors for more details.
    /// * `max_reaction_count` - The maximum number of reactions that can be set on a message in the chat
    ///
    /// # Notes
    /// Use builder methods to set optional fields.
    #[must_use]
    pub fn new<T0: Into<i64>, T1: Into<i64>, T2: Into<i64>>(
        id: T0,
        accent_color_id: T1,
        max_reaction_count: T2,
    ) -> Self {
        Self {
            id: id.into(),
            title: None,
            username: None,
            is_forum: None,
            is_direct_messages: None,
            accent_color_id: accent_color_id.into(),
            max_reaction_count: max_reaction_count.into(),
            photo: None,
            active_usernames: None,
            available_reactions: None,
            background_custom_emoji_id: None,
            profile_accent_color_id: None,
            profile_background_custom_emoji_id: None,
            join_to_send_messages: None,
            join_by_request: None,
            description: None,
            invite_link: None,
            pinned_message: None,
            permissions: None,
            slow_mode_delay: None,
            unrestrict_boost_count: None,
            message_auto_delete_time: None,
            has_aggressive_anti_spam_enabled: None,
            has_hidden_members: None,
            has_protected_content: None,
            has_visible_history: None,
            sticker_set_name: None,
            custom_emoji_sticker_set_name: None,
            linked_chat_id: None,
            location: None,
            unique_gift_colors: None,
            paid_message_star_count: None,
            guard_bot: None,
        }
    }

    /// Unique identifier for this chat. This number may have more than 32 significant bits and some programming languages may have difficulty/silent defects in interpreting it. But it has at most 52 significant bits, so a signed 64-bit integer or double-precision float type are safe for storing this identifier.
    #[must_use]
    pub fn id<T: Into<i64>>(mut self, val: T) -> Self {
        self.id = val.into();
        self
    }

    /// Title, for supergroups, channels and group chats
    #[must_use]
    pub fn title<T: Into<Box<str>>>(mut self, val: T) -> Self {
        self.title = Some(val.into());
        self
    }

    /// Title, for supergroups, channels and group chats
    #[must_use]
    pub fn title_option<T: Into<Box<str>>>(mut self, val: Option<T>) -> Self {
        self.title = val.map(Into::into);
        self
    }

    /// Username, for private chats, supergroups and channels if available
    #[must_use]
    pub fn username<T: Into<Box<str>>>(mut self, val: T) -> Self {
        self.username = Some(val.into());
        self
    }

    /// Username, for private chats, supergroups and channels if available
    #[must_use]
    pub fn username_option<T: Into<Box<str>>>(mut self, val: Option<T>) -> Self {
        self.username = val.map(Into::into);
        self
    }

    /// `true`, if the supergroup chat is a forum (has topics enabled)
    #[must_use]
    pub fn is_forum<T: Into<bool>>(mut self, val: T) -> Self {
        self.is_forum = Some(val.into());
        self
    }

    /// `true`, if the supergroup chat is a forum (has topics enabled)
    #[must_use]
    pub fn is_forum_option<T: Into<bool>>(mut self, val: Option<T>) -> Self {
        self.is_forum = val.map(Into::into);
        self
    }

    /// `true`, if the chat is the direct messages chat of a channel
    #[must_use]
    pub fn is_direct_messages<T: Into<bool>>(mut self, val: T) -> Self {
        self.is_direct_messages = Some(val.into());
        self
    }

    /// `true`, if the chat is the direct messages chat of a channel
    #[must_use]
    pub fn is_direct_messages_option<T: Into<bool>>(mut self, val: Option<T>) -> Self {
        self.is_direct_messages = val.map(Into::into);
        self
    }

    /// Identifier of the accent color for the chat name and backgrounds of the chat photo, reply header, and link preview. See accent colors for more details.
    #[must_use]
    pub fn accent_color_id<T: Into<i64>>(mut self, val: T) -> Self {
        self.accent_color_id = val.into();
        self
    }

    /// The maximum number of reactions that can be set on a message in the chat
    #[must_use]
    pub fn max_reaction_count<T: Into<i64>>(mut self, val: T) -> Self {
        self.max_reaction_count = val.into();
        self
    }

    /// Chat photo
    #[must_use]
    pub fn photo<T: Into<crate::types::ChatPhoto>>(mut self, val: T) -> Self {
        self.photo = Some(val.into());
        self
    }

    /// Chat photo
    #[must_use]
    pub fn photo_option<T: Into<crate::types::ChatPhoto>>(mut self, val: Option<T>) -> Self {
        self.photo = val.map(Into::into);
        self
    }

    /// If non-empty, the list of all active chat usernames; for private chats, supergroups and channels
    ///
    /// # Notes
    /// Adds multiple elements.
    #[must_use]
    pub fn active_usernames<T: Into<Box<[Box<str>]>>>(mut self, val: T) -> Self {
        self.active_usernames = Some(
            self.active_usernames
                .unwrap_or_default()
                .into_vec()
                .into_iter()
                .chain(val.into())
                .collect(),
        );
        self
    }

    /// If non-empty, the list of all active chat usernames; for private chats, supergroups and channels
    ///
    /// # Notes
    /// Adds a single element.
    #[must_use]
    pub fn active_username<T: Into<Box<str>>>(mut self, val: T) -> Self {
        self.active_usernames = Some(
            self.active_usernames
                .unwrap_or_default()
                .into_vec()
                .into_iter()
                .chain(Some(val.into()))
                .collect(),
        );
        self
    }

    /// If non-empty, the list of all active chat usernames; for private chats, supergroups and channels
    ///
    /// # Notes
    /// Adds a single element.
    #[must_use]
    pub fn active_usernames_option<T: Into<Box<[Box<str>]>>>(mut self, val: Option<T>) -> Self {
        self.active_usernames = val.map(Into::into);
        self
    }

    /// List of available reactions allowed in the chat. If omitted, then all emoji reactions are allowed.
    ///
    /// # Notes
    /// Adds multiple elements.
    #[must_use]
    pub fn available_reactions<T: Into<Box<[crate::types::ReactionType]>>>(
        mut self,
        val: T,
    ) -> Self {
        self.available_reactions = Some(
            self.available_reactions
                .unwrap_or_default()
                .into_vec()
                .into_iter()
                .chain(val.into())
                .collect(),
        );
        self
    }

    /// List of available reactions allowed in the chat. If omitted, then all emoji reactions are allowed.
    ///
    /// # Notes
    /// Adds a single element.
    #[must_use]
    pub fn available_reaction<T: Into<crate::types::ReactionType>>(mut self, val: T) -> Self {
        self.available_reactions = Some(
            self.available_reactions
                .unwrap_or_default()
                .into_vec()
                .into_iter()
                .chain(Some(val.into()))
                .collect(),
        );
        self
    }

    /// List of available reactions allowed in the chat. If omitted, then all emoji reactions are allowed.
    ///
    /// # Notes
    /// Adds a single element.
    #[must_use]
    pub fn available_reactions_option<T: Into<Box<[crate::types::ReactionType]>>>(
        mut self,
        val: Option<T>,
    ) -> Self {
        self.available_reactions = val.map(Into::into);
        self
    }

    /// Custom emoji identifier of the emoji chosen by the chat for the reply header and link preview background
    #[must_use]
    pub fn background_custom_emoji_id<T: Into<Box<str>>>(mut self, val: T) -> Self {
        self.background_custom_emoji_id = Some(val.into());
        self
    }

    /// Custom emoji identifier of the emoji chosen by the chat for the reply header and link preview background
    #[must_use]
    pub fn background_custom_emoji_id_option<T: Into<Box<str>>>(mut self, val: Option<T>) -> Self {
        self.background_custom_emoji_id = val.map(Into::into);
        self
    }

    /// Identifier of the accent color for the chat's profile background. See profile accent colors for more details.
    #[must_use]
    pub fn profile_accent_color_id<T: Into<i64>>(mut self, val: T) -> Self {
        self.profile_accent_color_id = Some(val.into());
        self
    }

    /// Identifier of the accent color for the chat's profile background. See profile accent colors for more details.
    #[must_use]
    pub fn profile_accent_color_id_option<T: Into<i64>>(mut self, val: Option<T>) -> Self {
        self.profile_accent_color_id = val.map(Into::into);
        self
    }

    /// Custom emoji identifier of the emoji chosen by the chat for its profile background
    #[must_use]
    pub fn profile_background_custom_emoji_id<T: Into<Box<str>>>(mut self, val: T) -> Self {
        self.profile_background_custom_emoji_id = Some(val.into());
        self
    }

    /// Custom emoji identifier of the emoji chosen by the chat for its profile background
    #[must_use]
    pub fn profile_background_custom_emoji_id_option<T: Into<Box<str>>>(
        mut self,
        val: Option<T>,
    ) -> Self {
        self.profile_background_custom_emoji_id = val.map(Into::into);
        self
    }

    /// `true`, if users need to join the supergroup before they can send messages
    #[must_use]
    pub fn join_to_send_messages<T: Into<bool>>(mut self, val: T) -> Self {
        self.join_to_send_messages = Some(val.into());
        self
    }

    /// `true`, if users need to join the supergroup before they can send messages
    #[must_use]
    pub fn join_to_send_messages_option<T: Into<bool>>(mut self, val: Option<T>) -> Self {
        self.join_to_send_messages = val.map(Into::into);
        self
    }

    /// `true`, if all users directly joining the supergroup without using an invite link need to be approved by supergroup administrators
    #[must_use]
    pub fn join_by_request<T: Into<bool>>(mut self, val: T) -> Self {
        self.join_by_request = Some(val.into());
        self
    }

    /// `true`, if all users directly joining the supergroup without using an invite link need to be approved by supergroup administrators
    #[must_use]
    pub fn join_by_request_option<T: Into<bool>>(mut self, val: Option<T>) -> Self {
        self.join_by_request = val.map(Into::into);
        self
    }

    /// Description, for groups, supergroups and channel chats
    #[must_use]
    pub fn description<T: Into<Box<str>>>(mut self, val: T) -> Self {
        self.description = Some(val.into());
        self
    }

    /// Description, for groups, supergroups and channel chats
    #[must_use]
    pub fn description_option<T: Into<Box<str>>>(mut self, val: Option<T>) -> Self {
        self.description = val.map(Into::into);
        self
    }

    /// Primary invite link, for groups, supergroups and channel chats
    #[must_use]
    pub fn invite_link<T: Into<Box<str>>>(mut self, val: T) -> Self {
        self.invite_link = Some(val.into());
        self
    }

    /// Primary invite link, for groups, supergroups and channel chats
    #[must_use]
    pub fn invite_link_option<T: Into<Box<str>>>(mut self, val: Option<T>) -> Self {
        self.invite_link = val.map(Into::into);
        self
    }

    /// The most recent pinned message (by sending date)
    #[must_use]
    pub fn pinned_message<T: Into<crate::types::Message>>(mut self, val: T) -> Self {
        self.pinned_message = Some(Box::new(val.into()));
        self
    }

    /// The most recent pinned message (by sending date)
    #[must_use]
    pub fn pinned_message_option<T: Into<crate::types::Message>>(mut self, val: Option<T>) -> Self {
        self.pinned_message = val.map(|val| Box::new(val.into()));
        self
    }

    /// Default chat member permissions, for groups and supergroups
    #[must_use]
    pub fn permissions<T: Into<crate::types::ChatPermissions>>(mut self, val: T) -> Self {
        self.permissions = Some(val.into());
        self
    }

    /// Default chat member permissions, for groups and supergroups
    #[must_use]
    pub fn permissions_option<T: Into<crate::types::ChatPermissions>>(
        mut self,
        val: Option<T>,
    ) -> Self {
        self.permissions = val.map(Into::into);
        self
    }

    /// For supergroups, the minimum allowed delay between consecutive messages sent by each unprivileged user; in seconds
    #[must_use]
    pub fn slow_mode_delay<T: Into<i64>>(mut self, val: T) -> Self {
        self.slow_mode_delay = Some(val.into());
        self
    }

    /// For supergroups, the minimum allowed delay between consecutive messages sent by each unprivileged user; in seconds
    #[must_use]
    pub fn slow_mode_delay_option<T: Into<i64>>(mut self, val: Option<T>) -> Self {
        self.slow_mode_delay = val.map(Into::into);
        self
    }

    /// For supergroups, the minimum number of boosts that a non-administrator user needs to add in order to ignore slow mode and chat permissions
    #[must_use]
    pub fn unrestrict_boost_count<T: Into<i64>>(mut self, val: T) -> Self {
        self.unrestrict_boost_count = Some(val.into());
        self
    }

    /// For supergroups, the minimum number of boosts that a non-administrator user needs to add in order to ignore slow mode and chat permissions
    #[must_use]
    pub fn unrestrict_boost_count_option<T: Into<i64>>(mut self, val: Option<T>) -> Self {
        self.unrestrict_boost_count = val.map(Into::into);
        self
    }

    /// The time after which all messages sent to the chat will be automatically deleted; in seconds
    #[must_use]
    pub fn message_auto_delete_time<T: Into<i64>>(mut self, val: T) -> Self {
        self.message_auto_delete_time = Some(val.into());
        self
    }

    /// The time after which all messages sent to the chat will be automatically deleted; in seconds
    #[must_use]
    pub fn message_auto_delete_time_option<T: Into<i64>>(mut self, val: Option<T>) -> Self {
        self.message_auto_delete_time = val.map(Into::into);
        self
    }

    /// `true`, if aggressive anti-spam checks are enabled in the supergroup. The field is only available to chat administrators.
    #[must_use]
    pub fn has_aggressive_anti_spam_enabled<T: Into<bool>>(mut self, val: T) -> Self {
        self.has_aggressive_anti_spam_enabled = Some(val.into());
        self
    }

    /// `true`, if aggressive anti-spam checks are enabled in the supergroup. The field is only available to chat administrators.
    #[must_use]
    pub fn has_aggressive_anti_spam_enabled_option<T: Into<bool>>(
        mut self,
        val: Option<T>,
    ) -> Self {
        self.has_aggressive_anti_spam_enabled = val.map(Into::into);
        self
    }

    /// `true`, if non-administrators can only get the list of bots and administrators in the chat
    #[must_use]
    pub fn has_hidden_members<T: Into<bool>>(mut self, val: T) -> Self {
        self.has_hidden_members = Some(val.into());
        self
    }

    /// `true`, if non-administrators can only get the list of bots and administrators in the chat
    #[must_use]
    pub fn has_hidden_members_option<T: Into<bool>>(mut self, val: Option<T>) -> Self {
        self.has_hidden_members = val.map(Into::into);
        self
    }

    /// `true`, if messages from the chat can't be forwarded to other chats
    #[must_use]
    pub fn has_protected_content<T: Into<bool>>(mut self, val: T) -> Self {
        self.has_protected_content = Some(val.into());
        self
    }

    /// `true`, if messages from the chat can't be forwarded to other chats
    #[must_use]
    pub fn has_protected_content_option<T: Into<bool>>(mut self, val: Option<T>) -> Self {
        self.has_protected_content = val.map(Into::into);
        self
    }

    /// `true`, if new chat members will have access to old messages; available only to chat administrators
    #[must_use]
    pub fn has_visible_history<T: Into<bool>>(mut self, val: T) -> Self {
        self.has_visible_history = Some(val.into());
        self
    }

    /// `true`, if new chat members will have access to old messages; available only to chat administrators
    #[must_use]
    pub fn has_visible_history_option<T: Into<bool>>(mut self, val: Option<T>) -> Self {
        self.has_visible_history = val.map(Into::into);
        self
    }

    /// For supergroups, name of the group sticker set
    #[must_use]
    pub fn sticker_set_name<T: Into<Box<str>>>(mut self, val: T) -> Self {
        self.sticker_set_name = Some(val.into());
        self
    }

    /// For supergroups, name of the group sticker set
    #[must_use]
    pub fn sticker_set_name_option<T: Into<Box<str>>>(mut self, val: Option<T>) -> Self {
        self.sticker_set_name = val.map(Into::into);
        self
    }

    /// For supergroups, the name of the group's custom emoji sticker set. Custom emoji from this set can be used by all users and bots in the group.
    #[must_use]
    pub fn custom_emoji_sticker_set_name<T: Into<Box<str>>>(mut self, val: T) -> Self {
        self.custom_emoji_sticker_set_name = Some(val.into());
        self
    }

    /// For supergroups, the name of the group's custom emoji sticker set. Custom emoji from this set can be used by all users and bots in the group.
    #[must_use]
    pub fn custom_emoji_sticker_set_name_option<T: Into<Box<str>>>(
        mut self,
        val: Option<T>,
    ) -> Self {
        self.custom_emoji_sticker_set_name = val.map(Into::into);
        self
    }

    /// Unique identifier for the linked chat, i.e. the discussion group identifier for a channel and vice versa; for supergroups and channel chats. This identifier may be greater than 32 bits and some programming languages may have difficulty/silent defects in interpreting it. But it is smaller than 52 bits, so a signed 64 bit integer or double-precision float type are safe for storing this identifier.
    #[must_use]
    pub fn linked_chat_id<T: Into<i64>>(mut self, val: T) -> Self {
        self.linked_chat_id = Some(val.into());
        self
    }

    /// Unique identifier for the linked chat, i.e. the discussion group identifier for a channel and vice versa; for supergroups and channel chats. This identifier may be greater than 32 bits and some programming languages may have difficulty/silent defects in interpreting it. But it is smaller than 52 bits, so a signed 64 bit integer or double-precision float type are safe for storing this identifier.
    #[must_use]
    pub fn linked_chat_id_option<T: Into<i64>>(mut self, val: Option<T>) -> Self {
        self.linked_chat_id = val.map(Into::into);
        self
    }

    /// For supergroups, the location to which the supergroup is connected
    #[must_use]
    pub fn location<T: Into<crate::types::ChatLocation>>(mut self, val: T) -> Self {
        self.location = Some(val.into());
        self
    }

    /// For supergroups, the location to which the supergroup is connected
    #[must_use]
    pub fn location_option<T: Into<crate::types::ChatLocation>>(mut self, val: Option<T>) -> Self {
        self.location = val.map(Into::into);
        self
    }

    /// The color scheme based on a unique gift that must be used for the chat's name, message replies and link previews
    #[must_use]
    pub fn unique_gift_colors<T: Into<crate::types::UniqueGiftColors>>(mut self, val: T) -> Self {
        self.unique_gift_colors = Some(val.into());
        self
    }

    /// The color scheme based on a unique gift that must be used for the chat's name, message replies and link previews
    #[must_use]
    pub fn unique_gift_colors_option<T: Into<crate::types::UniqueGiftColors>>(
        mut self,
        val: Option<T>,
    ) -> Self {
        self.unique_gift_colors = val.map(Into::into);
        self
    }

    /// The number of Telegram Stars a general user has to pay to send a message to the chat
    #[must_use]
    pub fn paid_message_star_count<T: Into<i64>>(mut self, val: T) -> Self {
        self.paid_message_star_count = Some(val.into());
        self
    }

    /// The number of Telegram Stars a general user has to pay to send a message to the chat
    #[must_use]
    pub fn paid_message_star_count_option<T: Into<i64>>(mut self, val: Option<T>) -> Self {
        self.paid_message_star_count = val.map(Into::into);
        self
    }

    /// The bot that processes join request queries in the chat. The field is only available to chat administrators.
    #[must_use]
    pub fn guard_bot<T: Into<crate::types::User>>(mut self, val: T) -> Self {
        self.guard_bot = Some(Box::new(val.into()));
        self
    }

    /// The bot that processes join request queries in the chat. The field is only available to chat administrators.
    #[must_use]
    pub fn guard_bot_option<T: Into<crate::types::User>>(mut self, val: Option<T>) -> Self {
        self.guard_bot = val.map(|val| Box::new(val.into()));
        self
    }
}
