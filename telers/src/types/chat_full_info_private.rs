use serde::{Deserialize, Serialize};
/// This object represents a private chat.
/// # Notes
/// This object represents a chat from original chat type `private`.
/// # Documentation
/// <https://core.telegram.org/bots/api#chatfullinfo>
#[derive(Clone, Debug, Serialize, Deserialize)]
pub struct ChatFullInfoPrivate {
    /// Unique identifier for this chat. This number may have more than 32 significant bits and some programming languages may have difficulty/silent defects in interpreting it. But it has at most 52 significant bits, so a signed 64-bit integer or double-precision float type are safe for storing this identifier.
    pub id: i64,
    /// Username, for private chats, supergroups and channels if available
    #[serde(skip_serializing_if = "Option::is_none")]
    pub username: Option<Box<str>>,
    /// First name of the other party in a private chat
    #[serde(skip_serializing_if = "Option::is_none")]
    pub first_name: Option<Box<str>>,
    /// Last name of the other party in a private chat
    #[serde(skip_serializing_if = "Option::is_none")]
    pub last_name: Option<Box<str>>,
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
    /// For private chats, the date of birth of the user
    #[serde(skip_serializing_if = "Option::is_none")]
    pub birthdate: Option<crate::types::Birthdate>,
    /// For private chats with business accounts, the intro of the business
    #[serde(skip_serializing_if = "Option::is_none")]
    pub business_intro: Option<crate::types::BusinessIntro>,
    /// For private chats with business accounts, the location of the business
    #[serde(skip_serializing_if = "Option::is_none")]
    pub business_location: Option<crate::types::BusinessLocation>,
    /// For private chats with business accounts, the opening hours of the business
    #[serde(skip_serializing_if = "Option::is_none")]
    pub business_opening_hours: Option<crate::types::BusinessOpeningHours>,
    /// For private chats, the personal channel of the user
    #[serde(skip_serializing_if = "Option::is_none")]
    pub personal_chat: Option<Box<crate::types::Chat>>,
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
    /// Custom emoji identifier of the emoji status of the chat or the other party in a private chat
    #[serde(skip_serializing_if = "Option::is_none")]
    pub emoji_status_custom_emoji_id: Option<Box<str>>,
    /// Expiration date of the emoji status of the chat or the other party in a private chat, in Unix time, if any
    #[serde(skip_serializing_if = "Option::is_none")]
    pub emoji_status_expiration_date: Option<i64>,
    /// Bio of the other party in a private chat
    #[serde(skip_serializing_if = "Option::is_none")]
    pub bio: Option<Box<str>>,
    /// `true`, if privacy settings of the other party in the private chat allows to use ``tg://user?id=<user_id>`` links only in chats with the user
    #[serde(skip_serializing_if = "Option::is_none")]
    pub has_private_forwards: Option<bool>,
    /// `true`, if the privacy settings of the other party restrict sending voice and video note messages in the private chat
    #[serde(skip_serializing_if = "Option::is_none")]
    pub has_restricted_voice_and_video_messages: Option<bool>,
    /// The most recent pinned message (by sending date)
    #[serde(skip_serializing_if = "Option::is_none")]
    pub pinned_message: Option<Box<crate::types::Message>>,
    /// Information about types of gifts that are accepted by the chat or by the corresponding user for private chats
    pub accepted_gift_types: crate::types::AcceptedGiftTypes,
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
    /// For private chats, the rating of the user if any
    #[serde(skip_serializing_if = "Option::is_none")]
    pub rating: Option<crate::types::UserRating>,
    /// For private chats, the first audio added to the profile of the user
    #[serde(skip_serializing_if = "Option::is_none")]
    pub first_profile_audio: Option<Box<crate::types::Audio>>,
    /// The color scheme based on a unique gift that must be used for the chat's name, message replies and link previews
    #[serde(skip_serializing_if = "Option::is_none")]
    pub unique_gift_colors: Option<crate::types::UniqueGiftColors>,
    /// The number of Telegram Stars a general user have to pay to send a message to the chat
    #[serde(skip_serializing_if = "Option::is_none")]
    pub paid_message_star_count: Option<i64>,
}
impl ChatFullInfoPrivate {
    /// Creates a new `ChatFullInfoPrivate`.
    ///
    /// # Arguments
    /// * `id` - Unique identifier for this chat. This number may have more than 32 significant bits and some programming languages may have difficulty/silent defects in interpreting it. But it has at most 52 significant bits, so a signed 64-bit integer or double-precision float type are safe for storing this identifier.
    /// * `accent_color_id` - Identifier of the accent color for the chat name and backgrounds of the chat photo, reply header, and link preview. See accent colors for more details.
    /// * `max_reaction_count` - The maximum number of reactions that can be set on a message in the chat
    /// * `accepted_gift_types` - Information about types of gifts that are accepted by the chat or by the corresponding user for private chats
    ///
    /// # Notes
    /// Use builder methods to set optional fields.
    #[must_use]
    pub fn new<
        T0: Into<i64>,
        T1: Into<i64>,
        T2: Into<i64>,
        T3: Into<crate::types::AcceptedGiftTypes>,
    >(
        id: T0,
        accent_color_id: T1,
        max_reaction_count: T2,
        accepted_gift_types: T3,
    ) -> Self {
        Self {
            id: id.into(),
            username: None,
            first_name: None,
            last_name: None,
            is_direct_messages: None,
            accent_color_id: accent_color_id.into(),
            max_reaction_count: max_reaction_count.into(),
            photo: None,
            active_usernames: None,
            birthdate: None,
            business_intro: None,
            business_location: None,
            business_opening_hours: None,
            personal_chat: None,
            available_reactions: None,
            background_custom_emoji_id: None,
            profile_accent_color_id: None,
            profile_background_custom_emoji_id: None,
            emoji_status_custom_emoji_id: None,
            emoji_status_expiration_date: None,
            bio: None,
            has_private_forwards: None,
            has_restricted_voice_and_video_messages: None,
            pinned_message: None,
            accepted_gift_types: accepted_gift_types.into(),
            message_auto_delete_time: None,
            has_hidden_members: None,
            has_protected_content: None,
            has_visible_history: None,
            rating: None,
            first_profile_audio: None,
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

    /// Username, for private chats, supergroups and channels if available
    #[must_use]
    pub fn username<T: Into<Box<str>>>(self, val: T) -> Self {
        let mut this = self;
        this.username = Some(val.into());
        this
    }

    /// Username, for private chats, supergroups and channels if available
    #[must_use]
    pub fn username_option<T: Into<Box<str>>>(self, val: Option<T>) -> Self {
        let mut this = self;
        this.username = val.map(Into::into);
        this
    }

    /// First name of the other party in a private chat
    #[must_use]
    pub fn first_name<T: Into<Box<str>>>(self, val: T) -> Self {
        let mut this = self;
        this.first_name = Some(val.into());
        this
    }

    /// First name of the other party in a private chat
    #[must_use]
    pub fn first_name_option<T: Into<Box<str>>>(self, val: Option<T>) -> Self {
        let mut this = self;
        this.first_name = val.map(Into::into);
        this
    }

    /// Last name of the other party in a private chat
    #[must_use]
    pub fn last_name<T: Into<Box<str>>>(self, val: T) -> Self {
        let mut this = self;
        this.last_name = Some(val.into());
        this
    }

    /// Last name of the other party in a private chat
    #[must_use]
    pub fn last_name_option<T: Into<Box<str>>>(self, val: Option<T>) -> Self {
        let mut this = self;
        this.last_name = val.map(Into::into);
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

    /// If non-empty, the list of all active chat usernames; for private chats, supergroups and channels
    ///
    /// # Notes
    /// Adds multiple elements.
    #[must_use]
    pub fn active_usernames<T: Into<Box<[Box<str>]>>>(self, val: T) -> Self {
        let mut this = self;
        this.active_usernames = Some(
            this.active_usernames
                .unwrap_or_default()
                .into_vec()
                .into_iter()
                .chain(val.into())
                .collect(),
        );
        this
    }

    /// If non-empty, the list of all active chat usernames; for private chats, supergroups and channels
    ///
    /// # Notes
    /// Adds a single element.
    #[must_use]
    pub fn active_username<T: Into<Box<str>>>(self, val: T) -> Self {
        let mut this = self;
        this.active_usernames = Some(
            this.active_usernames
                .unwrap_or_default()
                .into_vec()
                .into_iter()
                .chain(Some(val.into()))
                .collect(),
        );
        this
    }

    /// If non-empty, the list of all active chat usernames; for private chats, supergroups and channels
    ///
    /// # Notes
    /// Adds a single element.
    #[must_use]
    pub fn active_usernames_option<T: Into<Box<[Box<str>]>>>(self, val: Option<T>) -> Self {
        let mut this = self;
        this.active_usernames = val.map(Into::into);
        this
    }

    /// For private chats, the date of birth of the user
    #[must_use]
    pub fn birthdate<T: Into<crate::types::Birthdate>>(self, val: T) -> Self {
        let mut this = self;
        this.birthdate = Some(val.into());
        this
    }

    /// For private chats, the date of birth of the user
    #[must_use]
    pub fn birthdate_option<T: Into<crate::types::Birthdate>>(self, val: Option<T>) -> Self {
        let mut this = self;
        this.birthdate = val.map(Into::into);
        this
    }

    /// For private chats with business accounts, the intro of the business
    #[must_use]
    pub fn business_intro<T: Into<crate::types::BusinessIntro>>(self, val: T) -> Self {
        let mut this = self;
        this.business_intro = Some(val.into());
        this
    }

    /// For private chats with business accounts, the intro of the business
    #[must_use]
    pub fn business_intro_option<T: Into<crate::types::BusinessIntro>>(
        self,
        val: Option<T>,
    ) -> Self {
        let mut this = self;
        this.business_intro = val.map(Into::into);
        this
    }

    /// For private chats with business accounts, the location of the business
    #[must_use]
    pub fn business_location<T: Into<crate::types::BusinessLocation>>(self, val: T) -> Self {
        let mut this = self;
        this.business_location = Some(val.into());
        this
    }

    /// For private chats with business accounts, the location of the business
    #[must_use]
    pub fn business_location_option<T: Into<crate::types::BusinessLocation>>(
        self,
        val: Option<T>,
    ) -> Self {
        let mut this = self;
        this.business_location = val.map(Into::into);
        this
    }

    /// For private chats with business accounts, the opening hours of the business
    #[must_use]
    pub fn business_opening_hours<T: Into<crate::types::BusinessOpeningHours>>(
        self,
        val: T,
    ) -> Self {
        let mut this = self;
        this.business_opening_hours = Some(val.into());
        this
    }

    /// For private chats with business accounts, the opening hours of the business
    #[must_use]
    pub fn business_opening_hours_option<T: Into<crate::types::BusinessOpeningHours>>(
        self,
        val: Option<T>,
    ) -> Self {
        let mut this = self;
        this.business_opening_hours = val.map(Into::into);
        this
    }

    /// For private chats, the personal channel of the user
    #[must_use]
    pub fn personal_chat<T: Into<crate::types::Chat>>(self, val: T) -> Self {
        let mut this = self;
        this.personal_chat = Some(Box::new(val.into()));
        this
    }

    /// For private chats, the personal channel of the user
    #[must_use]
    pub fn personal_chat_option<T: Into<crate::types::Chat>>(self, val: Option<T>) -> Self {
        let mut this = self;
        this.personal_chat = val.map(|val| Box::new(val.into()));
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

    /// Custom emoji identifier of the emoji status of the chat or the other party in a private chat
    #[must_use]
    pub fn emoji_status_custom_emoji_id<T: Into<Box<str>>>(self, val: T) -> Self {
        let mut this = self;
        this.emoji_status_custom_emoji_id = Some(val.into());
        this
    }

    /// Custom emoji identifier of the emoji status of the chat or the other party in a private chat
    #[must_use]
    pub fn emoji_status_custom_emoji_id_option<T: Into<Box<str>>>(self, val: Option<T>) -> Self {
        let mut this = self;
        this.emoji_status_custom_emoji_id = val.map(Into::into);
        this
    }

    /// Expiration date of the emoji status of the chat or the other party in a private chat, in Unix time, if any
    #[must_use]
    pub fn emoji_status_expiration_date<T: Into<i64>>(self, val: T) -> Self {
        let mut this = self;
        this.emoji_status_expiration_date = Some(val.into());
        this
    }

    /// Expiration date of the emoji status of the chat or the other party in a private chat, in Unix time, if any
    #[must_use]
    pub fn emoji_status_expiration_date_option<T: Into<i64>>(self, val: Option<T>) -> Self {
        let mut this = self;
        this.emoji_status_expiration_date = val.map(Into::into);
        this
    }

    /// Bio of the other party in a private chat
    #[must_use]
    pub fn bio<T: Into<Box<str>>>(self, val: T) -> Self {
        let mut this = self;
        this.bio = Some(val.into());
        this
    }

    /// Bio of the other party in a private chat
    #[must_use]
    pub fn bio_option<T: Into<Box<str>>>(self, val: Option<T>) -> Self {
        let mut this = self;
        this.bio = val.map(Into::into);
        this
    }

    /// `true`, if privacy settings of the other party in the private chat allows to use ``tg://user?id=<user_id>`` links only in chats with the user
    #[must_use]
    pub fn has_private_forwards<T: Into<bool>>(self, val: T) -> Self {
        let mut this = self;
        this.has_private_forwards = Some(val.into());
        this
    }

    /// `true`, if privacy settings of the other party in the private chat allows to use ``tg://user?id=<user_id>`` links only in chats with the user
    #[must_use]
    pub fn has_private_forwards_option<T: Into<bool>>(self, val: Option<T>) -> Self {
        let mut this = self;
        this.has_private_forwards = val.map(Into::into);
        this
    }

    /// `true`, if the privacy settings of the other party restrict sending voice and video note messages in the private chat
    #[must_use]
    pub fn has_restricted_voice_and_video_messages<T: Into<bool>>(self, val: T) -> Self {
        let mut this = self;
        this.has_restricted_voice_and_video_messages = Some(val.into());
        this
    }

    /// `true`, if the privacy settings of the other party restrict sending voice and video note messages in the private chat
    #[must_use]
    pub fn has_restricted_voice_and_video_messages_option<T: Into<bool>>(
        self,
        val: Option<T>,
    ) -> Self {
        let mut this = self;
        this.has_restricted_voice_and_video_messages = val.map(Into::into);
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

    /// Information about types of gifts that are accepted by the chat or by the corresponding user for private chats
    #[must_use]
    pub fn accepted_gift_types<T: Into<crate::types::AcceptedGiftTypes>>(self, val: T) -> Self {
        let mut this = self;
        this.accepted_gift_types = val.into();
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

    /// For private chats, the rating of the user if any
    #[must_use]
    pub fn rating<T: Into<crate::types::UserRating>>(self, val: T) -> Self {
        let mut this = self;
        this.rating = Some(val.into());
        this
    }

    /// For private chats, the rating of the user if any
    #[must_use]
    pub fn rating_option<T: Into<crate::types::UserRating>>(self, val: Option<T>) -> Self {
        let mut this = self;
        this.rating = val.map(Into::into);
        this
    }

    /// For private chats, the first audio added to the profile of the user
    #[must_use]
    pub fn first_profile_audio<T: Into<crate::types::Audio>>(self, val: T) -> Self {
        let mut this = self;
        this.first_profile_audio = Some(Box::new(val.into()));
        this
    }

    /// For private chats, the first audio added to the profile of the user
    #[must_use]
    pub fn first_profile_audio_option<T: Into<crate::types::Audio>>(self, val: Option<T>) -> Self {
        let mut this = self;
        this.first_profile_audio = val.map(|val| Box::new(val.into()));
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
