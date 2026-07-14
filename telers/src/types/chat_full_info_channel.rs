use serde::{Deserialize, Serialize};
/// This object represents a channel chat.
/// # Notes
/// This object represents a chat from original chat type `channel`.
/// # Documentation
/// <https://core.telegram.org/bots/api#chatfullinfo>
#[derive(Clone, Debug, Serialize, Deserialize)]
pub struct ChatFullInfoChannel {
    /// Unique identifier for this chat. This number may have more than 32 significant bits and some programming languages may have difficulty/silent defects in interpreting it. But it has at most 52 significant bits, so a signed 64-bit integer or double-precision float type are safe for storing this identifier.
    pub id: i64,
    /// Title, for supergroups, channels and group chats
    #[serde(skip_serializing_if = "Option::is_none")]
    pub title: Option<Box<str>>,
    /// Username, for private chats, supergroups and channels if available
    #[serde(skip_serializing_if = "Option::is_none")]
    pub username: Option<Box<str>>,
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
    /// For private chats, the personal channel of the user
    #[serde(skip_serializing_if = "Option::is_none")]
    pub personal_chat: Option<Box<crate::types::Chat>>,
    /// Information about the corresponding channel chat; for direct messages chats only
    #[serde(skip_serializing_if = "Option::is_none")]
    pub parent_chat: Option<Box<crate::types::Chat>>,
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
    /// Description, for groups, supergroups and channel chats
    #[serde(skip_serializing_if = "Option::is_none")]
    pub description: Option<Box<str>>,
    /// Primary invite link, for groups, supergroups and channel chats
    #[serde(skip_serializing_if = "Option::is_none")]
    pub invite_link: Option<Box<str>>,
    /// The most recent pinned message (by sending date)
    #[serde(skip_serializing_if = "Option::is_none")]
    pub pinned_message: Option<Box<crate::types::Message>>,
    /// `true`, if paid media messages can be sent or forwarded to the channel chat. The field is available only for channel chats.
    #[serde(skip_serializing_if = "Option::is_none")]
    pub can_send_paid_media: Option<bool>,
    /// The time after which all messages sent to the chat will be automatically deleted; in seconds
    #[serde(skip_serializing_if = "Option::is_none")]
    pub message_auto_delete_time: Option<i64>,
    /// `true`, if non-administrators can only get the list of bots and administrators in the chat
    #[serde(skip_serializing_if = "Option::is_none")]
    pub has_hidden_members: Option<bool>,
    /// `true`, if messages from the chat can't be forwarded to other chats
    #[serde(skip_serializing_if = "Option::is_none")]
    pub has_protected_content: Option<bool>,
    /// `true`, if new chat members will have access to old messages; available only to chat administrators
    #[serde(skip_serializing_if = "Option::is_none")]
    pub has_visible_history: Option<bool>,
    /// Unique identifier for the linked chat, i.e. the discussion group identifier for a channel and vice versa; for supergroups and channel chats. This identifier may be greater than 32 bits and some programming languages may have difficulty/silent defects in interpreting it. But it is smaller than 52 bits, so a signed 64 bit integer or double-precision float type are safe for storing this identifier.
    #[serde(skip_serializing_if = "Option::is_none")]
    pub linked_chat_id: Option<i64>,
    /// The color scheme based on a unique gift that must be used for the chat's name, message replies and link previews
    #[serde(skip_serializing_if = "Option::is_none")]
    pub unique_gift_colors: Option<crate::types::UniqueGiftColors>,
    /// The number of Telegram Stars a general user has to pay to send a message to the chat
    #[serde(skip_serializing_if = "Option::is_none")]
    pub paid_message_star_count: Option<i64>,
    /// The bot that processes join request queries in the chat. The field is only available to chat administrators.
    #[serde(skip_serializing_if = "Option::is_none")]
    pub guard_bot: Option<Box<crate::types::User>>,
    /// The Community to which the chat belongs
    #[serde(skip_serializing_if = "Option::is_none")]
    pub community: Option<crate::types::Community>,
}
impl ChatFullInfoChannel {
    /// Creates a new `ChatFullInfoChannel`.
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
            is_direct_messages: None,
            accent_color_id: accent_color_id.into(),
            max_reaction_count: max_reaction_count.into(),
            photo: None,
            active_usernames: None,
            personal_chat: None,
            parent_chat: None,
            available_reactions: None,
            background_custom_emoji_id: None,
            profile_accent_color_id: None,
            profile_background_custom_emoji_id: None,
            description: None,
            invite_link: None,
            pinned_message: None,
            can_send_paid_media: None,
            message_auto_delete_time: None,
            has_hidden_members: None,
            has_protected_content: None,
            has_visible_history: None,
            linked_chat_id: None,
            unique_gift_colors: None,
            paid_message_star_count: None,
            guard_bot: None,
            community: None,
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

    /// For private chats, the personal channel of the user
    #[must_use]
    pub fn personal_chat<T: Into<crate::types::Chat>>(mut self, val: T) -> Self {
        self.personal_chat = Some(Box::new(val.into()));
        self
    }

    /// For private chats, the personal channel of the user
    #[must_use]
    pub fn personal_chat_option<T: Into<crate::types::Chat>>(mut self, val: Option<T>) -> Self {
        self.personal_chat = val.map(|val| Box::new(val.into()));
        self
    }

    /// Information about the corresponding channel chat; for direct messages chats only
    #[must_use]
    pub fn parent_chat<T: Into<crate::types::Chat>>(mut self, val: T) -> Self {
        self.parent_chat = Some(Box::new(val.into()));
        self
    }

    /// Information about the corresponding channel chat; for direct messages chats only
    #[must_use]
    pub fn parent_chat_option<T: Into<crate::types::Chat>>(mut self, val: Option<T>) -> Self {
        self.parent_chat = val.map(|val| Box::new(val.into()));
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

    /// `true`, if paid media messages can be sent or forwarded to the channel chat. The field is available only for channel chats.
    #[must_use]
    pub fn can_send_paid_media<T: Into<bool>>(mut self, val: T) -> Self {
        self.can_send_paid_media = Some(val.into());
        self
    }

    /// `true`, if paid media messages can be sent or forwarded to the channel chat. The field is available only for channel chats.
    #[must_use]
    pub fn can_send_paid_media_option<T: Into<bool>>(mut self, val: Option<T>) -> Self {
        self.can_send_paid_media = val.map(Into::into);
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

    /// The Community to which the chat belongs
    #[must_use]
    pub fn community<T: Into<crate::types::Community>>(mut self, val: T) -> Self {
        self.community = Some(val.into());
        self
    }

    /// The Community to which the chat belongs
    #[must_use]
    pub fn community_option<T: Into<crate::types::Community>>(mut self, val: Option<T>) -> Self {
        self.community = val.map(Into::into);
        self
    }
}
