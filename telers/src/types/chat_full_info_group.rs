use serde::{Deserialize, Serialize};
/// This object represents a group chat.
/// # Notes
/// This object represents a chat from original chat type `group`.
/// # Documentation
/// <https://core.telegram.org/bots/api#chatfullinfo>
#[derive(Clone, Debug, Serialize, Deserialize)]
pub struct ChatFullInfoGroup {
    /// Unique identifier for this chat. This number may have more than 32 significant bits and some programming languages may have difficulty/silent defects in interpreting it. But it has at most 52 significant bits, so a signed 64-bit integer or double-precision float type are safe for storing this identifier.
    pub id: i64,
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
    /// The most recent pinned message (by sending date)
    #[serde(skip_serializing_if = "Option::is_none")]
    pub pinned_message: Option<Box<crate::types::Message>>,
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
    /// `true`, if the bot can change the group sticker set
    #[serde(skip_serializing_if = "Option::is_none")]
    pub can_set_sticker_set: Option<bool>,
    /// The color scheme based on a unique gift that must be used for the chat's name, message replies and link previews
    #[serde(skip_serializing_if = "Option::is_none")]
    pub unique_gift_colors: Option<crate::types::UniqueGiftColors>,
    /// The number of Telegram Stars a general user have to pay to send a message to the chat
    #[serde(skip_serializing_if = "Option::is_none")]
    pub paid_message_star_count: Option<i64>,
}
impl ChatFullInfoGroup {
    /// Creates a new `ChatFullInfoGroup`.
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
            is_direct_messages: None,
            accent_color_id: accent_color_id.into(),
            max_reaction_count: max_reaction_count.into(),
            photo: None,
            available_reactions: None,
            background_custom_emoji_id: None,
            profile_accent_color_id: None,
            profile_background_custom_emoji_id: None,
            pinned_message: None,
            message_auto_delete_time: None,
            has_hidden_members: None,
            has_protected_content: None,
            has_visible_history: None,
            can_set_sticker_set: None,
            unique_gift_colors: None,
            paid_message_star_count: None,
        }
    }

    /// Unique identifier for this chat. This number may have more than 32 significant bits and some programming languages may have difficulty/silent defects in interpreting it. But it has at most 52 significant bits, so a signed 64-bit integer or double-precision float type are safe for storing this identifier.
    #[must_use]
    pub fn id<T: Into<i64>>(self, val: T) -> Self {
        let mut this = self;
        this.id = val.into();
        this
    }

    /// `true`, if the chat is the direct messages chat of a channel
    #[must_use]
    pub fn is_direct_messages<T: Into<bool>>(self, val: T) -> Self {
        let mut this = self;
        this.is_direct_messages = Some(val.into());
        this
    }

    /// `true`, if the chat is the direct messages chat of a channel
    #[must_use]
    pub fn is_direct_messages_option<T: Into<bool>>(self, val: Option<T>) -> Self {
        let mut this = self;
        this.is_direct_messages = val.map(Into::into);
        this
    }

    /// Identifier of the accent color for the chat name and backgrounds of the chat photo, reply header, and link preview. See accent colors for more details.
    #[must_use]
    pub fn accent_color_id<T: Into<i64>>(self, val: T) -> Self {
        let mut this = self;
        this.accent_color_id = val.into();
        this
    }

    /// The maximum number of reactions that can be set on a message in the chat
    #[must_use]
    pub fn max_reaction_count<T: Into<i64>>(self, val: T) -> Self {
        let mut this = self;
        this.max_reaction_count = val.into();
        this
    }

    /// Chat photo
    #[must_use]
    pub fn photo<T: Into<crate::types::ChatPhoto>>(self, val: T) -> Self {
        let mut this = self;
        this.photo = Some(val.into());
        this
    }

    /// Chat photo
    #[must_use]
    pub fn photo_option<T: Into<crate::types::ChatPhoto>>(self, val: Option<T>) -> Self {
        let mut this = self;
        this.photo = val.map(Into::into);
        this
    }

    /// List of available reactions allowed in the chat. If omitted, then all emoji reactions are allowed.
    ///
    /// # Notes
    /// Adds multiple elements.
    #[must_use]
    pub fn available_reactions<T: Into<Box<[crate::types::ReactionType]>>>(self, val: T) -> Self {
        let mut this = self;
        this.available_reactions = Some(
            this.available_reactions
                .unwrap_or_default()
                .into_vec()
                .into_iter()
                .chain(val.into())
                .collect(),
        );
        this
    }

    /// List of available reactions allowed in the chat. If omitted, then all emoji reactions are allowed.
    ///
    /// # Notes
    /// Adds a single element.
    #[must_use]
    pub fn available_reaction<T: Into<crate::types::ReactionType>>(self, val: T) -> Self {
        let mut this = self;
        this.available_reactions = Some(
            this.available_reactions
                .unwrap_or_default()
                .into_vec()
                .into_iter()
                .chain(Some(val.into()))
                .collect(),
        );
        this
    }

    /// List of available reactions allowed in the chat. If omitted, then all emoji reactions are allowed.
    ///
    /// # Notes
    /// Adds a single element.
    #[must_use]
    pub fn available_reactions_option<T: Into<Box<[crate::types::ReactionType]>>>(
        self,
        val: Option<T>,
    ) -> Self {
        let mut this = self;
        this.available_reactions = val.map(Into::into);
        this
    }

    /// Custom emoji identifier of the emoji chosen by the chat for the reply header and link preview background
    #[must_use]
    pub fn background_custom_emoji_id<T: Into<Box<str>>>(self, val: T) -> Self {
        let mut this = self;
        this.background_custom_emoji_id = Some(val.into());
        this
    }

    /// Custom emoji identifier of the emoji chosen by the chat for the reply header and link preview background
    #[must_use]
    pub fn background_custom_emoji_id_option<T: Into<Box<str>>>(self, val: Option<T>) -> Self {
        let mut this = self;
        this.background_custom_emoji_id = val.map(Into::into);
        this
    }

    /// Identifier of the accent color for the chat's profile background. See profile accent colors for more details.
    #[must_use]
    pub fn profile_accent_color_id<T: Into<i64>>(self, val: T) -> Self {
        let mut this = self;
        this.profile_accent_color_id = Some(val.into());
        this
    }

    /// Identifier of the accent color for the chat's profile background. See profile accent colors for more details.
    #[must_use]
    pub fn profile_accent_color_id_option<T: Into<i64>>(self, val: Option<T>) -> Self {
        let mut this = self;
        this.profile_accent_color_id = val.map(Into::into);
        this
    }

    /// Custom emoji identifier of the emoji chosen by the chat for its profile background
    #[must_use]
    pub fn profile_background_custom_emoji_id<T: Into<Box<str>>>(self, val: T) -> Self {
        let mut this = self;
        this.profile_background_custom_emoji_id = Some(val.into());
        this
    }

    /// Custom emoji identifier of the emoji chosen by the chat for its profile background
    #[must_use]
    pub fn profile_background_custom_emoji_id_option<T: Into<Box<str>>>(
        self,
        val: Option<T>,
    ) -> Self {
        let mut this = self;
        this.profile_background_custom_emoji_id = val.map(Into::into);
        this
    }

    /// The most recent pinned message (by sending date)
    #[must_use]
    pub fn pinned_message<T: Into<crate::types::Message>>(self, val: T) -> Self {
        let mut this = self;
        this.pinned_message = Some(Box::new(val.into()));
        this
    }

    /// The most recent pinned message (by sending date)
    #[must_use]
    pub fn pinned_message_option<T: Into<crate::types::Message>>(self, val: Option<T>) -> Self {
        let mut this = self;
        this.pinned_message = val.map(|val| Box::new(val.into()));
        this
    }

    /// The time after which all messages sent to the chat will be automatically deleted; in seconds
    #[must_use]
    pub fn message_auto_delete_time<T: Into<i64>>(self, val: T) -> Self {
        let mut this = self;
        this.message_auto_delete_time = Some(val.into());
        this
    }

    /// The time after which all messages sent to the chat will be automatically deleted; in seconds
    #[must_use]
    pub fn message_auto_delete_time_option<T: Into<i64>>(self, val: Option<T>) -> Self {
        let mut this = self;
        this.message_auto_delete_time = val.map(Into::into);
        this
    }

    /// `true`, if non-administrators can only get the list of bots and administrators in the chat
    #[must_use]
    pub fn has_hidden_members<T: Into<bool>>(self, val: T) -> Self {
        let mut this = self;
        this.has_hidden_members = Some(val.into());
        this
    }

    /// `true`, if non-administrators can only get the list of bots and administrators in the chat
    #[must_use]
    pub fn has_hidden_members_option<T: Into<bool>>(self, val: Option<T>) -> Self {
        let mut this = self;
        this.has_hidden_members = val.map(Into::into);
        this
    }

    /// `true`, if messages from the chat can't be forwarded to other chats
    #[must_use]
    pub fn has_protected_content<T: Into<bool>>(self, val: T) -> Self {
        let mut this = self;
        this.has_protected_content = Some(val.into());
        this
    }

    /// `true`, if messages from the chat can't be forwarded to other chats
    #[must_use]
    pub fn has_protected_content_option<T: Into<bool>>(self, val: Option<T>) -> Self {
        let mut this = self;
        this.has_protected_content = val.map(Into::into);
        this
    }

    /// `true`, if new chat members will have access to old messages; available only to chat administrators
    #[must_use]
    pub fn has_visible_history<T: Into<bool>>(self, val: T) -> Self {
        let mut this = self;
        this.has_visible_history = Some(val.into());
        this
    }

    /// `true`, if new chat members will have access to old messages; available only to chat administrators
    #[must_use]
    pub fn has_visible_history_option<T: Into<bool>>(self, val: Option<T>) -> Self {
        let mut this = self;
        this.has_visible_history = val.map(Into::into);
        this
    }

    /// `true`, if the bot can change the group sticker set
    #[must_use]
    pub fn can_set_sticker_set<T: Into<bool>>(self, val: T) -> Self {
        let mut this = self;
        this.can_set_sticker_set = Some(val.into());
        this
    }

    /// `true`, if the bot can change the group sticker set
    #[must_use]
    pub fn can_set_sticker_set_option<T: Into<bool>>(self, val: Option<T>) -> Self {
        let mut this = self;
        this.can_set_sticker_set = val.map(Into::into);
        this
    }

    /// The color scheme based on a unique gift that must be used for the chat's name, message replies and link previews
    #[must_use]
    pub fn unique_gift_colors<T: Into<crate::types::UniqueGiftColors>>(self, val: T) -> Self {
        let mut this = self;
        this.unique_gift_colors = Some(val.into());
        this
    }

    /// The color scheme based on a unique gift that must be used for the chat's name, message replies and link previews
    #[must_use]
    pub fn unique_gift_colors_option<T: Into<crate::types::UniqueGiftColors>>(
        self,
        val: Option<T>,
    ) -> Self {
        let mut this = self;
        this.unique_gift_colors = val.map(Into::into);
        this
    }

    /// The number of Telegram Stars a general user have to pay to send a message to the chat
    #[must_use]
    pub fn paid_message_star_count<T: Into<i64>>(self, val: T) -> Self {
        let mut this = self;
        this.paid_message_star_count = Some(val.into());
        this
    }

    /// The number of Telegram Stars a general user have to pay to send a message to the chat
    #[must_use]
    pub fn paid_message_star_count_option<T: Into<i64>>(self, val: Option<T>) -> Self {
        let mut this = self;
        this.paid_message_star_count = val.map(Into::into);
        this
    }
}
